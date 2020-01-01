use crate::table::Table;
use crate::utils::arrangement::ColumnDisplayInfo;


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

            // Iterate over each line, add it to the
            for line in cell.content.iter() {
                if line.len() as u16 > info.width {


                } else {
                    cell_content.push(line.clone());
                }
            }

            row_content.push(cell_content);
        }

        table_content.push(row_content);
    }

    table_content
}



pub fn split_line(line: &String, width: u16, info: &ColumnDisplayInfo) -> Vec<String> {
    let mut lines = Vec::new();
    let padding = info.padding.0 + info.padding.1;
    let content_width = info.width - padding;


    lines
}


pub fn allign_line(line: String, info: &ColumnDisplayInfo) -> String {

    line
}


pub fn pad_line(line: String, info: &ColumnDisplayInfo) -> String {

    line
}
