pub enum Alignment {
    Left,
    Right,
    Center,
}

/// The representation of a single cell in a table row.
/// Each cell contains a string.
pub struct Cell {
    /// Content is a list of strings.
    /// This is done to handle newlines more easily.
    /// On set_content, the incoming string is split by '\n'
    pub(crate) content: Vec<String>,
    alignment: Alignment,
    width: u16,
    height: u16,
}

impl Cell {
    /// Return a copy of the content contained in this cell.
    pub fn get_content(&self) -> String {
        return self.content.join("\n").clone();
    }

    /// Decide whether the content should be centered or aligned to the left/right.
    pub fn align(&mut self, alignment: Alignment) {
        self.alignment = alignment
    }
}
