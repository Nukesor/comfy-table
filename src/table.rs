use std::collections::HashMap;
use std::fmt;
use std::iter::IntoIterator;
use std::slice::{Iter, IterMut};

use crossterm::terminal::size;
use crossterm::tty::IsTty;
use strum::IntoEnumIterator;

use crate::cell::Cell;
use crate::column::Column;
use crate::row::Row;
use crate::style::presets::ASCII_FULL;
use crate::style::{ColumnConstraint, ContentArrangement, TableComponent};
use crate::utils::arrangement::arrange_content;
use crate::utils::borders::draw_borders;
use crate::utils::format::format_content;

/// This is the main interface for building a table.
/// Each table consists of [Rows](Row), which in turn contain [Cells](crate::cell::Cell).
///
/// There also exists a representation of a [Column].
/// Columns are automatically created when adding rows to a table.
#[derive(Debug)]
pub struct Table {
    pub(crate) columns: Vec<Column>,
    style: HashMap<TableComponent, char>,
    pub(crate) header: Option<Row>,
    pub(crate) rows: Vec<Row>,
    pub(crate) arrangement: ContentArrangement,
    pub(crate) delimiter: Option<char>,
    no_tty: bool,
    table_width: Option<u16>,
    enforce_styling: bool,
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
        let mut table = Table {
            columns: Vec::new(),
            header: None,
            rows: Vec::new(),
            arrangement: ContentArrangement::Disabled,
            delimiter: None,
            no_tty: false,
            table_width: None,
            style: HashMap::new(),
            enforce_styling: false,
        };

        table.load_preset(ASCII_FULL);

