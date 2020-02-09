use ::std::fmt;
use ::crossterm::terminal::size;
use ::std::collections::HashMap;
use ::std::iter::IntoIterator;
use ::std::slice::{Iter, IterMut};
use ::strum::IntoEnumIterator;

use crate::column::Column;
use crate::row::{Row, ToRow};
use crate::style::presets::ASCII_FULL;
use crate::style::{ColumnConstraint, ContentArrangement, TableComponent};
use crate::utils::arrangement::arrange_content;
use crate::utils::borders::draw_borders;
use crate::utils::format::format_content;

/// A table containing rows of cells.
#[derive(Debug)]
pub struct Table {
    pub(crate) columns: Vec<Column>,
    style: HashMap<TableComponent, char>,
    pub(crate) header: Option<Row>,
    pub(crate) rows: Vec<Row>,
    pub(crate) arrangement: ContentArrangement,
    no_tty: bool,
    table_width: Option<u16>,
    enforce_styling: bool,
}

impl fmt::Display for Table {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let display_info = arrange_content(self);
        let content = format_content(&self, &display_info);
        let lines = draw_borders(&self, content, &display_info);

        write!(f, "{}", lines.join("\n"))
    }
}

impl Table {
    /// Create a new table with default ASCII styling, no rows and a header
    pub fn new() -> Self {
        let mut table = Table {
            columns: Vec::new(),
            header: None,
            rows: Vec::new(),
            arrangement: ContentArrangement::Disabled,
            no_tty: false,
            table_width: None,
            style: HashMap::new(),
            enforce_styling: false,
        };

        table.load_preset(ASCII_FULL);

        table
    }

    /// Set the header row of the table. This is usually the title of each column.
    /// ```
    /// use comfy_table::{Table, Row};
    ///
    /// let mut table = Table::new();
    /// let header = Row::from(vec!["Header One", "Header Two"]);
    /// table.set_header(header);
    /// ```

    pub fn set_header<T: ToRow>(&mut self, row: T) -> &mut Self {
        let row = row.to_row();
        self.autogenerate_columns(&row);
        self.adjust_max_column_widths(&row);
        self.header = Some(row);

        self
    }

    pub fn get_header(&self) -> Option<&Row> {
        self.header.as_ref()
    }

    /// Add a new row to the table.
    /// ```
    /// use comfy_table::{Table, Row};
    ///
    /// let mut table = Table::new();
    /// let row = Row::from(vec!["One", "Two"]);
    /// table.add_row(row);
    /// ```
    pub fn add_row<T: ToRow>(&mut self, row: T) -> &mut Self {
        let mut row = row.to_row();
        self.autogenerate_columns(&row);
        self.adjust_max_column_widths(&row);
        row.index = Some(self.rows.len());
        self.rows.push(row);

        self
    }
    /// Enforce a max width that should be used in combination with [dynamic content arrangement](ContentArrangement::Dynamic).
    /// This is usually not necessary, if you plan to output your table to a tty, since the
    /// terminal width can be automatically determined.
    pub fn set_table_width(&mut self, table_width: u16) -> &mut Self {
        self.table_width = Some(table_width);

        self
    }

    /// Get the expected width of the table.
    ///
    /// This will be `Some(width)`, if the terminal width can be detected or if the table width is set via [set_table_width](Table::set_table_width).
    ///
    /// If neither is not possible, `None` will be returned.\
    /// This implies that both [Dynamic](ContentArrangement::Dynamic) and [Percentage](crate::style::ColumnConstraint::Percentage) won't work.
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

    /// Specify how comfy_table should arrange the content in your table.
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

    /// In case you are sure you don't want export tables to a tty
    /// or you experience problems with tty checking code, you can
    /// enforce a non_tty mode.
    ///
    /// This disables:
    ///
    /// - table_width lookup from the current tty
    /// - Styling and attributes on cells (unless you [Table::enforce_styling])
    ///
    /// If you use the [dynamic content arrangement](ContentArrangement::Dynamic),
    /// you need to set the width of your desired table manually with [set_table_width](Table::set_table_width).
    pub fn force_no_tty(&mut self) -> &mut Self {
        self.no_tty = true;

        self
    }

