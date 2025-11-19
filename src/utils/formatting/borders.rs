use crate::style::TableComponent;
use crate::table::Table;
use crate::utils::ColumnDisplayInfo;
use crate::utils::spanning::SpanTracker;

pub(crate) fn draw_borders(
    table: &Table,
    rows: &[Vec<Vec<String>>],
    display_info: &[ColumnDisplayInfo],
) -> Vec<String> {
    // We know how many lines there should be. Initialize the vector with the rough correct amount.
    // We might over allocate a bit, but that's better than under allocating.
    let mut lines = if let Some(capacity) = rows.first().map(|lines| lines.len()) {
        // Lines * 2 -> Lines + delimiters
        // + 5 -> header delimiters + header + bottom/top borders
        Vec::with_capacity(capacity * 2 + 5)
    } else {
        Vec::new()
    };

    // Build span information for border drawing
    let mut span_tracker = SpanTracker::new();
    let header_rows = if table.header.is_some() { 1 } else { 0 };

    if should_draw_top_border(table) {
        lines.push(draw_top_border(table, display_info));
    }

    draw_rows(
        &mut lines,
        rows,
        table,
        display_info,
        &mut span_tracker,
        header_rows,
    );

    if should_draw_bottom_border(table) {
        // Get the last row's first line to detect colspan for bottom border
        let last_row_line = rows
            .last()
            .and_then(|row| row.first().map(|line| line.as_slice()));
        lines.push(draw_bottom_border(table, display_info, last_row_line));
    }

    lines
}

fn draw_top_border(table: &Table, display_info: &[ColumnDisplayInfo]) -> String {
    let left_corner = table.style_or_default(TableComponent::TopLeftCorner);
    let top_border = table.style_or_default(TableComponent::TopBorder);
    let intersection = table.style_or_default(TableComponent::TopBorderIntersections);
    let right_corner = table.style_or_default(TableComponent::TopRightCorner);

    let mut line = String::new();
    // We only need the top left corner, if we need to draw a left border
    if should_draw_left_border(table) {
        line += &left_corner;
    }

    // Build the top border line depending on the columns' width.
    // Also add the border intersections.
    // Top border always shows physical columns, not logical structure
    let mut first = true;
    for info in display_info.iter() {
        // Only add something, if the column isn't hidden
        if !info.is_hidden {
            if !first {
                line += &intersection;
            }
            line += &top_border.repeat(info.width().into());
            first = false;
        }
    }

    // We only need the top right corner, if we need to draw a right border
    if should_draw_right_border(table) {
        line += &right_corner;
    }

    line
}

