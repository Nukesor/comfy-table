pub mod cell;
pub mod column;
pub mod row;
pub mod style;
pub mod table;
mod utils;

pub use prelude::*;
pub use crate::style::cell::CellAlignment;
pub use crate::style::column::Constraint;
pub use crate::style::table::ContentArrangement;

pub mod prelude {
    pub use crate::cell::Cell;
    pub use crate::row::Row;
    pub use crate::style::cell::CellAlignment;
    pub use crate::table::Table;
    pub use crate::column::Column;
}
