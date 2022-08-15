/// A Constraint can be added to a [columns](crate::Column).
///
/// They allow some control over Column widths as well as the dynamic arrangement process.
///
/// All percental boundaries will be ignored, if:
/// - you aren't using one of ContentArrangement::{Dynamic, DynamicFullWidth}
/// - the width of the table/terminal cannot be determined.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ColumnConstraint {
    /// This will completely hide a column.
    Hidden,
    /// Force the column to be as long as it's content.
    /// Use with caution! This can easily break your table, if the column's content is overly long.
    ContentWidth,
    /// Enforce a absolute width for a column.
    Absolute(Width),
    /// Specify a lower boundary, either fixed or as percentage of the total width.
    LowerBoundary(Width),
    /// Specify a upper boundary, either fixed or as percentage of the total width.
    UpperBoundary(Width),
    /// Specify both, an upper and a lower boundary.
    Boundaries { lower: Width, upper: Width },
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Width {
    /// A fixed amount of characters.
    /// This can be used to specify an upper/lower boundary as well as a fixed size for the column.
    Fixed(u16),
    /// A width equivalent to a certain percentage of the available width.
    /// Values above 100 will be automatically reduced to 100.
    ///
    /// This can be used to specify an upper/lower boundary as well as a fixed size for the column.
    ///
    /// **Warning:** This option will be ignored if:
    /// - you aren't using one of ContentArrangement::{Dynamic, DynamicFullWidth}
    /// - the width of the table/terminal cannot be determined.
    Percentage(u16),
}
