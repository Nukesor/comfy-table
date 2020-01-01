use crate::column::Column;
use crate::column::Constraint::*;
use crate::row::Row;
use crate::styling::table::TableStyle;


pub enum ContentArrangement {
    /// Don't do any automatic width calculation.
    /// Table with this mode might overflow and look ugly, if content gets too long.
    /// Constraints on columns are still respected.
    Disabled,
    /// Automatically determine the width of columns in regard to terminal width and content length.
    /// With this mode, the content in cells will wrap automatically and comfy-table tries to determine
    /// the best column layout for the given content.
    /// Constraints on columns are still respected.
    Automatic,

    /// Same as Automatic, but the full width of the terminal will always be used.
    /// Use this, if you don't want tables that differ in width.
    /// Constraints on columns are still respected.
    Full,
}


/// The representation of a table.
pub struct Table {
    pub (crate) columns: Vec<Column>,
    header: Row,
    rows: Vec<Row>,
    pub style: TableStyle,
    pub (crate) arrangement: ContentArrangement,
}

impl Table {
    /// Create a new table with default ASCII styling, no rows and a header
    pub fn new(header: Row) -> Self {
        Table {
            columns: Vec::new(),
            header: header,
            rows: Vec::new(),
            style: TableStyle::new(),
            arrangement: ContentArrangement::Automatic,
        }
    }

    pub fn to_string(&mut self) {

    }

    /// Set the header row of the table. This is usually the title of each column.
    pub fn set_header(&mut self, row: Row) -> &mut Self {
        self.header = row;

        self
    }

    /// Add a new row to the table.
    pub fn add_row(&mut self, mut row: Row) -> &mut Self {
        self.autogenerate_columns(&row);
        self.adjust_column_width(&row);
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
    fn adjust_column_width(&mut self, row: &Row) {
        let max_widths = row.max_content_widths();
        for (index, width) in max_widths.iter().enumerate() {
            // We expect this column to exist, since we autoenerate columns just before calling this function
            let mut column = self.columns.get_mut(index).unwrap();
            column.max_content_width = *width as u16;
        }
    }

    // Get the width of the longest row.
    // This is needed to automatically calculate the amount of columns that need to be created.
    // # Comment for now, maybe we don't need this at all.
    // fn get_max_column(&self) -> usize {
    //     let mut width;
    //     let mut longest = 0;
    //     for row in self.rows.iter() {
    //         width = row.cell_count();
    //         if width > longest {
    //             longest = width
    //         }
    //     }
    //     longest
    // }
}
