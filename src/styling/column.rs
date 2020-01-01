pub enum Constraint {
    /// Enforce a fix width for a column.
    Width(u16),
    /// Specify the exact percentage, this column should in respect to terminal width.
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
