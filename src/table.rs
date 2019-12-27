use crate::row::Row;
use crate::column::Column;
use crate::styling::table::TableStyle;


/// The representation of a table.
pub struct Table {
    autogenerate_columns: bool,
    columns: Vec<Column>,
    header: Row,
    rows: Vec<Row>,
    pub style: TableStyle,
}



impl Table {
    pub fn new(header: Row) -> Self {
        Table {
            autogenerate_columns: true,
            columns: Vec::new(),
            header: header,
            rows: Vec::new(),
            style: TableStyle::new(),
        }
    }

    pub fn to_str(&mut self) {
        if self.autogenerate_columns {
            self.autogenerate_columns()
        }
    }

    /// Set the header row of the table. This is usually the title of each column.
    pub fn set_header(&mut self, row: Row) -> &mut Self {
        self.header = row;

        self
    }

    /// Add a new row to the table.
    pub fn add_row(&mut self, row: Row) -> &mut Self {
        self.rows.push(row);

        self
    }

    /// Add a new column
    /// Only use this, if you don't use automatic formatting and if you want to style your columns on your own.
    /// *Caution:*
    /// If you use this, only the columns you added will be actually printed.
    /// Any cells in rows that don't belong to a column will be discarded.
    pub fn add_column(&mut self, column: Column) -> &mut Self {
        self.columns.push(column);
        self.autogenerate_columns = false;

        self
    }


    /// In case the user didn't supply any columns, we need to determine, how many columns should be generated.
    fn autogenerate_columns(&mut self) {
        let column_count = self.get_max_column();
        for index in 0..column_count {
        }
    }

    fn get_max_column(&self) -> usize {
        let mut length;
        let mut longest = 0;
        for row in self.rows.iter() {
            length = row.cell_count();
            if length > longest {
                longest = length
            }
        }
        longest
    }
}
