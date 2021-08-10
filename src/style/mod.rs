/// This module provides styling presets for tables.\
/// Every preset has an example preview.
pub mod presets;

/// Contains modifiers, that can be used to alter certain parts of a preset.\
/// For instance, the [UTF8_ROUND_CORNERS](modifiers::UTF8_ROUND_CORNERS) replaces all corners with round UTF8 box corners.
pub mod modifiers;

mod table;

mod cell;

mod column;

pub use cell::CellAlignment;
pub use column::{ColumnConstraint, Width};
pub use table::{ContentArrangement, TableComponent};

/// Attributes used for styling cell content. Reexport of crossterm's [Attributes](crossterm::style::Attribute) enum.
#[cfg(feature = "tty")]
pub use crossterm::style::Attribute;
/// Colors used for styling cell content. Reexport of crossterm's [Color](crossterm::style::Color) enum.
#[cfg(feature = "tty")]
pub use crossterm::style::Color;
