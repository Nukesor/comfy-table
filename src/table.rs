#[cfg(feature = "tty")]
use std::sync::OnceLock;
use std::{
    fmt,
    iter::IntoIterator,
    slice::{Iter, IterMut},
};

use crate::{
    cell::Cell,
    column::Column,
    row::Row,
    style::{ColumnConstraint, ContentArrangement, TableStyle, presets::ASCII_FULL},
    utils::build_table,
};

/// This is the main interface for building a table.
/// Each table consists of [Rows](Row), which in turn contain [Cells](crate::cell::Cell).
///
/// There also exists a representation of a [Column].
/// Columns are automatically created when adding rows to a table.
#[derive(Debug, Clone)]
pub struct Table {
    pub(crate) columns: Vec<Column>,
    pub(crate) style: TableStyle,
    pub(crate) header: Option<Row>,
    pub(crate) rows: Vec<Row>,
    pub(crate) arrangement: ContentArrangement,
    pub(crate) delimiter: Option<char>,
    pub(crate) truncation_indicator: String,
    #[cfg(feature = "tty")]
    no_tty: bool,
    #[cfg(feature = "tty")]
    is_tty_cache: OnceLock<bool>,
    #[cfg(feature = "tty")]
    use_stderr: bool,
    width: Option<u16>,
    #[cfg(feature = "tty")]
    enforce_styling: bool,
    /// Define whether everything in a cells should be styled, including whitespaces
    /// or whether only the text should be styled.
    #[cfg(feature = "tty")]
    pub(crate) style_text_only: bool,
}

impl fmt::Display for Table {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.lines().collect::<Vec<_>>().join("\n"))
    }
}

impl Default for Table {
    fn default() -> Self {
        Self::new()
    }
}

impl Table {
    /// Create a new table with default ASCII styling.
    pub fn new() -> Self {
        Self {
            columns: Vec::new(),
            header: None,
            rows: Vec::new(),
            arrangement: ContentArrangement::Disabled,
            delimiter: None,
            truncation_indicator: "…".to_string(),
            #[cfg(feature = "tty")]
            no_tty: false,
            #[cfg(feature = "tty")]
            is_tty_cache: OnceLock::new(),
            #[cfg(feature = "tty")]
            use_stderr: false,
            width: None,
            style: ASCII_FULL,
            #[cfg(feature = "tty")]
            enforce_styling: false,
            #[cfg(feature = "tty")]
            style_text_only: false,
        }
    }

    /// This is an alternative `fmt` function, which simply removes any trailing whitespaces.
    /// Trailing whitespaces often occur, when using tables without a right border.
    pub fn trim_fmt(&self) -> String {
        self.lines()
            .map(|line| line.trim_end().to_string())
            .collect::<Vec<_>>()
            .join("\n")
    }

    /// This is an alternative to `fmt`, but rather returns an iterator to each line, rather than
    /// one String separated by newlines.
    pub fn lines(&self) -> impl Iterator<Item = String> {
        build_table(self)
    }

    /// Set the header row of the table. This is usually the title of each column.\
    /// There'll be no header unless you explicitly set it with this function.
    ///
    /// ```
    /// use comfy_table::{Row, Table};
    ///
    /// let mut table = Table::new();
    /// let header = Row::from(vec!["Header One", "Header Two"]);
    /// table.set_header(header);
    /// ```
    pub fn set_header<T: Into<Row>>(&mut self, row: T) -> &mut Self {
        let row = row.into();
        self.autogenerate_columns(&row);
        self.header = Some(row);

        self
    }

    pub fn header(&self) -> Option<&Row> {
        self.header.as_ref()
    }

    /// Returns the number of currently present columns.
    ///
    /// ```
    /// use comfy_table::Table;
    ///
    /// let mut table = Table::new();
    /// table.set_header(vec!["Col 1", "Col 2", "Col 3"]);
    ///
    /// assert_eq!(table.column_count(), 3);
    /// ```
    pub fn column_count(&mut self) -> usize {
        self.discover_columns();
        self.columns.len()
    }

