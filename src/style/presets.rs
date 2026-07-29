use super::{ContentLineStyle, LineStyle, TableStyle};

/// The default style for tables.
///
/// ```text
/// +-------+-------+
/// | Hello | there |
/// +===============+
/// | a     | b     |
/// |-------+-------|
/// | c     | d     |
/// +-------+-------+
/// ```
pub const ASCII_FULL: TableStyle = TableStyle::new()
    .top_border(LineStyle::new('+', '-', '+', '+'))
    .header_lines(ContentLineStyle::new('|', '|', '|'))
    .header_separator(LineStyle::new('+', '=', '=', '+'))
    .content_lines(ContentLineStyle::new('|', '|', '|'))
    .row_separator(LineStyle::new('|', '-', '+', '|'))
    .bottom_border(LineStyle::new('+', '-', '+', '+'));

/// Just like ASCII_FULL, but without dividers between rows.
///
/// ```text
/// +-------+-------+
/// | Hello | there |
/// +===============+
/// | a     | b     |
/// | c     | d     |
/// +-------+-------+
pub const ASCII_FULL_CONDENSED: TableStyle = TableStyle::new()
    .top_border(LineStyle::new('+', '-', '+', '+'))
    .header_lines(ContentLineStyle::new('|', '|', '|'))
    .header_separator(LineStyle::new('+', '=', '=', '+'))
    .content_lines(ContentLineStyle::new('|', '|', '|'))
    .bottom_border(LineStyle::new('+', '-', '+', '+'));

/// Just like ASCII_FULL, but without any borders.
///
/// ```text
///  Hello | there
/// ===============
///  a     | b
/// -------+-------
///  c     | d
/// ```
pub const ASCII_NO_BORDERS: TableStyle = TableStyle::new()
    .header_lines(ContentLineStyle::none().junction('|'))
    .header_separator(LineStyle::none().fill('=').junction('='))
    .content_lines(ContentLineStyle::none().junction('|'))
    .row_separator(LineStyle::none().fill('-').junction('+'));

/// Just like ASCII_FULL, but without vertical/horizontal middle lines.
///
/// ```text
/// +---------------+
/// | Hello   there |
/// +===============+
/// | a       b     |
/// |               |
/// | c       d     |
/// +---------------+
/// ```
pub const ASCII_BORDERS_ONLY: TableStyle = TableStyle::new()
    .top_border(LineStyle::new('+', '-', '-', '+'))
    .header_lines(ContentLineStyle::none().left('|').right('|'))
    .header_separator(LineStyle::new('+', '=', '=', '+'))
    .content_lines(ContentLineStyle::none().left('|').right('|'))
    .row_separator(LineStyle::none().left('|').right('|'))
    .bottom_border(LineStyle::new('+', '-', '-', '+'));

/// Just like ASCII_BORDERS_ONLY, but without spacing between rows.
///
/// ```text
/// +---------------+
/// | Hello   there |
/// +===============+
/// | a       b     |
/// | c       d     |
/// +---------------+
/// ```
pub const ASCII_BORDERS_ONLY_CONDENSED: TableStyle = TableStyle::new()
    .top_border(LineStyle::new('+', '-', '-', '+'))
    .header_lines(ContentLineStyle::none().left('|').right('|'))
    .header_separator(LineStyle::new('+', '=', '=', '+'))
    .content_lines(ContentLineStyle::none().left('|').right('|'))
    .bottom_border(LineStyle::new('+', '-', '-', '+'));

/// Just like ASCII_FULL, but without vertical/horizontal middle lines and no side borders.
///
/// ```text
/// ---------------
///  Hello   there
/// ===============
///  a       b
/// ---------------
///  c       d
/// ---------------
/// ```
pub const ASCII_HORIZONTAL_ONLY: TableStyle = TableStyle::new()
    .top_border(LineStyle::none().fill('-').junction('-'))
    .header_separator(LineStyle::none().fill('=').junction('='))
    .row_separator(LineStyle::none().fill('-').junction('-'))
    .bottom_border(LineStyle::none().fill('-').junction('-'));

