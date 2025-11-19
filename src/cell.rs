#[cfg(feature = "tty")]
use crate::{Attribute, Color};

use crate::style::CellAlignment;

/// A stylable table cell with content.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Cell {
    /// The content is a list of strings.\
    /// This is done to make working with newlines more easily.\
    /// When creating a new [Cell], the given content is split by newline.
    pub(crate) content: Vec<String>,
    /// The delimiter which is used to split the text into consistent pieces.\
    /// The default is ` `.
    pub(crate) delimiter: Option<char>,
    pub(crate) alignment: Option<CellAlignment>,
    #[cfg(feature = "tty")]
    pub(crate) fg: Option<Color>,
    #[cfg(feature = "tty")]
    pub(crate) bg: Option<Color>,
    #[cfg(feature = "tty")]
    pub(crate) attributes: Vec<Attribute>,
    /// Number of columns this cell spans (default: 1)
    pub(crate) colspan: Option<u16>,
    /// Number of rows this cell spans (default: 1)
    pub(crate) rowspan: Option<u16>,
}

impl Cell {
    /// Create a new Cell
    #[allow(clippy::needless_pass_by_value)]
    pub fn new<T: ToString>(content: T) -> Self {
        Self::new_owned(content.to_string())
    }

    /// Create a new Cell from an owned String
    pub fn new_owned(content: String) -> Self {
        #[cfg_attr(not(feature = "custom_styling"), allow(unused_mut))]
        let mut split_content: Vec<String> = content.split('\n').map(ToString::to_string).collect();

        // Correct ansi codes so style is terminated and resumed around the split
        #[cfg(feature = "custom_styling")]
        crate::utils::formatting::content_split::fix_style_in_split_str(&mut split_content);

        Self {
            content: split_content,
            delimiter: None,
            alignment: None,
            #[cfg(feature = "tty")]
            fg: None,
            #[cfg(feature = "tty")]
            bg: None,
            #[cfg(feature = "tty")]
            attributes: Vec::new(),
            colspan: None,
            rowspan: None,
        }
    }

    /// Return a copy of the content contained in this cell.
    pub fn content(&self) -> String {
        self.content.join("\n")
    }

    /// Set the delimiter used to split text for this cell. \
    /// Normal text uses spaces (` `) as delimiters. This is necessary to help comfy-table
    /// understand the concept of _words_.
    #[must_use]
    pub fn set_delimiter(mut self, delimiter: char) -> Self {
        self.delimiter = Some(delimiter);

        self
    }

    /// Set the alignment of content for this cell.
    ///
    /// Setting this overwrites alignment settings of the
    /// [Column](crate::column::Column::set_cell_alignment) for this specific cell.
    /// ```
    /// use comfy_table::CellAlignment;
    /// use comfy_table::Cell;
    ///
    /// let mut cell = Cell::new("Some content")
    ///     .set_alignment(CellAlignment::Center);
    /// ```
    #[must_use]
    pub fn set_alignment(mut self, alignment: CellAlignment) -> Self {
        self.alignment = Some(alignment);

        self
    }

    /// Set the foreground text color for this cell.
    ///
    /// Look at [Color](crate::Color) for a list of all possible Colors.
    /// ```
    /// use comfy_table::Color;
    /// use comfy_table::Cell;
    ///
    /// let mut cell = Cell::new("Some content")
    ///     .fg(Color::Red);
    /// ```
    #[cfg(feature = "tty")]
    #[must_use]
    pub fn fg(mut self, color: Color) -> Self {
        self.fg = Some(color);

        self
    }

    /// Set the background color for this cell.
    ///
    /// Look at [Color](crate::Color) for a list of all possible Colors.
    /// ```
    /// use comfy_table::Color;
    /// use comfy_table::Cell;
    ///
    /// let mut cell = Cell::new("Some content")
    ///     .bg(Color::Red);
    /// ```
    #[cfg(feature = "tty")]
    #[must_use]
    pub fn bg(mut self, color: Color) -> Self {
        self.bg = Some(color);

        self
    }

    /// Add a styling attribute to the content cell.\
    /// Those can be **bold**, _italic_, blinking and many more.
    ///
    /// Look at [Attribute](crate::Attribute) for a list of all possible Colors.
    /// ```
    /// use comfy_table::Attribute;
    /// use comfy_table::Cell;
    ///
    /// let mut cell = Cell::new("Some content")
    ///     .add_attribute(Attribute::Bold);
    /// ```
    #[cfg(feature = "tty")]
    #[must_use]
    pub fn add_attribute(mut self, attribute: Attribute) -> Self {
        self.attributes.push(attribute);

        self
    }

    /// Same as add_attribute, but you can pass a vector of [Attributes](Attribute)
    #[cfg(feature = "tty")]
    #[must_use]
    pub fn add_attributes(mut self, mut attribute: Vec<Attribute>) -> Self {
        self.attributes.append(&mut attribute);

        self
    }

