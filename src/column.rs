use crate::style::cell::CellAlignment;
use crate::style::column::Constraint;

/// The Column struct exists for styling purposes:
///
/// 1. Content padding for cells in this column
/// 2. Constraints on how wide this column shall be
/// 3. Default alignment for cells in this column
///
/// Columns are automatically generated when adding rows or a header to a table.\
/// As a result columns can only be modified after the table is populated by some data.
///
/// ```
/// use comfy_table::{Table, Constraint, CellAlignment};
///
/// let mut table = Table::new();
/// table.set_header(&vec!["one", "two"]);
///
/// let mut column = table.get_column_mut(1).expect("This should be column two");
///
/// // Set the max width for all cells of this column to 20 characters.
/// column.set_constraint(Constraint::MaxWidth(20));
///
/// // Set the left padding to 5 spaces and the right padding to 1 space
/// column.set_padding((5, 1));
///
/// // Align content in all cells of this column to the center of the cell.
/// column.set_cell_alignment(CellAlignment::Center);
/// ```

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

    /// Set the padding for all cells of this column.\
    /// Padding is provided in the form of (left, right).\
    /// Default is `(1, 1)`.
    pub fn set_padding(&mut self, padding: (u16, u16)) -> &mut Self {
        self.padding = padding;

        self
    }

    /// Get the width in characters of the widest line in this column.
    pub fn get_max_content_width(&self) -> u16 {
        self.max_content_width
    }

    /// Set the constraint for this column. \
    /// Constraints allow to influence the auto-adjustment behavior of columns. \
    /// This can be useful to counter undesired auto-adjustment of content in tables.
    pub fn set_constraint(&mut self, constraint: Constraint) -> &mut Self {
        self.constraint = Some(constraint);

        self
    }

    /// Get the constraint that is used for this column.
    pub fn get_constraint(&mut self) -> Option<&Constraint> {
        self.constraint.as_ref()
    }

    /// Set the alignment for content inside of cells for this column. \
    /// **Note:** Alignment on a cell will always overwrite the column's setting.
    pub fn set_cell_alignment(&mut self, alignment: CellAlignment) {
        self.cell_alignment = Some(alignment);
    }
}
