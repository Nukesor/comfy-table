/// This module contains some styling presets for tables.
/// Every preset has an example. \
pub mod presets;

/// This module contains some constants, which can be used to modify certain parts of a preset.\
/// For instance, the [UTF8_ROUND_CORNERS](modifiers::UTF8_ROUND_CORNERS) modifies all corners to be round UTF8 box corners.
pub mod modifiers;

/// Everything about styling tables.
pub mod table;

/// Cell Alignment
pub mod cell;

pub mod column;

pub use cell::CellAlignment;
pub use table::{Component, ContentArrangement};
pub use column::Constraint;

/// Reexport for important crossterm enums
pub use ::crossterm::style::Attribute;
/// Reexport for important crossterm enums
pub use ::crossterm::style::Color;
