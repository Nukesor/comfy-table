use ::crossterm::style::{Color, Attribute};

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
    pub(crate) fg: Option<Color>,
    pub(crate) bg: Option<Color>,
    pub(crate) attributes: Vec<Attribute>,
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
            fg: None,
            bg: None,
            attributes: Vec::new(),
        }
    }

    /// Return a copy of the content contained in this cell.
    pub fn get_content(&self) -> String {
        return self.content.join("\n").clone();
    }

    /// Set the foreground text color for this cell.
    /// comfy-table uses [Crossterm Colors](crossterm::style::Color).
    /// Look at their documentation for all possible Colors.
    /// ```
    /// use comfy_table::style::Color;
    /// use comfy_table::cell::Cell;
    ///
    /// let mut cell = Cell::new("Some content")
    ///     .set_fg(Some(Color::Red));
    /// ```
    pub fn set_fg(mut self, color: Option<Color>) -> Self {
        self.fg = color;

        self
    }

    /// Set the background color for this cell.
    /// comfy-table uses [Crossterm Colors](crossterm::style::Color).
    /// Look at their documentation for all possible Colors.
    /// ```
    /// use comfy_table::style::Color;
    /// use comfy_table::cell::Cell;
    ///
    /// let mut cell = Cell::new("Some content")
    ///     .set_bg(Some(Color::Red));
    /// ```
    pub fn set_bg(mut self, color: Option<Color>) -> Self {
        self.bg = color;

        self
    }

    /// Add a styling attribute to the content cell
    /// Those can be **bold**, _italic_, blinking and many more.
    /// comfy-table uses [Crossterm Attributes](crossterm::style::Attribute).
    /// Look at their documentation for all possible Attributes.
    /// ```
    /// use comfy_table::style::Attribute;
    /// use comfy_table::cell::Cell;
    ///
    /// let mut cell = Cell::new("Some content")
    ///     .add_attribute(Attribute::Bold);
    /// ```
    pub fn add_attribute(mut self, attribute: Attribute) -> Self {
        self.attributes.push(attribute);

        self
    }

    /// Same as add_attribute, but you can pass a Vector of Attributes
    pub fn add_attributes(mut self, mut attribute: Vec<Attribute>) -> Self {
        self.attributes.append(&mut attribute);

        self
    }
}
