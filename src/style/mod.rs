#[cfg(all(feature = "tty", not(feature = "reexport_crossterm")))]
mod attribute;
mod cell;
#[cfg(all(feature = "tty", not(feature = "reexport_crossterm")))]
mod color;
mod column;
/// This module provides styling presets for tables.\
/// Every preset has an example preview.
pub mod presets;
mod table;
mod table_style;

pub use cell::CellAlignment;
pub use column::{ColumnConstraint, Width};
#[cfg(feature = "tty")]
pub use styling_enums::{Attribute, Color};
#[cfg(feature = "tty")]
pub(crate) use styling_enums::{map_attribute, map_color};
pub use table::ContentArrangement;
pub use table_style::{ContentLineStyle, LineStyle, TableStyle};

/// Convenience module to have cleaner and "identical" conditional re-exports for style enums.
#[cfg(all(feature = "tty", not(feature = "reexport_crossterm")))]
mod styling_enums {
    pub use super::{attribute::*, color::*};
}

/// Re-export the crossterm type directly instead of using the internal mirrored types.
/// This result in possible ABI incompatibilities when using comfy_table and crossterm in the same
/// project with different versions, but may also be very convenient for developers.
#[cfg(all(feature = "tty", feature = "reexport_crossterm"))]
mod styling_enums {
    /// Attributes used for styling cell content. Reexport of crossterm's
    /// [Attributes](crossterm::style::Attribute) enum.
    pub use crossterm::style::Attribute;
    /// Colors used for styling cell content. Reexport of crossterm's
    /// [Color](crossterm::style::Color) enum.
    pub use crossterm::style::Color;

    /// Convenience function to have the same mapping code for reexported types.
    #[inline]
    pub(crate) fn map_attribute(attribute: Attribute) -> Attribute {
        attribute
    }

    /// Convenience function to have the same mapping code for reexported types.
    #[inline]
    pub(crate) fn map_color(color: Color) -> Color {
        color
    }
}