    /// Set the number of columns this cell spans.
    ///
    /// By default, a cell spans 1 column. Setting a colspan greater than 1
    /// makes the cell occupy multiple columns. The cell's content will be
    /// rendered across all spanned columns, and borders between the spanned
    /// columns will be omitted.
    ///
    /// # Examples
    ///
    /// ```
    /// use comfy_table::{Cell, Table};
    ///
    /// let mut table = Table::new();
    /// table
    ///     .set_header(vec![
    ///         Cell::new("Header1").set_colspan(2),
    ///         Cell::new("Header3"),
    ///     ])
    ///     .add_row(vec![
    ///         Cell::new("Spans 2 columns").set_colspan(2),
    ///         Cell::new("Normal cell"),
    ///     ]);
    /// ```
    ///
    /// # Notes
    ///
    /// - When using colspan, you should add fewer cells to the row than the
    ///   number of columns. The spanned cell counts as multiple columns.
    /// - Colspan works with all table features including styling, alignment,
    ///   and dynamic width arrangement.
    /// - Hidden columns are automatically excluded from colspan calculations.
    #[must_use]
    pub fn set_colspan(mut self, cols: u16) -> Self {
        self.colspan = Some(cols);
        self
    }

    /// Set the number of rows this cell spans.
    ///
    /// By default, a cell spans 1 row. Setting a rowspan greater than 1
    /// makes the cell occupy multiple rows. The cell's content will appear
    /// only in the first row of the span, and subsequent rows will have
    /// empty space where the rowspan cell is located.
    ///
    /// # Examples
    ///
    /// ```
    /// use comfy_table::{Cell, Table};
    ///
    /// let mut table = Table::new();
    /// table
    ///     .set_header(vec!["Header1", "Header2", "Header3"])
    ///     .add_row(vec![
    ///         Cell::new("Spans 2 rows").set_rowspan(2),
    ///         Cell::new("Cell 2"),
    ///         Cell::new("Cell 3"),
    ///     ])
    ///     .add_row(vec![
    ///         // First position is occupied by rowspan above, so only add 2 cells
    ///         Cell::new("Cell 2 (row 2)"),
    ///         Cell::new("Cell 3 (row 2)"),
    ///     ]);
    /// ```
    ///
    /// # Notes
    ///
    /// - When using rowspan, subsequent rows should have fewer cells than
    ///   the number of columns, as the rowspan cell occupies space in those rows.
    /// - Rowspan content appears only in the starting row of the span.
    /// - Rowspan works with all table features including styling, alignment,
    ///   and multi-line content.
    /// - You can combine rowspan with colspan to create cells that span
    ///   both multiple rows and columns.
    #[must_use]
    pub fn set_rowspan(mut self, rows: u16) -> Self {
        self.rowspan = Some(rows);
        self
    }

    /// Get the number of columns this cell spans.
    ///
    /// Returns 1 if no colspan is set (default behavior).
    ///
    /// ```
    /// use comfy_table::Cell;
    ///
    /// let cell = Cell::new("Content");
    /// assert_eq!(cell.colspan(), 1);
    ///
    /// let cell = Cell::new("Content").set_colspan(3);
    /// assert_eq!(cell.colspan(), 3);
    /// ```
    pub fn colspan(&self) -> u16 {
        self.colspan.unwrap_or(1)
    }

    /// Get the number of rows this cell spans.
    ///
    /// Returns 1 if no rowspan is set (default behavior).
    ///
    /// ```
    /// use comfy_table::Cell;
    ///
    /// let cell = Cell::new("Content");
    /// assert_eq!(cell.rowspan(), 1);
    ///
    /// let cell = Cell::new("Content").set_rowspan(2);
    /// assert_eq!(cell.rowspan(), 2);
    /// ```
    pub fn rowspan(&self) -> u16 {
        self.rowspan.unwrap_or(1)
    }

    /// Alias for [set_colspan](Cell::set_colspan).
    ///
    /// ```
    /// use comfy_table::Cell;
    ///
    /// let cell = Cell::new("Spans 2 columns")
    ///     .span_columns(2);
    /// ```
    #[must_use]
    pub fn span_columns(self, cols: u16) -> Self {
        self.set_colspan(cols)
    }

    /// Alias for [set_rowspan](Cell::set_rowspan).
    ///
    /// ```
    /// use comfy_table::Cell;
    ///
    /// let cell = Cell::new("Spans 2 rows")
    ///     .span_rows(2);
    /// ```
    #[must_use]
    pub fn span_rows(self, rows: u16) -> Self {
        self.set_rowspan(rows)
    }
}

/// Convert anything with [ToString] to a new [Cell].
///
/// ```
/// # use comfy_table::Cell;
/// let cell: Cell = "content".into();
/// let cell: Cell = 5u32.into();
/// ```
impl<T: ToString> From<T> for Cell {
    fn from(content: T) -> Self {
        Self::new(content)
    }
}

/// A simple wrapper type for a `Vec<Cell>`.
///
/// This wrapper is needed to support generic conversions between iterables and `Vec<Cell>`.
/// Check the trait implementations for more docs.
pub struct Cells(pub Vec<Cell>);

/// Allow the conversion of a type to a [Cells], which is a simple vector of cells.
///
/// By default this is implemented for all Iterators over items implementing [ToString].
///
/// ```
/// use comfy_table::{Row, Cells};
///
/// let cells_string: Cells = vec!["One", "Two", "Three"].into();
/// let cells_integer: Cells = vec![1, 2, 3, 4].into();
/// ```
impl<T> From<T> for Cells
where
    T: IntoIterator,
    T::Item: Into<Cell>,
{
    fn from(cells: T) -> Self {
        Self(cells.into_iter().map(Into::into).collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_column_generation() {
        let content = "This is\nsome multiline\nstring".to_string();
        let cell = Cell::new(content.clone());

        assert_eq!(cell.content(), content);
    }
}