    /// Add a new row to the table.
    ///
    /// ```
    /// use comfy_table::{Row, Table};
    ///
    /// let mut table = Table::new();
    /// table.add_row(vec!["One", "Two"]);
    /// ```
    pub fn add_row<T: Into<Row>>(&mut self, row: T) -> &mut Self {
        let mut row = row.into();
        self.autogenerate_columns(&row);
        row.index = Some(self.rows.len());
        self.rows.push(row);

        self
    }

    /// Add a new row to the table if the predicate evaluates to `true`.
    ///
    /// ```
    /// use comfy_table::{Row, Table};
    ///
    /// let mut table = Table::new();
    /// table.add_row_if(|index, row| true, vec!["One", "Two"]);
    /// ```
    pub fn add_row_if<P, T>(&mut self, predicate: P, row: T) -> &mut Self
    where
        P: Fn(usize, &T) -> bool,
        T: Into<Row>,
    {
        if predicate(self.rows.len(), &row) {
            return self.add_row(row);
        }

        self
    }

    /// Add multiple rows to the table.
    ///
    /// ```
    /// use comfy_table::{Row, Table};
    ///
    /// let mut table = Table::new();
    /// let rows = vec![vec!["One", "Two"], vec!["Three", "Four"]];
    /// table.add_rows(rows);
    /// ```
    pub fn add_rows<I>(&mut self, rows: I) -> &mut Self
    where
        I: IntoIterator,
        I::Item: Into<Row>,
    {
        for row in rows.into_iter() {
            let mut row = row.into();
            self.autogenerate_columns(&row);
            row.index = Some(self.rows.len());
            self.rows.push(row);
        }

        self
    }

    /// Add multiple rows to the table if the predicate evaluates to `true`.
    ///
    /// ```
    /// use comfy_table::{Row, Table};
    ///
    /// let mut table = Table::new();
    /// let rows = vec![vec!["One", "Two"], vec!["Three", "Four"]];
    /// table.add_rows_if(|index, rows| true, rows);
    /// ```
    pub fn add_rows_if<P, I>(&mut self, predicate: P, rows: I) -> &mut Self
    where
        P: Fn(usize, &I) -> bool,
        I: IntoIterator,
        I::Item: Into<Row>,
    {
        if predicate(self.rows.len(), &rows) {
            return self.add_rows(rows);
        }

        self
    }

    /// Returns the number of currently present rows.
    ///
    /// ```
    /// use comfy_table::Table;
    ///
    /// let mut table = Table::new();
    /// table.add_row(vec!["One", "Two"]);
    ///
    /// assert_eq!(table.row_count(), 1);
    /// ```
    pub fn row_count(&self) -> usize {
        self.rows.len()
    }