    /// Returns whether the table will be handled as if it's printed to a tty.
    /// This function respects the [Table::force_no_tty] and [Table::force_tty] functions.
    /// Otherwise we try to determine, if we are on a tty.
    pub fn is_tty(&self) -> bool {
        if self.no_tty {
            return false;
        }

        atty::is(atty::Stream::Stdout)
    }

    /// Enforce terminal styling. Only useful if you forcefully disabled tty,
    /// but still want those fancy terminal styles.
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

    /// Enforce terminal styling. Only useful if you forcefully disabled tty,
    /// but still want those fancy terminal styles.
    pub fn should_style(&self) -> bool {
        if self.enforce_styling {
            return true;
        }
        self.is_tty()
    }

    /// Convenience method to set a [ColumnConstraint] for all columns at once.
    /// Simply pass any iterable with ColumnConstraints.
    /// If more Constraints are passed than there are Columns, these Constraints will be ignored
    /// ```
    /// use comfy_table::{Table, ColumnConstraint, ContentArrangement};
    /// let mut table = Table::new();
    /// table.add_row(&vec!["one", "two", "three"])
    ///     .set_content_arrangement(ContentArrangement::Dynamic)
    ///     .set_constraints(vec![
    ///         ColumnConstraint::MaxWidth(15),
    ///         ColumnConstraint::MinWidth(20),
    ///         ColumnConstraint::Width(40),
    /// ]);
    /// ```
    pub fn set_constraints<T: IntoIterator<Item = ColumnConstraint>>(
        &mut self,
        constraints: T,
    ) -> &mut Self {
        let mut constraints = constraints.into_iter();
        let mut column_iter = self.column_iter_mut();
        while let Some(column) = column_iter.next() {
            if let Some(constraint) = constraints.next() {
                column.set_constraint(constraint);
            } else {
                break;
            }
        }

        self
    }

    /// This function creates a TableStyle from a given preset string.
    /// Preset strings can be found in styling::presets::*
    ///
    /// Anyway, you can write your own preset strings and use them with this function.
    /// The function expects a characters for components to be in the same order as in the [TableComponent] enum.
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

    /// Modify a preset with a modifier string from [modifiers](crate::style::modifiers).
    /// For instance, the [UTF8_ROUND_CORNERS](crate::style::modifiers::UTF8_ROUND_CORNERS) modifies all corners to be round UTF8 box corners.
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

    /// Define the char that will be used to draw a specific component
    /// Look at [TableComponent] to see all stylable components
    ///
    /// If `None` is supplied, the element won't be displayed.
    /// In case of a e.g. *BorderIntersection a whitespace will be used as placeholder,
    /// unless related borders and and corners are set to `None` as well.
    ///
    /// For example, if `TopBorderIntersections` is `None` the first row would look like this:
    /// ```text
    /// +------ ------+
    /// | asdf | ghij |
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

    /// Get a copy of the char that's currently used for drawing this component
    /// ```
    /// use comfy_table::Table;
    /// use comfy_table::TableComponent::*;
    ///
    /// let mut table = Table::new();
    /// assert_eq!(table.get_style(TopLeftCorner), Some('+'));
    /// ```

    pub fn get_style(&mut self, component: TableComponent) -> Option<char> {
        match self.style.get(&component) {
            None => None,
            Some(character) => Some(*character),
        }
    }

    /// Remove the style for a specific component of the table.\
    /// By default, a space will be used as a placeholder instead.\
    /// Though, if for instance all components of the left border are removed, the left border won't be displayed.
    pub fn remove_style(&mut self, component: TableComponent) -> &mut Self {
        self.style.remove(&component);

        self
    }

    /// Reference to a specific column
    pub fn get_column(&self, index: usize) -> Option<&Column> {
        self.columns.get(index)
    }

    /// Mutable reference to a specific column
    pub fn get_column_mut(&mut self, index: usize) -> Option<&mut Column> {
        self.columns.get_mut(index)
    }

    /// Iterator over all columns
    pub fn column_iter(&mut self) -> Iter<Column> {
        self.columns.iter()
    }

    /// Mutable iterator over all columns
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

    /// Return a vector representing the maximum amount of characters in any line of this column. \
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
