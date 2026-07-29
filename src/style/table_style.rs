/// A styling rule for a type of horizontal line of a table.
///
/// This could be, for example, the top border or the lines between rows.
///
/// A styling rule consists of four optional parts:
///
/// ```text
/// ├╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌┤
/// ^     ^     ^           ^
/// left  fill  junction    right
/// ```
///
/// `left`/`right` are the border delimiters, `fill` is the horizontal delimiter between rows and
/// `junction` is drawn where vertical and horizontal lines cross in the middle
///
/// Styles that're wholly `None` won't be drawn.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct LineStyle {
    pub left: Option<char>,
    pub fill: Option<char>,
    pub junction: Option<char>,
    pub right: Option<char>,
}

impl LineStyle {
    /// Create a [LineStyle] with all four parts set.
    pub const fn new(left: char, fill: char, junction: char, right: char) -> Self {
        Self {
            left: Some(left),
            fill: Some(fill),
            junction: Some(junction),
            right: Some(right),
        }
    }

    /// Create a line without any parts set.
    pub const fn none() -> Self {
        Self {
            left: None,
            fill: None,
            junction: None,
            right: None,
        }
    }

    /// Set the left border character.
    pub const fn left(mut self, character: char) -> Self {
        self.left = Some(character);
        self
    }

    /// Set the fill character.
    pub const fn fill(mut self, character: char) -> Self {
        self.fill = Some(character);
        self
    }

    /// Set the junction character.
    pub const fn junction(mut self, character: char) -> Self {
        self.junction = Some(character);
        self
    }

    /// Set the right border character.
    pub const fn right(mut self, character: char) -> Self {
        self.right = Some(character);
        self
    }

    pub(crate) const fn is_visible(&self) -> bool {
        self.left.is_some()
            || self.fill.is_some()
            || self.junction.is_some()
            || self.right.is_some()
    }
}

/// A styling rule for the lines of a table that contain content.
///
/// This could be the header lines or the content lines of normal rows.
///
/// A styling rule consists of three optional parts:
///
/// ```text
/// │ a         ┆ b         │
/// ^           ^           ^
/// left        junction    right
/// ```
///
/// `left`/`right` are the border delimiters and `junction` is the vertical delimiter that's
/// drawn between columns.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct ContentLineStyle {
    pub left: Option<char>,
    pub junction: Option<char>,
    pub right: Option<char>,
}

impl ContentLineStyle {
    /// Create a [ContentLineStyle] with all three parts set.
    pub const fn new(left: char, junction: char, right: char) -> Self {
        Self {
            left: Some(left),
            junction: Some(junction),
            right: Some(right),
        }
    }

    /// Create a line without any parts set.
    pub const fn none() -> Self {
        Self {
            left: None,
            junction: None,
            right: None,
        }
    }

    /// Set the left border character.
    pub const fn left(mut self, character: char) -> Self {
        self.left = Some(character);
        self
    }

    /// Set the junction character.
    pub const fn junction(mut self, character: char) -> Self {
        self.junction = Some(character);
        self
    }

    /// Set the right border character.
    pub const fn right(mut self, character: char) -> Self {
        self.right = Some(character);
        self
    }
}

/// The full description of a table's look, built from four horizontal [LineStyle]s and two
/// [ContentLineStyle]s:
///
/// ```text
/// ┌─────────┬─────────┐   <- top_border
/// │ Hello   ┆ there   │   <- header_lines
/// ╞═════════╪═════════╡   <- header_separator
/// │ a       ┆ b       │   <- content_lines
/// ├╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┤   <- row_separator
/// │ c       ┆ d       │
/// └─────────┴─────────┘   <- bottom_border
/// ```
///
/// All functions for building a style are `const`, so custom styles can be declared as
/// constants, just like the ones in the [presets](crate::style::presets) module:
///
/// ```
/// use comfy_table::{ContentLineStyle, LineStyle, Table, TableStyle};
///
/// const MY_STYLE: TableStyle = TableStyle::new()
///     .top_border(LineStyle::new('┌', '─', '┬', '┐'))
///     .header_lines(ContentLineStyle::new('│', '┆', '│'))
///     .header_separator(LineStyle::new('╞', '═', '╪', '╡'))
///     .content_lines(ContentLineStyle::new('│', '┆', '│'))
///     .bottom_border(LineStyle::new('└', '─', '┴', '┘'));
///
/// let mut table = Table::new();
/// table.load_style(MY_STYLE);
/// ```
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct TableStyle {
    pub top_border: LineStyle,
    pub header_lines: ContentLineStyle,
    pub header_separator: LineStyle,
    pub content_lines: ContentLineStyle,
    pub row_separator: LineStyle,
    pub bottom_border: LineStyle,
}

