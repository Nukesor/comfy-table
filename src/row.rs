use crate::cell::{Cell, ToCells};
use ::std::slice::Iter;

/// Each row contains [Cells](crate::Cell) and can be added to a [Table](crate::Table).
#[derive(Clone, Debug)]
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

    /// Create a Row from any `Iter<T: ToCell>`
    /// ```
    /// use comfy_table::{Row, Cell};
    ///
    /// let row = Row::from(vec!["One", "Two", "Three",]);
    /// let row = Row::from(vec![
    ///    Cell::new("One"),
    ///    Cell::new("Two"),
    ///    Cell::new("Three"),
    /// ]);
    /// ```
    pub fn from<T: ToCells>(cells: T) -> Row {
        Row {
            index: None,
            cells: cells.to_cells(),
        }
    }

    /// Add a cell to the row
    /// ```
    /// use comfy_table::{Row, Cell};
    ///
    /// let mut row = Row::new();
    /// row.add_cell(Cell::new("One"));
    /// ```
    pub fn add_cell(&mut self, cell: Cell) -> &mut Self {
        self.cells.push(cell);

        self
    }

    /// Get the longest content width for all cells of this row
    pub(crate) fn max_content_widths(&self) -> Vec<usize> {
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

    /// An iterator over all cells of this row
    pub fn cell_iter(&self) -> Iter<Cell> {
        self.cells.iter()
    }
}

pub trait ToRow {
    fn to_row(self) -> Row;
}

impl<T: ToCells> ToRow for T {
    fn to_row(self) -> Row {
        Row::from(self.to_cells())
    }
}

impl ToRow for Row {
    fn to_row(self) -> Row {
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_correct_max_content_width() {
        let row = Row::from(&vec![
            "",
            "four",
            "fivef",
            "sixsix",
            "11 but with\na newline",
        ]);

        let max_content_widths = row.max_content_widths();

        assert_eq!(max_content_widths, vec![0, 4, 5, 6, 11]);
    }

    #[test]
    fn test_some_functions() {
        let cells = vec![
            "one",
            "two",
            "three",
        ];
        let mut row = Row::new();
        for cell in cells.iter() {
            row.add_cell(Cell::new(cell));
        }

        let mut cell_content_iter = cells.iter();
        for cell in row.cell_iter() {
            assert_eq!(cell.get_content(), cell_content_iter.next().unwrap().to_string());
        }
    }

}