fn draw_rows(
    lines: &mut Vec<String>,
    rows: &[Vec<Vec<String>>],
    table: &Table,
    display_info: &[ColumnDisplayInfo],
    span_tracker: &mut SpanTracker,
    header_rows: usize,
) {
    // Iterate over all rows
    let mut row_iter = rows.iter().enumerate().peekable();
    while let Some((row_index, row)) = row_iter.next() {
        let actual_row_index = if row_index < header_rows {
            row_index
        } else {
            row_index - header_rows
        };

        // Concatenate the line parts and insert the vertical borders if needed
        for line_parts in row.iter() {
            lines.push(embed_line(
                line_parts,
                table,
                actual_row_index,
                span_tracker,
            ));
        }

        // Draw the horizontal header line if desired, otherwise continue to the next iteration
        if row_index == 0 && table.header.is_some() {
            if should_draw_header(table) {
                // Header separator should match the header content width (widest line)
                // Draw all physical columns separately (like top border)
                lines.push(draw_horizontal_lines(
                    table,
                    display_info,
                    true,
                    0,
                    span_tracker,
                    row.first().map(|line| line.as_slice()).unwrap_or(&[]),
                ));
            }
            // Register rowspans from header for border drawing (we only need position info, not content)
            if let Some(header) = &table.header {
                let mut col_index = 0;
                for cell in &header.cells {
                    if cell.rowspan() > 1 {
                        span_tracker.register_rowspan(
                            0,
                            col_index,
                            cell.rowspan(),
                            cell.colspan(),
                            None,
                        );
                    }
                    col_index += cell.colspan() as usize;
                }
            }
            span_tracker.advance_row(1);
            continue;
        }

        // Register rowspans from data rows for border drawing
        if actual_row_index < table.rows.len() {
            let data_row = &table.rows[actual_row_index];
            let mut col_index = 0;
            for cell in &data_row.cells {
                // Skip positions occupied by rowspan
                while col_index < display_info.len()
                    && span_tracker
                        .is_col_occupied_by_rowspan(actual_row_index + header_rows, col_index)
                {
                    col_index += 1;
                }
                if col_index >= display_info.len() {
                    break;
                }
                if cell.rowspan() > 1 {
                    span_tracker.register_rowspan(
                        actual_row_index + header_rows,
                        col_index,
                        cell.rowspan(),
                        cell.colspan(),
                        None,
                    );
                }
                col_index += cell.colspan() as usize;
            }
        }

        // Draw a horizontal line, if we desired and if we aren't in the last row of the table.
        // When drawing the border after a row, we need to check for rowspans that continue into the next row.
        // So we check at the current row_index (the row we just processed).
        if row_iter.peek().is_some() && should_draw_horizontal_lines(table) {
            // Draw all physical columns separately (like top border), not based on row structure
            let border_line = row.first().map(|line| line.as_slice()).unwrap_or(&[]);
            // Check for rowspans at the current row_index (row we just processed)
            // Rowspans that started at this row or earlier and still have remaining_rows should skip borders
            lines.push(draw_horizontal_lines(
                table,
                display_info,
                false,
                actual_row_index + header_rows,
                span_tracker,
                border_line,
            ));
        }

        span_tracker.advance_row(actual_row_index + header_rows + 1);
    }
}

// Takes the parts of a single line, surrounds them with borders and adds vertical lines.
// Skips vertical borders within colspan cells (detected by empty strings).
fn embed_line(
    line_parts: &[String],
    table: &Table,
    _row_index: usize,
    _span_tracker: &SpanTracker,
) -> String {
    let vertical_lines = table.style_or_default(TableComponent::VerticalLines);
    let left_border = table.style_or_default(TableComponent::LeftBorder);
    let right_border = table.style_or_default(TableComponent::RightBorder);

    let mut line = String::new();
    if should_draw_left_border(table) {
        line += &left_border;
    }

    let mut part_iter = line_parts.iter().peekable();
    while let Some(part) = part_iter.next() {
        line += part;
        // Check if the next part exists and is not empty (empty string indicates colspan)
        let next_part = part_iter.peek();
        if let Some(next) = next_part {
            // If next part is empty, it's part of a colspan - skip vertical border
            if next.is_empty() {
                // Skip the border for colspan
            } else if should_draw_vertical_lines(table) {
                line += &vertical_lines;
            }
        } else if should_draw_right_border(table) {
            line += &right_border;
        }
    }

    line
}

