use crate::cell::Cell;

pub struct Row {
    cells: Vec<Cell>,
}

impl Row {
    pub fn new() -> Row {
        Row { cells: Vec::new() }
    }

    pub fn from_cells(cells: Vec<Cell>) -> Row {
        Row { cells: cells }
    }

    pub fn longest_content(&self) -> usize {
        let mut length;
        let mut longest = 0;
        for cell in self.cells.iter() {
            length = cell.content.len();
            if length > longest {
                longest = length;
            }
        }

        longest
    }

    pub fn cell_count(&self) -> usize {
        self.cells.len()
    }
}
