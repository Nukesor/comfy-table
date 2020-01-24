use ::crossterm::style::style;

use crate::row::Row;
use crate::cell::Cell;
use crate::style::cell::CellAlignment;
use crate::table::Table;
use crate::utils::arrangement::ColumnDisplayInfo;

/// Returns the formatted content of the table.
/// The content is organized in the following structure
///
/// tc stands for table content and represents the returned value
/// ``` text
///      column1          column2
/// row1 tc[0][0][0]      tc[0][0][1]
///      tc[0][1][0]      tc[0][1][1]
///      tc[0][2][0]      tc[0][2][1]
///
/// row2 tc[1][0][0]      tc[1][0][1]
///      tc[1][1][0]      tc[1][1][1]
///      tc[1][2][0]      tc[1][2][1]
/// ```
///
/// The strings for each row will be padded and aligned accordingly
/// to their respective column.
pub fn format_content(
    table: &Table,
    display_info: &Vec<ColumnDisplayInfo>,
) -> Vec<Vec<Vec<String>>> {
    // The content of the whole table
    let mut table_content = Vec::new();

    // Format table header if it exists
    match table.get_header() {
        Some(header) => {
            table_content.push(format_row(header, display_info));
        }
        None => (),
    }

    for row in table.rows.iter() {
        table_content.push(format_row(row, display_info));
    }
    table_content
}

pub fn format_row(row: &Row, display_info: &Vec<ColumnDisplayInfo>) -> Vec<Vec<String>> {
    // The content of this specific row
    let mut temp_row_content = Vec::new();
    let mut max_content_lines = 0;

    let mut cell_iter = row.cells.iter();
    // Now iterate over all cells and handle them according to their alignment
    for info in display_info.iter() {
        // Each cell is devided into several lines devided by newline
        // Every line that's too long will be split into two/several lines
        let mut cell_content = Vec::new();

        let cell = if let Some(cell) = cell_iter.next() {
            cell
        } else {
            cell_content.push(" ".repeat(info.width as usize));
            temp_row_content.push(cell_content);
            continue;
        };

        // We simply ignore hidden columns
        if info.hidden {
            continue;
        }

        // Iterate over each line and split it into multiple lines, if necessary.
        // Newlines added by the user will be preserved.
        for line in cell.content.iter() {
            if line.len() as u16 > info.width {
                let mut splitted = split_line(line.clone(), &info, cell);
                cell_content.append(&mut splitted);
            } else {
                let mut line = align_line(line.clone(), info, cell);
                if true {
                    line = style_line(line, cell);
                }
                cell_content.push(line);
            }
        }

        // Calculate the maximum amount of lines on this row.
        if cell_content.len() > max_content_lines {
            max_content_lines = cell_content.len();
        }

        temp_row_content.push(cell_content);
    }

    // Right now, we have a different structure than desired.
    // The content is organized by `row->cell->line`.
    // We want to remove the cell from our datastructure, since this makes the next step a lot easier.
    // In the end it should look like this: `row->line->column`.
    // To achieve this, we calculate the max amount of lines for the current row.
    // Afterwards, we iterate over each cell and convert the current structure to the desired one.
    // This step basically transforms this:
    //  tc[0][0][0]     tc[0][1][0]
    //  tc[0][0][1]     tc[0][1][1]
    //  tc[0][0][2]     This part of the line is missing
    //
    // to this:
    //  tc[0][0][0]     tc[0][0][1]
    //  tc[0][1][0]     tc[0][1][1]
    //  tc[0][2][0]     tc[0][2][1] <- Now filled with placeholder (spaces)
    let max_lines = temp_row_content
        .iter()
        .map(|cell| cell.len())
        .max()
        .unwrap_or(0);
    let mut row_content = Vec::new();
    for index in 0..max_lines {
        let mut line = Vec::new();
        let mut cell_iter = temp_row_content.iter();
        for info in display_info.iter() {
            let cell = cell_iter.next().unwrap();
            match cell.get(index) {
                // The current cell has content for this line. Append it
                Some(content) => line.push(content.clone()),
                // The current cell doesn't have content for this line.
                // Fill with a placeholder (empty spaces)
                None => line.push(" ".repeat(info.width as usize)),
            }
        }
        row_content.push(line);
    }

    row_content
}

