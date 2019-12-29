use crate::column::Column;
use crate::row::Row;
use crate::styling::table::TableStyle;

/// The representation of a table.
pub struct Table {
    columns: Vec<Column>,
    header: Row,
    rows: Vec<Row>,
    pub style: TableStyle,
}

impl Table {
    /// Create a new table with default ASCII styling, no rows and a header
    pub fn new(header: Row) -> Self {
        Table {
            columns: Vec::new(),
            header: header,
            rows: Vec::new(),
            style: TableStyle::new(),
        }
    }

    pub fn to_str(&mut self) {}

    /// Set the header row of the table. This is usually the title of each column.
    pub fn set_header(&mut self, row: Row) -> &mut Self {
        self.header = row;

        self
    }

    /// Add a new row to the table.
    pub fn add_row(&mut self, row: Row) -> &mut Self {
        self.autogenerate_columns(&row);
        self.rows.push(row);

        self
    }

    /// In case the user didn't supply any columns, we need to determine, how many columns should be generated.
    fn autogenerate_columns(&mut self, row: &Row) {
        let column_count = row.cell_count();
        let new_columns = column_count - self.columns.len();
        for index in 0.. {
            self.columns.push(Column::new());
        }
    }

    // Get the length of the longest row.
    // This is needed to automatically calculate the amount of columns that need to be created.
    // # Comment for now, maybe we don't need this at all.
    // fn get_max_column(&self) -> usize {
    //     let mut length;
    //     let mut longest = 0;
    //     for row in self.rows.iter() {
    //         length = row.cell_count();
    //         if length > longest {
    //             longest = length
    //         }
    //     }
    //     longest
    // }
}
