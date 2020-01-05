use crate::styling::cell::CellAlignment;

/// The representation of a single cell in a table row.
/// Each cell contains a string, which will later be displayed at the respective row/column.
pub struct Cell {
    /// Content is a list of strings.
    /// This is done to handle newlines more easily.
    /// On set_content, the incoming string is split by '\n'
    pub (crate) content: Vec<String>,
}

impl Cell {
    /// Create a new Cell from a String
    pub fn new(content: String) -> Self {
        Cell {
            content: content.split('\n').map(|content| content.to_string()).collect(),
        }
    }

    /// Return a copy of the content contained in this cell.
    pub fn get_content(&self) -> String {
        return self.content.join("\n").clone();
    }
}
