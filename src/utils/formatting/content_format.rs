#[cfg(feature = "tty")]
use crossterm::style::{Stylize, style};
use unicode_segmentation::UnicodeSegmentation;
use unicode_width::UnicodeWidthStr;

use super::content_split::measure_text_width;
use super::content_split::split_line;

use crate::cell::Cell;
use crate::row::Row;
use crate::style::CellAlignment;
#[cfg(feature = "tty")]
use crate::style::{map_attribute, map_color};
use crate::table::Table;
use crate::utils::ColumnDisplayInfo;
use crate::utils::spanning::SpanTracker;

pub fn delimiter(cell: &Cell, info: &ColumnDisplayInfo, table: &Table) -> char {
    // Determine, which delimiter should be used
    if let Some(delimiter) = cell.delimiter {
        delimiter
    } else if let Some(delimiter) = info.delimiter {
        delimiter
    } else if let Some(delimiter) = table.delimiter {
        delimiter
    } else {
        ' '
    }
}

/// Returns the formatted content of the table.
/// The content is organized in the following structure
///
/// tc stands for table content and represents the returned value
/// ``` text
///      column1          column2
/// row1 tc[0][0][0]      tc[0][0][1] <-line1
///      tc[0][1][0]      tc[0][1][1] <-line2
///      tc[0][2][0]      tc[0][2][1] <-line3
///
/// row2 tc[1][0][0]      tc[1][0][1] <-line1
///      tc[1][1][0]      tc[1][1][1] <-line2
///      tc[1][2][0]      tc[1][2][1] <-line3
/// ```
///
/// The strings for each row will be padded and aligned according to their respective column.
pub fn format_content(table: &Table, display_info: &[ColumnDisplayInfo]) -> Vec<Vec<Vec<String>>> {
    // The content of the whole table
    let mut table_content = Vec::with_capacity(table.rows.len() + 1);
    let mut span_tracker = SpanTracker::new();

    // Format table header if it exists
    if let Some(header) = table.header() {
        table_content.push(format_row(
            header,
            display_info,
            table,
            0,
            &mut span_tracker,
        ));
        span_tracker.advance_row(1);
    }

    for (row_index, row) in table.rows.iter().enumerate() {
        let actual_row_index = if table.header.is_some() {
            row_index + 1
        } else {
            row_index
        };
        table_content.push(format_row(
            row,
            display_info,
            table,
            actual_row_index,
            &mut span_tracker,
        ));
        // Advance row AFTER processing, so rowspan content is available for the next row
        span_tracker.advance_row(actual_row_index + 1);
    }
    table_content
}