        table
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
        let display_info = arrange_content(self);
        let content = format_content(&self, &display_info);
        draw_borders(&self, content, &display_info).into_iter()
    }

    /// Set the header row of the table. This is usually the title of each column.\
    /// There'll be no header unless you explicitly set it with this function.
    ///
    /// ```
    /// use comfy_table::{Table, Row};
    ///
    /// let mut table = Table::new();
    /// let header = Row::from(vec!["Header One", "Header Two"]);
    /// table.set_header(header);
    /// ```

    pub fn set_header<T: Into<Row>>(&mut self, row: T) -> &mut Self {
        let row = row.into();
        self.autogenerate_columns(&row);
        self.adjust_max_column_widths(&row);
        self.header = Some(row);

        self
    }

    pub fn get_header(&self) -> Option<&Row> {
        self.header.as_ref()
    }

    /// Add a new row to the table.
    ///
    /// ```
    /// use comfy_table::{Table, Row};
    ///
    /// let mut table = Table::new();
    /// let row = Row::from(vec!["One", "Two"]);
    /// table.add_row(row);
    /// ```
    pub fn add_row<T: Into<Row>>(&mut self, row: T) -> &mut Self {
        let mut row = row.into();
        self.autogenerate_columns(&row);
        self.adjust_max_column_widths(&row);
        row.index = Some(self.rows.len());
        self.rows.push(row);

        self
    }
    /// Enforce a max width that should be used in combination with [dynamic content arrangement](ContentArrangement::Dynamic).\
    /// This is usually not necessary, if you plan to output your table to a tty,
    /// since the terminal width can be automatically determined.
    pub fn set_table_width(&mut self, table_width: u16) -> &mut Self {
        self.table_width = Some(table_width);

        self
    }

    /// Get the expected width of the table.
    ///
    /// This will be `Some(width)`, if the terminal width can be detected or if the table width is set via [set_table_width](Table::set_table_width).
    ///
    /// If neither is not possible, `None` will be returned.\
    /// This implies that both the [Dynamic](ContentArrangement::Dynamic) mode and the [Percentage](crate::style::ColumnConstraint::Percentage) constraint won't work.
    pub fn get_table_width(&self) -> Option<u16> {
        if let Some(width) = self.table_width {
            Some(width)
        } else if self.is_tty() {
            let (table_width, _) = size().unwrap();
            Some(table_width)
        } else {
            None
        }
    }

    /// Specify how Comfy Table should arrange the content in your table.
    ///
    /// ```
    /// use comfy_table::{Table, ContentArrangement};
    ///
    /// let mut table = Table::new();
    /// table.set_content_arrangement(ContentArrangement::Dynamic);
    /// ```
    pub fn set_content_arrangement(&mut self, arrangement: ContentArrangement) -> &mut Self {
        self.arrangement = arrangement;

        self
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

    /// In case you are sure you don't want export tables to a tty or you experience
    /// problems with tty specific code, you can enforce a non_tty mode.
    ///
    /// This disables:
    ///
    /// - table_width lookup from the current tty
    /// - Styling and attributes on cells (unless you use [Table::enforce_styling])
    ///
    /// If you use the [dynamic content arrangement](ContentArrangement::Dynamic),
    /// you need to set the width of your desired table manually with [set_table_width](Table::set_table_width).
    pub fn force_no_tty(&mut self) -> &mut Self {
        self.no_tty = true;

        self
    }

    /// Returns whether the table will be handled as if it's printed to a tty.
    ///
    /// This function respects the [Table::force_no_tty] function.\
    /// Otherwise we try to determine, if we are on a tty.
    pub fn is_tty(&self) -> bool {
        if self.no_tty {
            return false;
        }

        ::std::io::stdout().is_tty()
    }

    /// Enforce terminal styling.
    ///
    /// Only useful if you forcefully disabled tty, but still want those fancy terminal styles.
    ///
    /// ```
    /// use comfy_table::Table;
    ///
    /// let mut table = Table::new();
    /// table.force_no_tty()
    ///     .enforce_styling();
    /// ```
    pub fn enforce_styling(&mut self) -> &mut Self {
        self.enforce_styling = true;

        self
    }

    /// Returns whether the content of this table should be styled with the current settings and
    /// environment.
    pub fn should_style(&self) -> bool {
        if self.enforce_styling {
            return true;
        }
        self.is_tty()
    }

    /// Convenience method to set a [ColumnConstraint] for all columns at once.
    ///
    /// Simply pass any iterable with ColumnConstraints.\
    /// If more constraints are passed than there are columns, the superfluous constraints will be ignored.
    /// ```
    /// use comfy_table::{Table, ColumnConstraint, ContentArrangement};
    ///
    /// let mut table = Table::new();
    /// table.add_row(&vec!["one", "two", "three"])
    ///     .set_content_arrangement(ContentArrangement::Dynamic)
    ///     .set_constraints(vec![
    ///         ColumnConstraint::MaxWidth(15),
    ///         ColumnConstraint::MinWidth(20),
    /// ]);
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

    /// This function creates a TableStyle from a given preset string.\
    /// Preset strings can be found in `styling::presets::*`.
    ///
    /// You can also write your own preset strings and use them with this function.
    /// There's the convenience method [Table::current_style_as_preset], which prints you a preset
    /// string from your current style configuration. \
    /// The function expects the to-be-drawn characters to be in the same order as in the [TableComponent] enum.
    ///
    /// If the string isn't long enough, the default [ASCII_FULL] style will be used for all remaining components.
    ///
    /// If the string is too long, remaining charaacters will be simply ignored.
    pub fn load_preset(&mut self, preset: &str) -> &mut Self {
        let mut components = TableComponent::iter();

        for character in preset.chars() {
            if let Some(component) = components.next() {
                // White spaces mean "don't draw this" in presets
                // If we want to override the default preset, we need to remove
                // this component from the HashMap in case we find a whitespace.
                if character == ' ' {
                    self.remove_style(component);
                    continue;
                }

                self.set_style(component, character);
            } else {
                break;
            }
        }

        self
    }

    /// Returns the current style as a preset string.
    ///
    /// A pure convenience method, so you're not force to fiddle with those preset strings yourself.
    ///
    /// ```
    /// use comfy_table::Table;
    /// use comfy_table::presets::UTF8_FULL;
    ///
    /// let mut table = Table::new();
    /// table.load_preset(UTF8_FULL);
    ///
    /// assert_eq!(UTF8_FULL, table.current_style_as_preset())
    /// ```
    pub fn current_style_as_preset(&mut self) -> String {
        let components = TableComponent::iter();
        let mut preset_string = String::new();

        for component in components {
            match self.get_style(component) {
                None => preset_string.push(' '),
                Some(character) => preset_string.push(character),
            }
        }

        preset_string
    }

    /// Modify a preset with a modifier string from [modifiers](crate::style::modifiers).
    ///
    /// For instance, the [UTF8_ROUND_CORNERS](crate::style::modifiers::UTF8_ROUND_CORNERS) modifies all corners to be round UTF8 box corners.
    ///
    /// ```
    /// use comfy_table::Table;
    /// use comfy_table::presets::UTF8_FULL;
    /// use comfy_table::modifiers::UTF8_ROUND_CORNERS;
    ///
    /// let mut table = Table::new();
    /// table.load_preset(UTF8_FULL);
    /// table.apply_modifier(UTF8_ROUND_CORNERS);
    /// ```

    pub fn apply_modifier(&mut self, modifier: &str) -> &mut Self {
        let mut components = TableComponent::iter();

        for character in modifier.chars() {
            // Skip spaces while applying modifiers.
            if character == ' ' {
                components.next();
                continue;
            }
            if let Some(component) = components.next() {
                self.set_style(component, character);
            } else {
                break;
            }
        }

        self
    }

    /// Define the char that will be used to draw a specific component.\
    /// Look at [TableComponent] to see all stylable components
    ///
    /// If `None` is supplied, the element won't be displayed.\
    /// In case of a e.g. *BorderIntersection a whitespace will be used as placeholder,
    /// unless related borders and and corners are set to `None` as well.
    ///
    /// For example, if `TopBorderIntersections` is `None` the first row would look like this:
    ///
    /// ```text
    /// +------ ------+
    /// | this | test |
    /// ```
    ///
    /// If in addition `TopLeftCorner`,`TopBorder` and `TopRightCorner` would be `None` as well,
    /// the first line wouldn't be displayed at all.
    ///
    /// ```
    /// use comfy_table::Table;
    /// use comfy_table::presets::UTF8_FULL;
    /// use comfy_table::TableComponent::*;
    ///
    /// let mut table = Table::new();
    /// // Load the UTF8_FULL preset
    /// table.load_preset(UTF8_FULL);
    /// // Set all outer corners to round UTF8 corners
    /// // This is basically the same as the UTF8_ROUND_CORNERS modifier
    /// table.set_style(TopLeftCorner, '╭');
    /// table.set_style(TopRightCorner, '╮');
    /// table.set_style(BottomLeftCorner, '╰');
    /// table.set_style(BottomRightCorner, '╯');
    /// ```
    pub fn set_style(&mut self, component: TableComponent, character: char) -> &mut Self {
        self.style.insert(component, character);

        self
    }

    /// Get a copy of the char that's currently used for drawing this component.
    /// ```
    /// use comfy_table::Table;
    /// use comfy_table::TableComponent::*;
    ///
    /// let mut table = Table::new();
    /// assert_eq!(table.get_style(TopLeftCorner), Some('+'));
    /// ```

    pub fn get_style(&mut self, component: TableComponent) -> Option<char> {
        self.style.get(&component).copied()
    }

    /// Remove the style for a specific component of the table.\
    /// By default, a space will be used as a placeholder instead.\
    /// Though, if for instance all components of the left border are removed, the left border won't be displayed.
    pub fn remove_style(&mut self, component: TableComponent) -> &mut Self {
        self.style.remove(&component);

        self
    }

    /// Get a reference to a specific column.
    pub fn get_column(&self, index: usize) -> Option<&Column> {
        self.columns.get(index)
    }

    /// Get a mutable reference to a specific column.
    pub fn get_column_mut(&mut self, index: usize) -> Option<&mut Column> {
        self.columns.get_mut(index)
    }

    /// Iterator over all columns
    pub fn column_iter(&mut self) -> Iter<Column> {
        self.columns.iter()
    }

    /// Get a mutable iterator over all columns.
    ///
    /// ```
    /// use comfy_table::{Table, ColumnConstraint};
    /// let mut table = Table::new();
    /// table.add_row(&vec!["First", "Second", "Third"]);
    ///
    /// // Add a ColumnConstraint to each column (left->right)
    /// // first -> min width of 10
    /// // second -> max width of 8
    /// // third -> fixed width of 10
    /// let constraints = vec![
    ///     ColumnConstraint::MinWidth(10),
    ///     ColumnConstraint::MaxWidth(8),
    ///     ColumnConstraint::Width(10),
    /// ];
    ///
    /// // Add the constraints to their respective column
    /// for (column_index, column) in table.column_iter_mut().enumerate() {
    ///     let constraint = constraints.get(column_index).unwrap();
    ///     column.set_constraint(*constraint);
    /// }
    /// ```
    pub fn column_iter_mut(&mut self) -> IterMut<Column> {
        self.columns.iter_mut()
    }

    /// Get a mutable iterator over cells of a column.
    /// The iterator returns a nested Option<Option<Cell>>, since there might be
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
    /// assert_eq!(cell_iter.next().unwrap().unwrap().get_content(), "Second");
    /// assert!(cell_iter.next().unwrap().is_none());
    /// assert_eq!(cell_iter.next().unwrap().unwrap().get_content(), "Fifth");
    /// assert!(cell_iter.next().is_none());
    /// ```
    pub fn column_cells_iter(&self, column_index: usize) -> ColumnCellIter {
        ColumnCellIter {
            rows: &self.rows,
            column_index,
            row_index: 0,
        }
    }

    /// Reference to a specific row
    pub fn get_row(&self, index: usize) -> Option<&Row> {
        self.rows.get(index)
    }

    /// Mutable reference to a specific row
    pub fn get_row_mut(&mut self, index: usize) -> Option<&mut Row> {
        self.rows.get_mut(index)
    }

    /// Iterator over all rows
    pub fn row_iter(&mut self) -> Iter<Row> {
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
    pub fn row_iter_mut(&mut self) -> IterMut<Row> {
        self.rows.iter_mut()
    }

    /// Return a vector representing the maximum amount of characters in any line of this column.\
    /// This is mostly needed for internal testing and formatting, but can be interesting
    /// if you want to see the widths of the longest lines for each column.
    pub fn column_max_content_widths(&self) -> Vec<u16> {
        self.columns
            .iter()
            .map(|column| column.max_content_width)
            .collect()
    }

    pub(crate) fn style_or_default(&self, component: TableComponent) -> String {
        match self.style.get(&component) {
            None => " ".to_string(),
            Some(character) => character.to_string(),
        }
    }

    pub(crate) fn style_exists(&self, component: TableComponent) -> bool {
        self.style.get(&component).is_some()
    }

    /// Autogenerate new columns, if a row is added with more cells than existing columns.
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
}

/// This is an iterator over all cells of a specific column.
/// A dedicated struct is necessary, since data is usually handled by rows and thereby stored in
/// `Table::rows`. That's why this iterator also has to be implemented on the Table struct.
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

            // Check if the row has the requested column.
            if let Some(cell) = row.cells.get(self.column_index) {
                return Some(Some(cell));
            }

            return Some(None);
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

        println!("{}", table);
    }
}
