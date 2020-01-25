use ::std::collections::HashMap;
use ::strum::IntoEnumIterator;

use crate::column::Column;
use crate::row::{Row, ToRow};
use crate::style::table::{ContentArrangement, Component};
use crate::utils::arrangement::arrange_content;
use crate::utils::borders::draw_borders;
use crate::utils::format::format_content;
use crate::style::presets::ASCII_FULL;

/// The representation of a table.
pub struct Table {
    pub(crate) columns: Vec<Column>,
    style: HashMap<Component, char>,
    pub(crate) header: Option<Row>,
    pub(crate) rows: Vec<Row>,
    pub(crate) arrangement: ContentArrangement,
    tty: Option<bool>,
    max_content_width: Option<u16>,
}

impl ToString for Table {
    fn to_string(&self) -> String {
        let display_info = arrange_content(self);
        let content = format_content(&self, &display_info);
        let lines = draw_borders(&self, content, &display_info);

        lines.join("\n")
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
            tty: None,
            max_content_width: None,
            style: HashMap::new(),
        };

        table.load_preset(ASCII_FULL);

        table
    }

    /// Set the header row of the table. This is usually the title of each column.
    pub fn set_header<T: ToRow>(&mut self, row: T) -> &mut Self {
        let row = row.to_row();
        self.autogenerate_columns(&row);
        self.adjust_max_column_widths(&row);
        self.header = Some(row);

        self
    }

    /// Add a new row to the table.
    pub fn add_row<T: ToRow>(&mut self, row: T) -> &mut Self {
        let mut row = row.to_row();
        self.autogenerate_columns(&row);
        self.adjust_max_column_widths(&row);
        row.index = Some(self.rows.len());
        self.rows.push(row);

        self
    }

    pub fn get_header(&self) -> Option<&Row> {
        self.header.as_ref()
    }

    /// Force formatting output as if
    /// I don't expect
    pub fn force_tty(&mut self) -> &mut Self {
        self.tty = Some(true);

        self
    }

    /// In case you are sure you don't want export tables to a tty
    /// or you experience problems with tty checking code, you can
    /// enforce a non_tty mode.
    ///
    /// This disables:
    ///
    /// - Automatic max_width lookup from the current tty
    /// - Styling and attributes on cells
    ///
    /// If you use the dynamic content arrangement mode (ContentArrangement::Dynamic),
    /// you need to set the width of your desired table manually with [Table::set_max_width].
    pub fn force_no_tty(&mut self) -> &mut Self {
        self.tty = Some(false);

        self
    }

    /// Returns whether the table will be handled as if it's printed to a tty.
    /// This function respects the [Table::force_no_tty] and [Table::force_tty] functions.
    /// Otherwise we try to automatically determine, if we are on a tty.
    pub fn is_tty(&self) -> bool {
        match self.tty {
            Some(is_tty) => is_tty,
            None => {
                atty::is(atty::Stream::Stdout)
            }
        }
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
        self.columns
            .iter()
            .map(|column| column.max_content_width)
            .collect()
    }

    /// This function creates a TableStyle from a given preset string.
    /// Preset strings can be found in styling::presets::*
    ///
    /// Anyway, you can write your own preset strings and use them with this function.
    /// The function expects a characters for components to be in the same order as in the [Component] enum.
    ///
    /// If the string isn't long enough, the default [ASCII_FULL] style will be used for all remaining components.
    ///
    /// If the string is too long, remaining charaacters will be simply ignored.
    pub fn load_preset(&mut self, preset: &str) {
        let mut components = Component::iter();

        for character in preset.chars() {
            if let Some(component) = components.next() {
                // White spaces mean "don't draw this" in presets
                // If we want to override the default preset, we need to remove
                // this component from the HashMap in case we find a whitespace.
                if character == ' ' {
                    self.style.remove(&component);
                    continue;
                }

                self.style.insert(component, character);
            } else {
                break;
            }
        }
    }

    /// Modify a preset with a modifier string from [modifiers](crate::style::modifiers).
    /// For instance, the [UTF8_ROUND_CORNERS](crate::style::modifiers::UTF8_ROUND_CORNERS) modifies all corners to be round UTF8 box corners.
    pub fn apply_modifier(&mut self, modifier: &str) -> &mut Self {
        let mut components = Component::iter();

        for character in modifier.chars() {
            // Skip spaces while applying modifiers.
            if character == ' ' {
                continue;
            }
            if let Some(component) = components.next() {
                self.style.insert(component, character);
            } else {
                break;
            }
        }

        self
    }

    /// Define the char that will be used to draw a specific component
    /// Look at [Component] to see all stylable Components
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
    pub fn set_style(&mut self, component: Component, character: Option<char>) -> &mut Self {
        match character {
            Some(character) => {
                self.style.insert(component, character);
            }
            None => (),
        };

        self
    }

    /// Get a copy of the char currently used for drawing a specific component
    pub fn get_style(&mut self, component: Component) -> Option<char> {
        match self.style.get(&component) {
            None => None,
            Some(character) => Some(*character),
        }
    }

    pub (crate)fn style_or_default(&self, component: Component) -> String {
        match self.style.get(&component) {
            None => " ".to_string(),
            Some(character) => character.to_string(),
        }
    }

    pub (crate)fn style_exists(&self, component: Component) -> bool {
        self.style.get(&component).is_some()
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
