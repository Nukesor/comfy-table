pub enum Constraint {
    /// Specify the exact percentage, this column should in respect to terminal width
    Percentage(u16),
    /// Specify a min amount of characters per line for a column
    MinWidth(u16),
    /// Specify a max amount of allowed characters for per line for a column
    MaxWidth(u16),
}


/// The Column struct mainly exists for styling purposes.
/// It's used to determine how much horizontal space each column should get and
/// allows users to manipulate this option.
/// On top of this, the column determines how much padding each cell should get.
pub struct Column {
    /// Left/right padding for each cell of this column in spaces
    padding: (u32, u32),
    constraint: Option<Constraint>,
}

impl Column {
    pub fn new() -> Self {
        Column {
            padding: (1, 1),
            constraint: None,
        }
    }

    /// Set the padding for all cells of this column
    /// Padding is provided in the form of (left, right).
    /// Default is (1, 1)
    pub fn set_padding(&mut self, padding: (u32, u32)) -> &mut Self {
        self.padding = padding;

        self
    }


    /// Set the constraint for this column.
    /// Adding a constraint allows to define some additional styling parameters for columns
    /// This can be useful to counter undesired auto-formatting of content in tables.
    pub fn set_constraint(&mut self, constraint: Constraint) -> &mut Self {
        self.constraint = Some(constraint);

        self
    }
}
