/// Allow the conversion of a type to a vector of cells.
/// By default this is implemented for all types implementing
/// IntoIterator where the iterated Item type implements ToString.
/// E.g. a Vec<i32> works
pub trait ToCells {
    fn to_cells(&mut self) -> Vec<Cell>;
}

impl<T: Clone + IntoIterator> ToCells for T
where
    T::Item: ToString,
{
    fn to_cells(&mut self) -> Vec<Cell> {
        self.clone()
            .into_iter()
            .map(|item| Cell::new(item))
            .collect()
    }
}

/// This is quite expensive, but it's convenient
impl ToString for Cell {
    fn to_string(&self) -> String {
        self.get_content()
    }
}

/// The representation of a single cell in a table row.
/// Each cell contains a string, which will later be displayed at the respective row/column.
#[derive(Clone)]
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
            content: content
                .to_string()
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