/// Split a cell content line if it's longer than the specified columns width - padding
/// This function tries to do this in a smart way, by taking the content's deliminator
/// splitting it at these deliminators and reconnecting them until a line is full.
/// Splitting of parts only occurs if the part doesn't fit in a single line by itself.
pub fn split_line(line: String, info: &ColumnDisplayInfo, cell: &Cell) -> Vec<String> {
    let mut lines = Vec::new();
    let padding = info.padding.0 + info.padding.1;
    let content_width = info.width - padding;

    // Split the line by the given deliminator and turn the content into a stack.
    // Reverse it, since we want to push/pop without reversing the text.
    let mut splitted = line.split(' ').collect::<Vec<&str>>();
    splitted.reverse();

    let mut current_line = String::new();
    while let Some(next) = splitted.pop() {
        let current_length = current_line.chars().count();
        let next_length = next.chars().count();

        // The theoretical length of the current line after combining it with the next part
        let added_length = next_length + current_length + 1;

        // The line is empty try to add the next part
        if current_line.len() == 0 {
            // Next part fits in line. Add and continue
            if next_length <= content_width as usize {
                current_line += next;
                continue;

            // It doesn't fit, split it and put the remaining part back on the stack.
            } else {
                let (next, remaining) = next.split_at(content_width as usize);
                splitted.push(remaining);
                lines.push(next.to_string());
            }
        }
        // The next word/section fits into the current line
        else if added_length <= content_width as usize {
            current_line += " ";
            current_line += next;
            // Already push the next line, if there isn't space for more than to chars
            if current_line.chars().count() >= content_width as usize - 2 {
                lines.push(current_line);
                current_line = String::new();
            }
        // The next word/section doesn't fit
        } else {
            // The word is longer than the specified content_width
            // Split  the word, push the remaining string back on the stack
            if next_length > content_width as usize {
                let (next, remaining) = next.split_at(current_length + 1);
                current_line += " ";
                current_line += next;
                splitted.push(remaining);

                // Push the finished line, and start a new one
                lines.push(current_line);
                current_line = String::new();
            } else {
                // The next part fits into a single line.
                // Push the current line and make the next part the next line
                lines.push(current_line);
                current_line = next.to_string();
            }
        }
    }

    // Iterate over all generated lines of this cell and align them
    // If cell styling should be applied, do this here as well.
    lines = lines
        .iter()
        .map(|line| align_line(line.to_string(), info, cell))
        .map(|line| {
            if true {
                return style_line(line, cell);
            }
            line
        })
        .collect();

    lines
}

/// Apply the alignment for a column. Alignment can be either Left/Right/Center.
/// In every case all lines will be exactly the same character length `info.width - padding long`
/// This is needed, so we can simply insert it into the border frame later on.
/// Padding is applied in this function as well.
pub fn align_line(mut line: String, info: &ColumnDisplayInfo, cell: &Cell) -> String {
    let padding = info.padding.0 + info.padding.1;
    let content_width = info.width - padding;

    let remaining = content_width as f32 - line.chars().count() as f32;

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
            line += &" ".repeat(remaining as usize);
        }
        CellAlignment::Right => {
            line = " ".repeat(remaining as usize) + &line;
        }
        CellAlignment::Center => {
            let left_padding = (remaining / 2f32).ceil() as usize;
            let right_padding = (remaining / 2f32).floor() as usize;
            line = " ".repeat(left_padding) + &line + &" ".repeat(right_padding);
        }
    }

    pad_line(line, info)
}

/// Apply the column's padding to this line
pub fn pad_line(line: String, info: &ColumnDisplayInfo) -> String {
    let mut padded_line = String::new();

    padded_line += &" ".repeat(info.padding.0 as usize);
    padded_line += &line;
    padded_line += &" ".repeat(info.padding.1 as usize);

    padded_line
}

pub fn style_line(line: String, cell: &Cell) -> String {
    let mut content = style(line);

    // Apply frontend color
    if let Some(color) = cell.fg {
        content = content.with(color);
    }

    // Apply backend color
    if let Some(color) = cell.bg {
        content = content.on(color);
    }

    for attribute in cell.attributes.iter() {
        content = content.attribute(*attribute);
    }

    content.to_string()
}
