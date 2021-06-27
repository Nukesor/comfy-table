pub mod arrangement;
pub mod borders;
pub mod format;
mod split;

use crate::column::Column;
use crate::style::{CellAlignment, ColumnConstraint};

/// This struct is ONLY used when table.to_string() is called.
/// It's purpose is to store intermediate results, information on how to
/// arrange the table and other convenience variables.
///
/// The idea is to have a place for all this intermediate stuff, without
/// actually touching the Column struct.
#[derive(Debug)]
pub struct ColumnDisplayInfo {
    pub padding: (u16, u16),
    pub delimiter: Option<char>,
    /// The actual allowed content width after arrangement
    pub content_width: Option<u16>,
    /// The max amount of characters over all lines in this column
    pub max_content_width: u16,
    /// A constraint that should be considered during dynamic arrangement
    pub constraint: Option<ColumnConstraint>,
    /// The content alignment of cells in this column
    pub cell_alignment: Option<CellAlignment>,
}

impl ColumnDisplayInfo {
    pub fn new(column: &Column) -> Self {
        ColumnDisplayInfo {
            padding: column.padding,
            delimiter: column.delimiter,
            max_content_width: column.max_content_width,
            content_width: None,
            constraint: None::<ColumnConstraint>,
            cell_alignment: column.cell_alignment,
        }
    }
    pub fn padding_width(&self) -> u16 {
        self.padding.0 + self.padding.1
    }

    pub fn content_width(&self) -> Option<u16> {
        self.content_width
    }

    pub fn set_content_width(&mut self, mut width: u16) {
        // Don't allow content widths of 0.
        if width == 0 {
            width = 1;
        }
        self.content_width = Some(width);
    }

    pub fn max_width(&self) -> u16 {
        self.max_content_width + self.padding.0 + self.padding.1
    }

    pub fn width(&self) -> Option<u16> {
        match self.content_width {
            None => None,
            Some(width) => Some(width + self.padding.0 + self.padding.1),
        }
    }

    pub fn is_hidden(&self) -> bool {
        matches!(self.constraint, Some(ColumnConstraint::Hidden))
    }

    /// Return the remaining value after subtracting the padding width.
    /// The minimum content has to be always 1.
    pub fn without_padding(&self, width: u16) -> u16 {
        let padding = self.padding_width();
        // Default minimum content width has to be 1
        if padding >= width {
            return 1;
        }

        width - padding
    }
}
