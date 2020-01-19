use crate::column::Column;
use crate::row::{Row, ToRow};
use crate::styling::table::{ContentArrangement, TableStyle};
use crate::utils::arrangement::arrange_content;
use crate::utils::borders::draw_borders;
use crate::utils::format::format_content;

/// The representation of a table.
pub struct Table {
    pub(crate) columns: Vec<Column>,
    header: Option<Row>,
    pub(crate) rows: Vec<Row>,
    pub table_style: TableStyle,
    pub(crate) arrangement: ContentArrangement,
}

impl Table {
    /// Create a new table with default ASCII styling, no rows and a header
    pub fn new() -> Self {
        Table {
            columns: Vec::new(),
            header: None,
            rows: Vec::new(),
            table_style: TableStyle::new(),
            arrangement: ContentArrangement::Disabled,
        }
    }

    pub fn to_string(&mut self) -> String {
        let display_info = arrange_content(self);
        let content = format_content(&self, &display_info);
        let lines = draw_borders(content, &self.table_style, &display_info);

        lines.join("\n")
    }

    /// Set the header row of the table. This is usually the title of each column.
    pub fn set_header<T: ToRow>(&mut self, mut row: T) -> &mut Self {
        let row = row.to_row();
        self.autogenerate_columns(&row);
        self.adjust_max_column_widths(&row);
        self.header = Some(row);
        self.table_style.has_header = true;

        self
    }

    pub fn get_header(&self) -> Option<&Row> {
        self.header.as_ref()
    }

    /// Add a new row to the table.
    pub fn add_row<T: ToRow>(&mut self, mut row: T) -> &mut Self {
        let mut row = row.to_row();
        self.autogenerate_columns(&row);
        self.adjust_max_column_widths(&row);
        row.index = Some(self.rows.len());
        self.rows.push(row);

        self
    }

    pub fn get_column(&self, index: usize) -> Option<&Column> {
        self.columns.get(index)
    }

    pub fn get_column_mut(&mut self, index: usize) -> Option<&mut Column> {
        self.columns.get_mut(index)
    }

    /// Autogenerate new columns, if a row is added with more cells than existing columns
    fn autogenerate_columns(&mut self, row: &Row) {
        if row.cell_count() > self.columns.len() {
            for index in self.columns.len()..row.cell_count() {
                self.columns.push(Column::new(index));
            }
        }
    }

    /// Update the max_content_width for all columns depending on the new row
    fn adjust_max_column_widths(&mut self, row: &Row) {
        let max_widths = row.max_content_widths();
        for (index, width) in max_widths.iter().enumerate() {
            // We expect this column to exist, since we autoenerate columns just before calling this function
            let mut column = self.columns.get_mut(index).unwrap();
            if column.max_content_width < *width as u16 {
                column.max_content_width = *width as u16;
            }
        }
    }

    /// Return a vector representing the maximum amount of characters in any line of this column.
    /// This is mostly needed for internal testing and formatting, but it can be interesting
    /// if you want to check how wide the longest line for each column is during runtime.
    pub fn column_max_content_widths(&self) -> Vec<u16> {
        self.columns.iter().map(|column| column.max_content_width).collect()
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_column_generation() {
        let mut table = Table::new();
        table.set_header(&vec!["thr", "four", "fivef"]);

        // When adding a new row, columns are automatically generated
        assert_eq!(table.columns.len(), 3);
        // The max content width is also correctly set for each column
        assert_eq!(table.column_max_content_widths(), vec![3, 4, 5]);

        // When adding a new row, the max content width is updated accordingly
        table.add_row(&vec!["four", "fivef", "very long text with 23"]);
        assert_eq!(table.column_max_content_widths(), vec![4, 5, 22]);

        // Now add a row that has column lines. The max content width shouldn't change
        table.add_row(&vec!["", "", "shorter"]);
        assert_eq!(table.column_max_content_widths(), vec![4, 5, 22]);
    }
}
