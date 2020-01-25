/// Constraints can be added to (columns)[crate::Column].\
/// They allow some control over the automatic content arrangement process. \
pub enum ColumnConstraint {
    /// Enforce a fix width for a column.
    Width(u16),
    /// Specify the exact percentage, this column should in respect terminal width or
    /// the fix value set with [crate::table::Table::set_table_width].
    /// **Warning:** This option will be ignored, if the width cannot be determined!
    Percentage(u16),
    /// Specify a min amount of characters per line for a column.
    MinWidth(u16),
    /// Specify a max amount of allowed characters for per line for a column.
    MaxWidth(u16),
    /// Force the column to be as long as it's content.
    /// Use with caution! This can easily break your table, if the column's content is overly long.
    ContentWidth,
    /// Hide this Column. It won't be displayed at all.
    Hidden,
}