pub(crate) fn format_row(
    row: &Row,
    display_infos: &[ColumnDisplayInfo],
    table: &Table,
    row_index: usize,
    span_tracker: &mut SpanTracker,
) -> Vec<Vec<String>> {
    // The content of this specific row
    // We'll build a vector where each element represents a column position
    // For colspan cells, we'll store the formatted content once and mark the spanned positions
    let mut temp_row_content: Vec<Option<Vec<String>>> = vec![None; display_infos.len()];
    // Track which columns are part of a colspan (maps col_index -> colspan)
    let mut colspan_map: Vec<Option<usize>> = vec![None; display_infos.len()];
    let mut col_index = 0;

    // Process each cell in the row
    for cell in &row.cells {
        // Skip column positions that are occupied by rowspan from above
        while col_index < display_infos.len()
            && span_tracker.is_col_occupied_by_rowspan(row_index, col_index)
        {
            // This position is occupied by a rowspan, mark it as such
            temp_row_content[col_index] = Some(vec!["".to_string()]);
            col_index += 1;
        }

        if col_index >= display_infos.len() {
            break;
        }

        let colspan = cell.colspan() as usize;
        let rowspan = cell.rowspan();

        // Get the spanned column infos
        let spanned_infos: Vec<&ColumnDisplayInfo> = display_infos
            .iter()
            .skip(col_index)
            .take(colspan)
            .filter(|info| !info.is_hidden)
            .collect();

        if spanned_infos.is_empty() {
            col_index += colspan;
            continue;
        }

        // Calculate combined width for colspan cells
        // Sum the content widths, PLUS add 3 chars per span for the missing borders " | " between columns
        // If there were 2 separate cells, they'd have " | " (3 chars) between them
        // Use the number of visible columns, not the logical colspan (hidden columns don't need border compensation)
        let visible_colspan_count = spanned_infos.len();
        let borders_between = (visible_colspan_count.saturating_sub(1)) as u16 * 3;
        let combined_content_width: u16 = spanned_infos
            .iter()
            .map(|info| info.content_width)
            .sum::<u16>()
            + borders_between;
        let combined_padding_left: u16 = spanned_infos
            .first()
            .map(|info| info.padding.0)
            .unwrap_or(0);
        let combined_padding_right: u16 =
            spanned_infos.last().map(|info| info.padding.1).unwrap_or(0);

        // Create a temporary ColumnDisplayInfo for the spanned cell
        let spanned_info = ColumnDisplayInfo {
            padding: (combined_padding_left, combined_padding_right),
            delimiter: spanned_infos[0].delimiter,
            content_width: combined_content_width,
            cell_alignment: cell.alignment.or(spanned_infos[0].cell_alignment),
            is_hidden: false,
        };

        // Format the cell content
        let mut cell_lines = Vec::new();
        let cell_delimiter = delimiter(cell, &spanned_info, table);

        // Iterate over each line and split it into multiple lines if necessary.
        // Newlines added by the user will be preserved.
        for line in cell.content.iter() {
            if measure_text_width(line) > combined_content_width.into() {
                let mut parts = split_line(line, &spanned_info, cell_delimiter);
                cell_lines.append(&mut parts);
            } else {
                cell_lines.push(line.into());
            }
        }

        // Remove all unneeded lines of this cell, if the row's height is capped to a certain
        // amount of lines and there're too many lines in this cell.
        // This then truncates and inserts a '...' string at the end of the last line to indicate
        // that the cell has been truncated.
        if let Some(lines) = row.max_height {
            if cell_lines.len() > lines {
                // We already have to many lines. Cut off the surplus lines.
                let _ = cell_lines.split_off(lines);

                // Directly access the last line.
                let last_line = cell_lines
                    .get_mut(lines - 1)
                    .expect("We know it's this long.");

                // Truncate any ansi codes, as the following cutoff might break ansi code
                // otherwise anyway. This could be handled smarter, but it's simple and just works.
                #[cfg(feature = "custom_styling")]
                {
                    let stripped = console::strip_ansi_codes(last_line).to_string();
                    *last_line = stripped;
                }

                let max_width: usize = combined_content_width.into();
                let indicator_width = table.truncation_indicator.width();

                let mut truncate_at = 0;
                // Start the accumulated_width with the indicator_width, which is the minimum width
                // we may show anyway.
                let mut accumulated_width = indicator_width;
                let mut full_string_fits = false;

                // Leave these print statements in here in case we ever have to debug this annoying
                // stuff again.
                //println!("\nSTART:");
                //println!("\nMax width: {max_width}, Indicator width: {indicator_width}");
                //println!("Full line hex: {last_line}");
                //println!(
                //    "Full line hex: {}",
                //    last_line
                //        .as_bytes()
                //        .iter()
                //        .map(|byte| format!("{byte:02x}"))
                //        .collect::<Vec<String>>()
                //        .join(", ")
                //);

                // Iterate through the UTF-8 graphemes.
                // Check the `split_long_word` inline function docs to see why we're using
                // graphemes.
                // **Note:** The `index` here is the **byte** index. So we cannot just
                //    String::truncate afterwards. We have to convert to a byte vector to perform
                //    the truncation first.
                let mut grapheme_iter = last_line.grapheme_indices(true).peekable();
                while let Some((index, grapheme)) = grapheme_iter.next() {
                    // Leave these print statements in here in case we ever have to debug this
                    // annoying stuff again
                    //println!(
                    //    "Current index: {index}, Next grapheme: {grapheme} (width: {})",
                    //    grapheme.width()
                    //);
                    //println!(
                    //    "Next grapheme hex: {}",
                    //    grapheme
                    //        .as_bytes()
                    //        .iter()
                    //        .map(|byte| format!("{byte:02x}"))
                    //        .collect::<Vec<String>>()
                    //        .join(", ")
                    //);

                    // Immediately save where to truncate in case this grapheme doesn't fit.
                    // The index is just before the current grapheme actually starts.
                    truncate_at = index;
                    // Check if the next grapheme would break the boundary of the allowed line
                    // length.
                    let new_width = accumulated_width + grapheme.width();
                    //println!(
                    //    "Next width: {new_width}/{max_width} ({accumulated_width} + {})",
                    //    grapheme.width()
                    //);
                    if new_width > max_width {
                        //println!(
                        //    "Breaking: {:?}",
                        //    accumulated_width + grapheme.width() > max_width
                        //);
                        break;
                    }

                    // The grapheme seems to fit. Save the index and check the next one.
                    accumulated_width += grapheme.width();

                    // This is a special case.
                    // We reached the last char, meaning that full last line + the indicator fit.
                    if grapheme_iter.peek().is_none() {
                        full_string_fits = true
                    }
                }

                // Only do any truncation logic if the line doesn't fit.
                if !full_string_fits {
                    // Truncate the string at the byte index just behind the last valid grapheme
                    // and overwrite the last line with the new truncated string.
                    let mut last_line_bytes = last_line.clone().into_bytes();
                    last_line_bytes.truncate(truncate_at);
                    let new_last_line = String::from_utf8(last_line_bytes)
                        .expect("We cut at an exact char boundary");
                    *last_line = new_last_line;
                }

                // Push the truncation indicator.
                last_line.push_str(&table.truncation_indicator);
            }
        }

        // Iterate over all generated lines of this cell and align them
        let aligned_cell_lines: Vec<String> = cell_lines
            .iter()
            .map(|line| align_line(table, &spanned_info, cell, line.to_string()))
            .collect();

        // Store the formatted cell content in the first column position
        // For colspan > 1, mark all spanned columns
        // Clone for rowspan registration if needed
        let content_for_storage = aligned_cell_lines.clone();
        temp_row_content[col_index] = Some(aligned_cell_lines);
        for i in 0..colspan {
            if col_index + i < colspan_map.len() {
                if i == 0 {
                    colspan_map[col_index + i] = Some(colspan);
                } else {
                    colspan_map[col_index + i] = Some(0); // Mark as spanned (0 means part of colspan)
                }
            }
        }

        // Register rowspan if needed, caching the formatted content
        if rowspan > 1 {
            span_tracker.register_rowspan(
                row_index,
                col_index,
                rowspan,
                colspan as u16,
                Some(content_for_storage),
            );
        }

        // Advance column index by colspan
        col_index += colspan;
    }

    // Fill in any remaining positions that weren't covered (shouldn't happen in valid tables)
    for i in col_index..display_infos.len() {
        if temp_row_content[i].is_none() && !span_tracker.is_col_occupied_by_rowspan(row_index, i) {
            if display_infos[i].is_hidden {
                continue;
            }
            temp_row_content[i] = Some(vec![" ".repeat(display_infos[i].width().into())]);
        }
    }

    // Now convert from column-based to line-based structure
    // Find the maximum number of lines across all cells
    let max_lines = temp_row_content
        .iter()
        .filter_map(|cell| cell.as_ref().map(|lines| lines.len()))
        .max()
        .unwrap_or(0);

    let mut row_content = Vec::with_capacity(max_lines);

    // Build the row content line by line
    for line_index in 0..max_lines {
        let mut line = Vec::with_capacity(display_infos.len());
        let mut current_col = 0;

        while current_col < display_infos.len() {
            if display_infos[current_col].is_hidden {
                current_col += 1;
                continue;
            }

            // Check if this position is occupied by a rowspan from above
            // Rowspan content should ONLY appear in the FIRST row where it starts
            // Subsequent rows should have empty space where the rowspan is
            if let Some((start_row, start_col, colspan)) =
                span_tracker.get_rowspan_start(row_index, current_col)
            {
                // This is a rowspan position from a previous row
                // Only show content if this IS the starting row, otherwise show empty space
                if start_row == row_index {
                    // This is the starting row, get the cached formatted content
                    if let Some(cached_content) =
                        span_tracker.get_rowspan_content(row_index, start_col)
                    {
                        // Use the cached formatted content
                        if let Some(content) = cached_content.get(line_index) {
                            line.push(content.clone());
                        } else {
                            // Empty line for rowspan - calculate combined width
                            let spanned_infos: Vec<&ColumnDisplayInfo> = display_infos
                                .iter()
                                .skip(start_col)
                                .take(colspan as usize)
                                .filter(|info| !info.is_hidden)
                                .collect();
                            let width_sum: usize =
                                spanned_infos.iter().map(|info| info.width() as usize).sum();
                            let combined_width = width_sum + (colspan as usize - 1); // Add separator compensation
                            line.push(" ".repeat(combined_width));
                        }
                    } else {
                        // Fallback: empty string if content not found
                        line.push("".to_string());
                    }
                } else {
                    // This is NOT the starting row - show empty space
                    let spanned_infos: Vec<&ColumnDisplayInfo> = display_infos
                        .iter()
                        .skip(start_col)
                        .take(colspan as usize)
                        .filter(|info| !info.is_hidden)
                        .collect();
                    let width_sum: usize =
                        spanned_infos.iter().map(|info| info.width() as usize).sum();
                    let combined_width = width_sum + (colspan as usize - 1); // Add separator compensation
                    line.push(" ".repeat(combined_width));
                }
                // Advance by colspan to skip all columns in the rowspan
                current_col += colspan as usize;
                continue;
            }

            // Check if this column has content
            if let Some(cell_lines) = &temp_row_content[current_col] {
                // Check if this cell spans multiple columns
                let colspan = colspan_map[current_col].unwrap_or(1);

                if colspan == 1 {
                    // Normal cell
                    if let Some(content) = cell_lines.get(line_index) {
                        line.push(content.clone());
                    } else {
                        line.push(" ".repeat(display_infos[current_col].width().into()));
                    }
                    current_col += 1;
                } else {
                    // Colspan cell - the content is already formatted to the combined width
                    if let Some(content) = cell_lines.get(line_index) {
                        line.push(content.clone());
                    } else {
                        // Calculate combined width for empty line (only visible columns)
                        let visible_cols: Vec<&ColumnDisplayInfo> = display_infos
                            [current_col..current_col + colspan]
                            .iter()
                            .filter(|info| !info.is_hidden)
                            .collect();
                        let width_sum: usize =
                            visible_cols.iter().map(|info| info.width() as usize).sum();
                        let visible_colspan_count = visible_cols.len();
                        let combined_width = width_sum + (visible_colspan_count.saturating_sub(1)); // Add separator compensation
                        line.push(" ".repeat(combined_width));
                    }
                    // Skip the spanned columns - they're already included in the content above
                    // We need to advance through colspan-1 more logical columns
                    // For visible columns, add empty strings (borders will be drawn correctly)
                    let mut logical_cols_skipped = 0;
                    while logical_cols_skipped < colspan - 1
                        && current_col + 1 < display_infos.len()
                    {
                        current_col += 1;
                        logical_cols_skipped += 1;
                        // Only add empty string for visible columns (hidden ones are skipped by outer loop)
                        if !display_infos[current_col].is_hidden {
                            line.push("".to_string());
                        }
                    }
                    current_col += 1;
                }
            } else {
                // No content for this column, fill with spaces
                line.push(" ".repeat(display_infos[current_col].width().into()));
                current_col += 1;
            }
        }

        row_content.push(line);
    }

    row_content
}