    /// Returns if the table is empty (contains no data rows).
    ///
    /// ```
    /// use comfy_table::Table;
    ///
    /// let mut table = Table::new();
    /// assert!(table.is_empty());
    ///
    /// table.add_row(vec!["One", "Two"]);
    /// assert!(!table.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.rows.is_empty()
    }

    /// Enforce a max width that should be used in combination with [dynamic content
    /// arrangement](ContentArrangement::Dynamic).\ This is usually not necessary, if you plan
    /// to output your table to a tty, since the terminal width can be automatically determined.
    pub fn set_width(&mut self, width: u16) -> &mut Self {
        self.width = Some(width);

        self
    }

    /// Get the expected width of the table.
    ///
    /// This will be `Some(width)`, if the terminal width can be detected or if the table width is
    /// set via [set_width](Table::set_width).
    ///
    /// If neither is not possible, `None` will be returned.\
    /// This implies that both the [Dynamic](ContentArrangement::Dynamic) mode and the
    /// [Percentage](crate::style::Width::Percentage) constraint won't work.
    #[cfg(feature = "tty")]
    pub fn width(&self) -> Option<u16> {
        if let Some(width) = self.width {
            Some(width)
        } else if self.is_tty() {
            if let Ok((width, _)) = crossterm::terminal::size() {
                Some(width)
            } else {
                None
            }
        } else {
            None
        }
    }

    #[cfg(not(feature = "tty"))]
    pub fn width(&self) -> Option<u16> {
        self.width
    }

    /// Specify how Comfy Table should arrange the content in your table.
    ///
    /// ```
    /// use comfy_table::{ContentArrangement, Table};
    ///
    /// let mut table = Table::new();
    /// table.set_content_arrangement(ContentArrangement::Dynamic);
    /// ```
    pub fn set_content_arrangement(&mut self, arrangement: ContentArrangement) -> &mut Self {
        self.arrangement = arrangement;

        self
    }

    /// Get the current content arrangement of the table.
    pub fn content_arrangement(&self) -> ContentArrangement {
        self.arrangement.clone()
    }

    /// Set the delimiter used to split text in all cells.
    ///
    /// A custom delimiter on a cell in will overwrite the column's delimiter.\
    /// Normal text uses spaces (` `) as delimiters. This is necessary to help comfy-table
    /// understand the concept of _words_.
    pub fn set_delimiter(&mut self, delimiter: char) -> &mut Self {
        self.delimiter = Some(delimiter);

        self
    }

    /// Set the truncation indicator for cells that are too long to be displayed.
    ///
    /// Defaults to "…". Set it to "..." for example if you want to stick to ASCII.
    pub fn set_truncation_indicator(&mut self, indicator: &str) -> &mut Self {
        self.truncation_indicator = indicator.to_string();

        self
    }

    /// In case you are sure you don't want export tables to a tty or you experience
    /// problems with tty specific code, you can enforce a non_tty mode.
    ///
    /// This disables:
    ///
    /// - width lookup from the current tty
    /// - Styling and attributes on cells (unless you use [Table::enforce_styling])
    ///
    /// If you use the [dynamic content arrangement](ContentArrangement::Dynamic),
    /// you need to set the width of your desired table manually with [set_width](Table::set_width).
    #[cfg(feature = "tty")]
    pub fn force_no_tty(&mut self) -> &mut Self {
        self.no_tty = true;

        self
    }

    /// Use this function to check whether `stderr` is a tty.
    ///
    /// The default is `stdout`.
    #[cfg(feature = "tty")]
    pub fn use_stderr(&mut self) -> &mut Self {
        self.use_stderr = true;

        self
    }

    /// Returns whether the table will be handled as if it's printed to a tty.
    ///
    /// By default, comfy-table looks at `stdout` and checks whether it's a tty.
    /// This behavior can be changed via [Table::force_no_tty] and [Table::use_stderr].
    #[cfg(feature = "tty")]
    pub fn is_tty(&self) -> bool {
        use std::io::IsTerminal;

        if self.no_tty {
            return false;
        }

        *self.is_tty_cache.get_or_init(|| {
            if self.use_stderr {
                std::io::stderr().is_terminal()
            } else {
                std::io::stdout().is_terminal()
            }
        })
    }

    /// Enforce terminal styling.
    ///
    /// Only useful if you forcefully disabled tty, but still want those fancy terminal styles.
    ///
    /// ```
    /// use comfy_table::Table;
    ///
    /// let mut table = Table::new();
    /// table.force_no_tty().enforce_styling();
    /// ```
    #[cfg(feature = "tty")]
    pub fn enforce_styling(&mut self) -> &mut Self {
        self.enforce_styling = true;

        self
    }

    /// Returns whether the content of this table should be styled with the current settings and
    /// environment.
    #[cfg(feature = "tty")]
    pub fn should_style(&self) -> bool {
        if self.enforce_styling {
            return true;
        }
        self.is_tty()
    }

    /// By default, the whole content of a cells will be styled.
    /// Calling this function disables this behavior for all cells, resulting in
    /// only the text of cells being styled.
    #[cfg(feature = "tty")]
    pub fn style_text_only(&mut self) {
        self.style_text_only = true;
    }

    /// Convenience method to set a [ColumnConstraint] for all columns at once.
    /// Constraints are used to influence the way the columns will be arranged.
    /// Check out their docs for more information.
    ///
    /// **Attention:**
    /// This function should be called after at least one row (or the headers) has been added to the
    /// table. Before that, the columns won't initialized.
    ///
    /// If more constraints are passed than there are columns, any superfluous constraints will be
    /// ignored.
    ///
    /// ```
    /// use comfy_table::{CellAlignment, ColumnConstraint::*, ContentArrangement, Table, Width::*};
    ///
    /// let mut table = Table::new();
    /// table
    ///     .add_row(&vec!["one", "two", "three"])
    ///     .set_content_arrangement(ContentArrangement::Dynamic)
    ///     .set_constraints(vec![UpperBoundary(Fixed(15)), LowerBoundary(Fixed(20))]);
    /// ```
    pub fn set_constraints<T: IntoIterator<Item = ColumnConstraint>>(
        &mut self,
        constraints: T,
    ) -> &mut Self {
        let mut constraints = constraints.into_iter();
        for column in self.column_iter_mut() {
            if let Some(constraint) = constraints.next() {
                column.set_constraint(constraint);
            } else {
                break;
            }
        }

        self
    }

    /// Load a [TableStyle] for this table, replacing the current style. \
    /// Preset styles can be found in the [presets](crate::style::presets) module.
    ///
    /// You can also build your own styles by creating your own [TableStyle].
    ///
    /// ```
    /// use comfy_table::{Table, presets::UTF8_FULL};
    ///
    /// let mut table = Table::new();
    /// table.load_style(UTF8_FULL.with_rounded_corners());
    /// ```
    pub fn load_style(&mut self, style: TableStyle) -> &mut Self {
        self.style = style;

        self
    }

    /// Returns a copy of the table's current [TableStyle].
    ///
    /// ```
    /// use comfy_table::{Table, presets::UTF8_FULL};
    ///
    /// let mut table = Table::new();
    /// table.load_style(UTF8_FULL);
    ///
    /// assert_eq!(UTF8_FULL, table.style())
    /// ```
    pub fn style(&self) -> TableStyle {
        self.style
    }

    /// Get a mutable handle to the table's [TableStyle] to edit it in place.
    ///
    /// ```
    /// use comfy_table::{Table, presets::UTF8_FULL};
    ///
    /// let mut table = Table::new();
    /// // Load the UTF8_FULL style
    /// table.load_style(UTF8_FULL);
    /// // Set all outer corners to round UTF8 corners
    /// // This is basically the same as TableStyle::with_rounded_corners
    /// let style = table.style_mut();
    /// style.top_border.left = Some('╭');
    /// style.top_border.right = Some('╮');
    /// style.bottom_border.left = Some('╰');
    /// style.bottom_border.right = Some('╯');
    /// ```
    pub fn style_mut(&mut self) -> &mut TableStyle {
        &mut self.style
    }

    /// Get a reference to a specific column.
    pub fn column(&self, index: usize) -> Option<&Column> {
        self.columns.get(index)
    }

    /// Get a mutable reference to a specific column.
    pub fn column_mut(&mut self, index: usize) -> Option<&mut Column> {
        self.columns.get_mut(index)
    }

    /// Iterator over all columns
    pub fn column_iter(&self) -> Iter<'_, Column> {
        self.columns.iter()
    }

    /// Get a mutable iterator over all columns.
    ///
    /// ```
    /// use comfy_table::{ColumnConstraint::*, Table, Width::*};
    ///
    /// let mut table = Table::new();
    /// table.add_row(&vec!["First", "Second", "Third"]);
    ///
    /// // Add a ColumnConstraint to each column (left->right)
    /// // first -> min width of 10
    /// // second -> max width of 8
    /// // third -> fixed width of 10
    /// let constraints = vec![
    ///     LowerBoundary(Fixed(10)),
    ///     UpperBoundary(Fixed(8)),
    ///     Absolute(Fixed(10)),
    /// ];
    ///
    /// // Add the constraints to their respective column
    /// for (column_index, column) in table.column_iter_mut().enumerate() {
    ///     let constraint = constraints.get(column_index).unwrap();
    ///     column.set_constraint(*constraint);
    /// }
    /// ```
    pub fn column_iter_mut(&mut self) -> IterMut<'_, Column> {
        self.columns.iter_mut()
    }

    /// Get a mutable iterator over cells of a column.
    /// The iterator returns a nested `Option<Option<Cell>>`, since there might be
    /// rows that are missing this specific Cell.
    ///
    /// ```
    /// use comfy_table::Table;
    /// let mut table = Table::new();
    /// table.add_row(&vec!["First", "Second"]);
    /// table.add_row(&vec!["Third"]);
    /// table.add_row(&vec!["Fourth", "Fifth"]);
    ///
    /// // Create an iterator over the second column
    /// let mut cell_iter = table.column_cells_iter(1);
    /// assert_eq!(cell_iter.next().unwrap().unwrap().content(), "Second");
    /// assert!(cell_iter.next().unwrap().is_none());
    /// assert_eq!(cell_iter.next().unwrap().unwrap().content(), "Fifth");
    /// assert!(cell_iter.next().is_none());
    /// ```
    pub fn column_cells_iter(&self, column_index: usize) -> ColumnCellIter<'_> {
        ColumnCellIter {
            rows: &self.rows,
            column_index,
            row_index: 0,
        }
    }

    /// Get a mutable iterator over cells of a column, including the header cell.
    /// The header cell will be the very first cell returned.
    /// The iterator returns a nested `Option<Option<Cell>>`, since there might be
    /// rows that are missing this specific Cell.
    ///
    /// ```
    /// use comfy_table::Table;
    /// let mut table = Table::new();
    /// table.set_header(&vec!["A", "B"]);
    /// table.add_row(&vec!["First", "Second"]);
    /// table.add_row(&vec!["Third"]);
    /// table.add_row(&vec!["Fourth", "Fifth"]);
    ///
    /// // Create an iterator over the second column
    /// let mut cell_iter = table.column_cells_with_header_iter(1);
    /// assert_eq!(cell_iter.next().unwrap().unwrap().content(), "B");
    /// assert_eq!(cell_iter.next().unwrap().unwrap().content(), "Second");
    /// assert!(cell_iter.next().unwrap().is_none());
    /// assert_eq!(cell_iter.next().unwrap().unwrap().content(), "Fifth");
    /// assert!(cell_iter.next().is_none());
    /// ```
    pub fn column_cells_with_header_iter(
        &self,
        column_index: usize,
    ) -> ColumnCellsWithHeaderIter<'_> {
        ColumnCellsWithHeaderIter {
            header_checked: false,
            header: &self.header,
            rows: &self.rows,
            column_index,
            row_index: 0,
        }
    }

    /// Reference to a specific row
    pub fn row(&self, index: usize) -> Option<&Row> {
        self.rows.get(index)
    }

    /// Mutable reference to a specific row
    pub fn row_mut(&mut self, index: usize) -> Option<&mut Row> {
        self.rows.get_mut(index)
    }

    /// Iterator over all rows
    pub fn row_iter(&self) -> Iter<'_, Row> {
        self.rows.iter()
    }

    /// Get a mutable iterator over all rows.
    ///
    /// ```
    /// use comfy_table::Table;
    /// let mut table = Table::new();
    /// table.add_row(&vec!["First", "Second", "Third"]);
    ///
    /// // Add the constraints to their respective row
    /// for row in table.row_iter_mut() {
    ///     row.max_height(5);
    /// }
    /// assert!(table.row_iter_mut().len() == 1);
    /// ```
    pub fn row_iter_mut(&mut self) -> IterMut<'_, Row> {
        self.rows.iter_mut()
    }

    /// Return a vector representing the maximum amount of characters in any line of this column.\
    ///
    /// **Attention** This scans the whole current content of the table.
    pub fn column_max_content_widths(&self) -> Vec<u16> {
        fn set_max_content_widths(max_widths: &mut [u16], row: &Row) {
            // Get the max width for each cell of the row
            let row_max_widths = row.max_content_widths();
            for (index, width) in row_max_widths.iter().enumerate() {
                let mut width = (*width).try_into().unwrap_or(u16::MAX);
                // A column's content is at least 1 char wide.
                width = std::cmp::max(1, width);

                // Set a new max, if the current cell is the longest for that column.
                let current_max = max_widths[index];
                if current_max < width {
                    max_widths[index] = width;
                }
            }
        }
        // The vector that'll contain the max widths per column.
        let mut max_widths = vec![0; self.columns.len()];

        if let Some(header) = &self.header {
            set_max_content_widths(&mut max_widths, header);
        }
        // Iterate through all rows of the table.
        for row in self.rows.iter() {
            set_max_content_widths(&mut max_widths, row);
        }

        max_widths
    }

    /// Autogenerate new columns, if a row is added with more cells than existing columns.
    fn autogenerate_columns(&mut self, row: &Row) {
        if row.cell_count() > self.columns.len() {
            for index in self.columns.len()..row.cell_count() {
                self.columns.push(Column::new(index));
            }
        }
    }

    /// Calling this might be necessary if you add new cells to rows that're already added to the
    /// table.
    ///
    /// If more cells than're currently know to the table are added to that row,
    /// the table cannot know about these, since new [Column]s are only
    /// automatically detected when a new row is added.
    ///
    /// To make sure everything works as expected, just call this function if you're adding cells
    /// to rows that're already added to the table.
    pub fn discover_columns(&mut self) {
        for row in self.rows.iter() {
            if row.cell_count() > self.columns.len() {
                for index in self.columns.len()..row.cell_count() {
                    self.columns.push(Column::new(index));
                }
            }
        }
    }
}

