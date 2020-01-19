use crate::styling::cell::CellAlignment;
use crate::styling::column::Constraint;

/// The Column struct mainly exists for styling purposes.
/// It's used to determine how much horizontal space each column should get and
/// allows users to manipulate this option.
/// On top of this, the column determines how much padding each cell should get.
pub struct Column {
    /// The index of the column
    pub index: usize,
    /// Left/right padding for each cell of this column in spaces
    pub(crate) padding: (u16, u16),
    /// Define the cell alligment for all cells of this column
    pub(crate) cell_alignment: Option<CellAlignment>,
    pub(crate) max_content_width: u16,
    pub(crate) constraint: Option<Constraint>,
}

impl Column {
    pub fn new(index: usize) -> Self {
        Column {
            index: index,
            padding: (1, 1),
            constraint: None,
            max_content_width: 0,
            cell_alignment: None,
        }
    }

    /// Set the padding for all cells of this column
    /// Padding is provided in the form of (left, right).
    /// Default is (1, 1)
    pub fn set_padding(&mut self, padding: (u16, u16)) -> &mut Self {
        self.padding = padding;

        self
    }

    /// Get the width in characters of the widest line in this column.
    pub fn get_max_content_width(&self) -> u16 {
        self.max_content_width
    }

    /// Set the constraint for this column.
    /// Adding a constraint allows to define some additional styling parameters for columns
    /// This can be useful to counter undesired auto-formatting of content in tables.
    pub fn set_constraint(&mut self, constraint: Constraint) -> &mut Self {
        self.constraint = Some(constraint);

        self
    }

    /// Get the constraint that is used for this column.
    pub fn get_constraint(&mut self) -> Option<&Constraint> {
        self.constraint.as_ref()
    }

    /// Set the alignment for content inside of cells for this column
    /// If the alignment attribute is set on a cell in this column as well,
    /// The cell's alighment value will overwrite the column's setting.
    pub fn set_cell_alignment(&mut self, alignment: Option<CellAlignment>) {
        self.cell_alignment = alignment;
    }
}