/// Apply the alignment for a column. Alignment can be either Left/Right/Center.
/// In every case all lines will be exactly the same character length `info.width - padding long`
/// This is needed, so we can simply insert it into the border frame later on.
/// Padding is applied in this function as well.
#[allow(unused_variables)]
fn align_line(table: &Table, info: &ColumnDisplayInfo, cell: &Cell, mut line: String) -> String {
    let content_width = info.content_width;
    let remaining: usize = usize::from(content_width).saturating_sub(measure_text_width(&line));

    // Apply the styling before aligning the line, if the user requests it.
    // That way non-delimiter whitespaces won't have stuff like underlines.
    #[cfg(feature = "tty")]
    if table.should_style() && table.style_text_only {
        line = style_line(line, cell);
    }

    // Determine the alignment of the column cells.
    // Cell settings overwrite the columns Alignment settings.
    // Default is Left
    let alignment = if let Some(alignment) = cell.alignment {
        alignment
    } else if let Some(alignment) = info.cell_alignment {
        alignment
    } else {
        CellAlignment::Left
    };

    // Apply left/right/both side padding depending on the alignment of the column
    match alignment {
        CellAlignment::Left => {
            line += &" ".repeat(remaining);
        }
        CellAlignment::Right => {
            line = " ".repeat(remaining) + &line;
        }
        CellAlignment::Center => {
            let left_padding = (remaining as f32 / 2f32).ceil() as usize;
            let right_padding = (remaining as f32 / 2f32).floor() as usize;
            line = " ".repeat(left_padding) + &line + &" ".repeat(right_padding);
        }
    }

    line = pad_line(&line, info);

    #[cfg(feature = "tty")]
    if table.should_style() && !table.style_text_only {
        return style_line(line, cell);
    }

    line
}

/// Apply the column's padding to this line
fn pad_line(line: &str, info: &ColumnDisplayInfo) -> String {
    let mut padded_line = String::new();

    padded_line += &" ".repeat(info.padding.0.into());
    padded_line += line;
    padded_line += &" ".repeat(info.padding.1.into());

    padded_line
}

#[cfg(feature = "tty")]
fn style_line(line: String, cell: &Cell) -> String {
    // Just return the line, if there's no need to style.
    if cell.fg.is_none() && cell.bg.is_none() && cell.attributes.is_empty() {
        return line;
    }

    let mut content = style(line);

    // Apply text color
    if let Some(color) = cell.fg {
        content = content.with(map_color(color));
    }

    // Apply background color
    if let Some(color) = cell.bg {
        content = content.on(map_color(color));
    }

    for attribute in cell.attributes.iter() {
        content = content.attribute(map_attribute(*attribute));
    }

    content.to_string()
}