impl TableStyle {
    /// Create a style that doesn't draw anything.
    pub const fn new() -> Self {
        Self {
            top_border: LineStyle::none(),
            header_lines: ContentLineStyle::none(),
            header_separator: LineStyle::none(),
            content_lines: ContentLineStyle::none(),
            row_separator: LineStyle::none(),
            bottom_border: LineStyle::none(),
        }
    }

    /// Set the top border of the table.
    pub const fn top_border(mut self, line: LineStyle) -> Self {
        self.top_border = line;
        self
    }

    /// Set the style of the lines that contain the header's content.
    pub const fn header_lines(mut self, line: ContentLineStyle) -> Self {
        self.header_lines = line;
        self
    }

    /// Set the line that's drawn between the header and the first row.
    pub const fn header_separator(mut self, line: LineStyle) -> Self {
        self.header_separator = line;
        self
    }

    /// Set the style of the lines that contain the rows' content.
    pub const fn content_lines(mut self, line: ContentLineStyle) -> Self {
        self.content_lines = line;
        self
    }

    /// Set the line that's drawn between rows.
    pub const fn row_separator(mut self, line: LineStyle) -> Self {
        self.row_separator = line;
        self
    }

    /// Set the bottom border of the table.
    pub const fn bottom_border(mut self, line: LineStyle) -> Self {
        self.bottom_border = line;
        self
    }

    /// Convert the outer corners to round corners.
    /// ```text
    /// ╭───────┬───────╮
    /// │ Hello │ there │
    /// ╞═══════╪═══════╡
    /// │ a     ┆ b     │
    /// ├╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌┤
    /// │ c     ┆ d     │
    /// ╰───────┴───────╯
    /// ```
    pub const fn with_rounded_corners(mut self) -> Self {
        self.top_border.left = Some('╭');
        self.top_border.right = Some('╮');
        self.bottom_border.left = Some('╰');
        self.bottom_border.right = Some('╯');
        self
    }

    /// Convert the inner borders to solid lines.
    /// ```text
    /// ┌───────┬───────┐
    /// │ Hello │ there │
    /// ╞═══════╪═══════╡
    /// │ a     │ b     │
    /// ├───────┼───────┤
    /// │ c     │ d     │
    /// └───────┴───────┘
    /// ```
    pub const fn with_solid_inner_borders(mut self) -> Self {
        self.header_lines.junction = Some('│');
        self.content_lines.junction = Some('│');
        self.row_separator.fill = Some('─');
        self
    }

    pub(crate) const fn has_top_border(&self) -> bool {
        self.top_border.is_visible()
    }

    pub(crate) const fn has_bottom_border(&self) -> bool {
        self.bottom_border.is_visible()
    }

    pub(crate) const fn has_header_separator(&self) -> bool {
        self.header_separator.is_visible()
    }

    pub(crate) const fn has_row_separator(&self) -> bool {
        self.row_separator.is_visible()
    }

    /// The left border is drawn as soon as any component in the leftmost column exists.
    pub(crate) const fn has_left_border(&self) -> bool {
        self.header_lines.left.is_some()
            || self.content_lines.left.is_some()
            || self.top_border.left.is_some()
            || self.header_separator.left.is_some()
            || self.row_separator.left.is_some()
            || self.bottom_border.left.is_some()
    }

    /// The right border is drawn as soon as any component in the rightmost column exists.
    pub(crate) const fn has_right_border(&self) -> bool {
        self.header_lines.right.is_some()
            || self.content_lines.right.is_some()
            || self.top_border.right.is_some()
            || self.header_separator.right.is_some()
            || self.row_separator.right.is_some()
            || self.bottom_border.right.is_some()
    }

    /// Vertical lines are drawn as soon as any component between two columns exists.
    pub(crate) const fn has_vertical_lines(&self) -> bool {
        self.header_lines.junction.is_some()
            || self.content_lines.junction.is_some()
            || self.top_border.junction.is_some()
            || self.header_separator.junction.is_some()
            || self.row_separator.junction.is_some()
            || self.bottom_border.junction.is_some()
    }
}