/// An iterator over cells of a specific column.
/// A dedicated struct is necessary, as data is usually handled by rows and thereby stored in
/// `Table::rows`. This type is returned by [Table::column_cells_iter].
pub struct ColumnCellIter<'a> {
    rows: &'a [Row],
    column_index: usize,
    row_index: usize,
}

impl<'a> Iterator for ColumnCellIter<'a> {
    type Item = Option<&'a Cell>;
    fn next(&mut self) -> Option<Option<&'a Cell>> {
        // Check if there's a next row
        if let Some(row) = self.rows.get(self.row_index) {
            self.row_index += 1;

            // Return the cell (if it exists).
            return Some(row.cells.get(self.column_index));
        }

        None
    }
}

/// An iterator over cells of a specific column.
/// A dedicated struct is necessary, as data is usually handled by rows and thereby stored in
/// `Table::rows`. This type is returned by [Table::column_cells_iter].
pub struct ColumnCellsWithHeaderIter<'a> {
    header_checked: bool,
    header: &'a Option<Row>,
    rows: &'a [Row],
    column_index: usize,
    row_index: usize,
}

impl<'a> Iterator for ColumnCellsWithHeaderIter<'a> {
    type Item = Option<&'a Cell>;
    fn next(&mut self) -> Option<Option<&'a Cell>> {
        // Get the header as the first cell
        if !self.header_checked {
            self.header_checked = true;

            return match self.header {
                Some(header) => {
                    // Return the cell (if it exists).
                    Some(header.cells.get(self.column_index))
                }
                None => Some(None),
            };
        }

        // Check if there's a next row
        if let Some(row) = self.rows.get(self.row_index) {
            self.row_index += 1;

            // Return the cell (if it exists).
            return Some(row.cells.get(self.column_index));
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_column_generation() {
        let mut table = Table::new();
        table.set_header(vec!["thr", "four", "fivef"]);

        // When adding a new row, columns are automatically generated
        assert_eq!(table.columns.len(), 3);
        // The max content width is also correctly set for each column
        assert_eq!(table.column_max_content_widths(), vec![3, 4, 5]);

        // When adding a new row, the max content width is updated accordingly
        table.add_row(vec!["four", "fivef", "very long text with 23"]);
        assert_eq!(table.column_max_content_widths(), vec![4, 5, 22]);

        // Now add a row that has column lines. The max content width shouldn't change
        table.add_row(vec!["", "", "shorter"]);
        assert_eq!(table.column_max_content_widths(), vec![4, 5, 22]);

        println!("{table}");
    }
}
