use crate::table::Table;
use crate::utils::arrangement::ColumnDisplayInfo;
use crate::styling::cell::CellAlignment;


/// Returns the formatted content of the table.
/// The content is organized in the following structure
///
/// tc stands for table content and represents the returned value
/// ``` text
///      column1          column2
/// row1 tc[0][0][0]      tc[0][1][0]
///      tc[0][0][1]      tc[0][1][1]
///      tc[0][0][2]      tc[0][1][2]
///
/// row2 tc[1][0][0]      tc[1][1][0]
///      tc[1][0][1]      tc[1][1][1]
///      tc[1][0][2]      tc[1][1][2]
/// ```
///
/// The strings for each row will be padded and aligned accordingly
/// to their respective column.
pub fn format_content(table: &Table, display_info: Vec<ColumnDisplayInfo>) -> Vec<Vec<Vec<String>>> {
    // The content of the whole table
    let mut table_content = Vec::new();
    for (row_index, row) in table.rows.iter().enumerate() {
        // The content of this specific row
        let mut row_content = Vec::new();

        // Now iterate over all cells and handle them according to their alignment
        for (column_index, cell) in row.cells.iter().enumerate() {
            // Each cell is devided into several lines devided by newline
            // Every line that's too long will be split into two/several lines
            let mut cell_content = Vec::new();

            let info = display_info.get(column_index).unwrap();
            // We simply ignore hidden columns
            if info.hidden {
                continue
            }

            // Iterate over each line and split it into multiple, if necessary.
            // Newlines added by the user will be preserved.
            for line in cell.content.iter() {
                if line.len() as u16 > info.width {
                    let mut splitted = split_line(line.clone(), &info);
                    cell_content.append(&mut splitted);
                } else {

                    cell_content.push(align_line(line.clone(), info));
                }
            }

            row_content.push(cell_content);
        }

        table_content.push(row_content);
    }

    table_content
}



pub fn split_line(line: String, info: &ColumnDisplayInfo) -> Vec<String> {
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
                continue

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
    lines = lines.iter().map(|line| align_line(line.to_string(), info)).collect();

    lines
}


// Apply the alignment for a column. Alignment can be either Left/Right/Center.
// In every case all lines will be exactly the same character length `info.width - padding long`
// This is needed, so we can simply insert it into the border frame later on.
pub fn align_line(mut line: String, info: &ColumnDisplayInfo) -> String {
    let padding = info.padding.0 + info.padding.1;
    let content_width = info.width - padding;

    let remaining = content_width as f32 - line.chars().count() as f32;

    // Determine the alignment of the column cells. Default is Left
    let alignment = if let Some(alignment) = info.cell_alignment {
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

    line
}


pub fn pad_line(line: String, info: &ColumnDisplayInfo) -> String {
    let mut padded_line = String::new();

    padded_line += &" ".repeat(info.padding.0 as usize);
    padded_line += &line;
    padded_line += &" ".repeat(info.padding.1 as usize);

    padded_line
}
