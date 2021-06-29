/// A Constraint can be added to a [columns](crate::Column).
///
/// They allow some control over Column widths as well as the dynamic arrangement process.
///
/// All percental boundaries will be ignored, if:
/// - the terminal width cannot be determined.
/// - you aren't doing dynamic content arrangement.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ColumnConstraint {
    /// This will completely hide a column.
    Hidden,
    /// Force the column to be as long as it's content.
    /// Use with caution! This can easily break your table, if the column's content is overly long.
    ContentWidth,
    /// Enforce a absolute width for a column.
    Absolute(Boundary),
    /// Specify a lower boundary, either fixed or as percentage of the total width.
    LowerBoundary(Boundary),
    /// Specify a upper boundary, either fixed or as percentage of the total width.
    UpperBoundary(Boundary),
    /// Specify both, an upper and a lower boundary.
    Boundaries { lower: Boundary, upper: Boundary },
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Boundary {
    /// Specify a min amount of characters per line for a column.
    Fixed(u16),
    /// Set a a minimum percentage in respect to table_width for this column.
    /// Values above 100 will be automatically reduced to 100.
    /// **Warning:** This option will be ignored, if the width cannot be determined!
    Percentage(u16),
}