// The horizontal line that separates between rows.
// Skips horizontal lines within rowspan cells.
// Makes borders continuous for colspan cells.
fn draw_horizontal_lines(
    table: &Table,
    display_info: &[ColumnDisplayInfo],
    header: bool,
    row_index: usize,
    span_tracker: &SpanTracker,
    row_line: &[String],
) -> String {
    // Styling depends on whether we're currently on the header line or not.
    let (left_intersection, horizontal_lines, middle_intersection, right_intersection) = if header {
        (
            table.style_or_default(TableComponent::LeftHeaderIntersection),
            table.style_or_default(TableComponent::HeaderLines),
            table.style_or_default(TableComponent::MiddleHeaderIntersections),
            table.style_or_default(TableComponent::RightHeaderIntersection),
        )
    } else {
        (
            table.style_or_default(TableComponent::LeftBorderIntersections),
            table.style_or_default(TableComponent::HorizontalLines),
            table.style_or_default(TableComponent::MiddleIntersections),
            table.style_or_default(TableComponent::RightBorderIntersections),
        )
    };

    let mut line = String::new();
    // We only need the bottom left corner, if we need to draw a left border
    if should_draw_left_border(table) {
        line += &left_intersection;
    }

    // Draw borders following the logical structure of the preceding row.
    // Use row_line to detect colspan: empty strings indicate colspan continuation.
    // row_line parts correspond to visible logical columns only.
    let mut first = true;
    let mut visible_col_index = 0; // Index into visible columns (matches row_line index)

    // Iterate through physical columns
    let mut col_index = 0;
    while col_index < display_info.len() {
        let info = &display_info[col_index];

        // Skip hidden columns
        if info.is_hidden {
            col_index += 1;
            continue;
        }

        // Check if this column is part of a rowspan that continues into the next row
        if let Some((_start_row, start_col, rowspan_colspan)) =
            span_tracker.get_rowspan_start_at_row(row_index, col_index)
        {
            // This column is part of a rowspan, skip ALL columns in the rowspan's colspan range
            let mut rowspan_width = 0;
            let mut visible_cols_in_rowspan: usize = 0;
            for i in start_col..start_col + rowspan_colspan as usize {
                if i < display_info.len() && !display_info[i].is_hidden {
                    rowspan_width += display_info[i].width() as usize;
                    visible_cols_in_rowspan += 1;
                }
            }
            // Add 1 character per missing separator (visible_cols_in_rowspan - 1 separators would be missing)
            rowspan_width += visible_cols_in_rowspan.saturating_sub(1);
            line += &" ".repeat(rowspan_width);
            col_index = start_col + rowspan_colspan as usize;
            // Advance visible_col_index past the rowspan columns
            let visible_cols_skipped = display_info[start_col..col_index]
                .iter()
                .filter(|info| !info.is_hidden)
                .count();
            visible_col_index += visible_cols_skipped;
            first = false;
            continue;
        }

        // Check if we have a corresponding row_line part
        if visible_col_index < row_line.len() {
            let part = &row_line[visible_col_index];

            if part.is_empty() {
                // Empty part indicates colspan continuation - no separator, border continues
                line += &horizontal_lines.repeat(info.width() as usize);
                visible_col_index += 1;
                col_index += 1;
                continue;
            } else {
                // Non-empty part - this is a logical cell (possibly colspan)
                // Calculate how many visible columns this cell spans by counting following empty parts
                let mut colspan_visible_count = 1;
                let mut lookahead = visible_col_index + 1;
                while lookahead < row_line.len() && row_line[lookahead].is_empty() {
                    colspan_visible_count += 1;
                    lookahead += 1;
                }

                // Calculate total width for this colspan cell by summing widths of spanned columns
                // Add 1 character per span (colspan - 1) to account for missing separator characters
                let mut colspan_width = 0;
                let mut temp_col = col_index;
                let mut cols_counted = 0;
                while cols_counted < colspan_visible_count && temp_col < display_info.len() {
                    if !display_info[temp_col].is_hidden {
                        colspan_width += display_info[temp_col].width() as usize;
                        cols_counted += 1;
                    }
                    if cols_counted < colspan_visible_count {
                        temp_col += 1;
                    } else {
                        break;
                    }
                }
                // Add 1 character per missing separator (colspan - 1 separators would be missing)
                colspan_width += colspan_visible_count - 1;

                // Add separator before this logical cell (if not first)
                if !first {
                    line += &middle_intersection;
                }
                // Draw continuous border for the entire colspan
                line += &horizontal_lines.repeat(colspan_width);
                first = false;

                // Advance past all columns in this colspan
                visible_col_index += colspan_visible_count;
                // Advance physical column index past the colspan
                let mut visible_advanced = 0;
                while visible_advanced < colspan_visible_count && col_index < display_info.len() {
                    if !display_info[col_index].is_hidden {
                        visible_advanced += 1;
                    }
                    if visible_advanced < colspan_visible_count {
                        col_index += 1;
                    } else {
                        col_index += 1;
                        break;
                    }
                }
                continue;
            }
        } else {
            // No more row_line parts, but we still have physical columns
            // This shouldn't happen normally, but handle it gracefully
            if !first {
                line += &middle_intersection;
            }
            line += &horizontal_lines.repeat(info.width() as usize);
            first = false;
            col_index += 1;
            visible_col_index += 1;
        }
    }

    // We only need the bottom right corner, if we need to draw a right border
    if should_draw_right_border(table) {
        line += &right_intersection;
    }

    line
}

