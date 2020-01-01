pub enum Constraint {
    /// Enforce a fix width for a column.
    Width(u16),
    /// Specify the exact percentage, this column should in respect to terminal width.
    Percentage(u16),
    /// Specify a min amount of characters per line for a column.
    MinWidth(u16),
    /// Specify a max amount of allowed characters for per line for a column.
    MaxWidth(u16),
    /// Force the column to be as long as it's content.
    /// Use with caution! This can easily break your table, if the column's content is overly long.
    ContentWidth,
    /// Hide this Column. It won't be displayed at all.
    Hidden,
}

/// The Column struct mainly exists for styling purposes.
/// It's used to determine how much horizontal space each column should get and
/// allows users to manipulate this option.
/// On top of this, the column determines how much padding each cell should get.
pub struct Column {
    /// The index of the column
    pub index: usize,
    /// Left/right padding for each cell of this column in spaces
    padding: (u32, u32),
    pub (crate) constraint: Option<Constraint>,
    pub (crate) max_content_width: u16,
}

impl Column {
    pub fn new(index: usize) -> Self {
        Column {
            index: index,
            padding: (1, 1),
            constraint: None,
            max_content_width: 1,
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

    /// Get the constraint that is used for this column.
    pub fn get_constraint(&mut self) -> Option<&Constraint> {
        self.constraint.as_ref()
    }
}
