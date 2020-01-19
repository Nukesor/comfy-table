use crate::cell::{ToCells, Cell};

pub trait ToRow {
    fn to_row(&mut self) -> Row;

}

impl<T: ToCells> ToRow for T {
    fn to_row(&mut self) -> Row {
        Row::from(self.to_cells())
    }
}

// This is somewhat expensive, but convenient
impl ToRow for Row {
    fn to_row(&mut self) -> Row {
        self.clone()
    }
}

#[derive(Clone)]
pub struct Row {
    /// Index of the row. This will be set as soon as the row is added to the table
    pub(crate) index: Option<usize>,
    pub(crate) cells: Vec<Cell>,
}

impl Row {
    pub fn new() -> Row {
        Row {
            index: None,
            cells: Vec::new(),
        }
    }

    pub fn from<T: ToCells>(mut cells: T) -> Row {
        Row {
            index: None,
            cells: cells.to_cells(),
        }
    }

    /// Get the longest content width for all cells of this row
    pub fn max_content_widths(&self) -> Vec<usize> {
        // Iterate over all cells
        self.cells
            .iter()
            .map(|cell| {
                // Iterate over all content strings and return a vector of string widths.
                // Each entry represents the longest string width for a cell.
                cell.content
                    .iter()
                    .map(|string| string.len())
                    .max()
                    .unwrap_or(0)
            })
            .collect()
    }

    /// Return the amount of cells on this row.
    pub fn cell_count(&self) -> usize {
        self.cells.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_correct_max_content_width() {
        let row = Row::from(&vec!["", "four", "fivef", "sixsix", "11 but with\na newline"]);

        let max_content_widths = row.max_content_widths();

        assert_eq!(max_content_widths, vec![0, 4, 5, 6, 11]);
    }
}