/// Markdown like table styles.
///
/// ```text
/// | Hello | there |
/// |-------|-------|
/// | a     | b     |
/// | c     | d     |
/// ```
pub const ASCII_MARKDOWN: TableStyle = TableStyle::new()
    .header_lines(ContentLineStyle::new('|', '|', '|'))
    .header_separator(LineStyle::new('|', '-', '|', '|'))
    .content_lines(ContentLineStyle::new('|', '|', '|'));

/// The UTF8 enabled version of the default style for tables.\
/// Quite beautiful isn't it? It's drawn with UTF8's box drawing characters.
///
/// ```text
/// ┌───────┬───────┐
/// │ Hello ┆ there │
/// ╞═══════╪═══════╡
/// │ a     ┆ b     │
/// ├╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌┤
/// │ c     ┆ d     │
/// └───────┴───────┘
/// ```
pub const UTF8_FULL: TableStyle = TableStyle::new()
    .top_border(LineStyle::new('┌', '─', '┬', '┐'))
    .header_lines(ContentLineStyle::new('│', '┆', '│'))
    .header_separator(LineStyle::new('╞', '═', '╪', '╡'))
    .content_lines(ContentLineStyle::new('│', '┆', '│'))
    .row_separator(LineStyle::new('├', '╌', '┼', '┤'))
    .bottom_border(LineStyle::new('└', '─', '┴', '┘'));

/// Default UTF8 style, but without dividers between rows.
///
/// ```text
/// ┌───────┬───────┐
/// │ Hello ┆ there │
/// ╞═══════╪═══════╡
/// │ a     ┆ b     │
/// │ c     ┆ d     │
/// └───────┴───────┘
/// ```
pub const UTF8_FULL_CONDENSED: TableStyle = TableStyle::new()
    .top_border(LineStyle::new('┌', '─', '┬', '┐'))
    .header_lines(ContentLineStyle::new('│', '┆', '│'))
    .header_separator(LineStyle::new('╞', '═', '╪', '╡'))
    .content_lines(ContentLineStyle::new('│', '┆', '│'))
    .bottom_border(LineStyle::new('└', '─', '┴', '┘'));

/// Default UTF8 style, but without any borders.
///
/// ```text
///  Hello ┆ there
/// ═══════╪═══════
///  a     ┆ b
/// ╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌
///  c     ┆ d
/// ```
pub const UTF8_NO_BORDERS: TableStyle = TableStyle::new()
    .header_lines(ContentLineStyle::none().junction('┆'))
    .header_separator(LineStyle::none().fill('═').junction('╪'))
    .content_lines(ContentLineStyle::none().junction('┆'))
    .row_separator(LineStyle::none().fill('╌').junction('┼'));

/// Just like the UTF8_FULL style, but without vertical/horizontal middle lines.
///
/// ```text
/// ┌───────────────┐
/// │ Hello   there │
/// ╞═══════════════╡
/// │ a       b     │
/// │ c       d     │
/// └───────────────┘
/// ```
pub const UTF8_BORDERS_ONLY: TableStyle = TableStyle::new()
    .top_border(LineStyle::new('┌', '─', '─', '┐'))
    .header_lines(ContentLineStyle::none().left('│').right('│'))
    .header_separator(LineStyle::new('╞', '═', '═', '╡'))
    .content_lines(ContentLineStyle::none().left('│').right('│'))
    .bottom_border(LineStyle::new('└', '─', '─', '┘'));

/// Only display vertical lines.
///
/// ```text
/// ───────────────
///  Hello   there
/// ═══════════════
///  a       b
/// ───────────────
///  c       d
/// ───────────────
/// ```
pub const UTF8_HORIZONTAL_ONLY: TableStyle = TableStyle::new()
    .top_border(LineStyle::none().fill('─').junction('─'))
    .header_separator(LineStyle::none().fill('═').junction('═'))
    .row_separator(LineStyle::none().fill('─').junction('─'))
    .bottom_border(LineStyle::none().fill('─').junction('─'));

/// Don't draw any borders or other lines.
/// Useful, if you want to simply organize some data without any cosmetics.
///
/// ```text
///  Hello  there
///  a      b
///  c      d
/// ```
pub const NOTHING: TableStyle = TableStyle::new();
