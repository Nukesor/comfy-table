/// The representation of a single cell in a table row.
/// Each cell contains a string, which will later be displayed at the respective row/column.
pub struct Cell {
    /// Content is a list of strings.
    /// This is done to handle newlines more easily.
    /// On set_content, the incoming string is split by '\n'
    pub(crate) content: Vec<String>,
}

impl Cell {
    /// Create a new Cell
    pub fn new<T: ToString>(content: T) -> Self {
        Cell {
            content: content.to_string()
                .split('\n')
                .map(|content| content.to_string())
                .collect(),
        }
    }

    /// Return a copy of the content contained in this cell.
    pub fn get_content(&self) -> String {
        return self.content.join("\n").clone();
    }
}


pub trait ToCells {
    fn to_cells(&mut self) -> Vec<Cell>;
}

impl<T: Copy + IntoIterator> ToCells for T where
    T::Item: ToString {
    fn to_cells(&mut self) -> Vec<Cell> {
        self.into_iter().map(|item| Cell::new(item)).collect()
    }
}
