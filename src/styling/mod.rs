/// This module contains some styling presets for tables.
/// Each constant has an example for the respective preset.
pub mod presets;

/// This module contains some constants, which can be used to modify certain parts of a preset.
/// For instance, the [UTF8_ROUND_CORNERS](crate::styling::modifiers::UTF8_ROUND_CORNERS) modifies all corners to be round UTF8 box corners.
///
/// All constants in this module consist of strings, with each character in the same position as the respective component in the [Component](crate::styling::table::Component) enum.
/// Only non-whitespace characters are considered. You can use this, to write your own modifiers as well.
pub mod modifiers;

/// Everything about styling tables.
pub mod table;

pub mod cell;

pub mod column;