fn draw_bottom_border(
    table: &Table,
    display_info: &[ColumnDisplayInfo],
    _last_row_line: Option<&[String]>,
) -> String {
    let left_corner = table.style_or_default(TableComponent::BottomLeftCorner);
    let bottom_border = table.style_or_default(TableComponent::BottomBorder);
    let intersection = table.style_or_default(TableComponent::BottomBorderIntersections);
    let right_corner = table.style_or_default(TableComponent::BottomRightCorner);

    let mut line = String::new();
    // We only need the bottom left corner, if we need to draw a left border
    if should_draw_left_border(table) {
        line += &left_corner;
    }

    // Build the bottom border line depending on the columns' width.
    // Also add the border intersections.
    // Bottom border always shows physical columns, matching the top border exactly
    let mut first = true;
    for info in display_info.iter() {
        // Only add something, if the column isn't hidden
        if !info.is_hidden {
            if !first {
                line += &intersection;
            }
            line += &bottom_border.repeat(info.width().into());
            first = false;
        }
    }

    // We only need the bottom right corner, if we need to draw a right border
    if should_draw_right_border(table) {
        line += &right_corner;
    }

    line
}

fn should_draw_top_border(table: &Table) -> bool {
    if table.style_exists(TableComponent::TopLeftCorner)
        || table.style_exists(TableComponent::TopBorder)
        || table.style_exists(TableComponent::TopBorderIntersections)
        || table.style_exists(TableComponent::TopRightCorner)
    {
        return true;
    }

    false
}

fn should_draw_bottom_border(table: &Table) -> bool {
    if table.style_exists(TableComponent::BottomLeftCorner)
        || table.style_exists(TableComponent::BottomBorder)
        || table.style_exists(TableComponent::BottomBorderIntersections)
        || table.style_exists(TableComponent::BottomRightCorner)
    {
        return true;
    }

    false
}

pub fn should_draw_left_border(table: &Table) -> bool {
    if table.style_exists(TableComponent::TopLeftCorner)
        || table.style_exists(TableComponent::LeftBorder)
        || table.style_exists(TableComponent::LeftBorderIntersections)
        || table.style_exists(TableComponent::LeftHeaderIntersection)
        || table.style_exists(TableComponent::BottomLeftCorner)
    {
        return true;
    }

    false
}

pub fn should_draw_right_border(table: &Table) -> bool {
    if table.style_exists(TableComponent::TopRightCorner)
        || table.style_exists(TableComponent::RightBorder)
        || table.style_exists(TableComponent::RightBorderIntersections)
        || table.style_exists(TableComponent::RightHeaderIntersection)
        || table.style_exists(TableComponent::BottomRightCorner)
    {
        return true;
    }

    false
}

fn should_draw_horizontal_lines(table: &Table) -> bool {
    if table.style_exists(TableComponent::LeftBorderIntersections)
        || table.style_exists(TableComponent::HorizontalLines)
        || table.style_exists(TableComponent::MiddleIntersections)
        || table.style_exists(TableComponent::RightBorderIntersections)
    {
        return true;
    }

    false
}

pub fn should_draw_vertical_lines(table: &Table) -> bool {
    if table.style_exists(TableComponent::TopBorderIntersections)
        || table.style_exists(TableComponent::MiddleHeaderIntersections)
        || table.style_exists(TableComponent::VerticalLines)
        || table.style_exists(TableComponent::MiddleIntersections)
        || table.style_exists(TableComponent::BottomBorderIntersections)
    {
        return true;
    }

    false
}

fn should_draw_header(table: &Table) -> bool {
    if table.style_exists(TableComponent::LeftHeaderIntersection)
        || table.style_exists(TableComponent::HeaderLines)
        || table.style_exists(TableComponent::MiddleHeaderIntersections)
        || table.style_exists(TableComponent::RightHeaderIntersection)
    {
        return true;
    }

    false
}
