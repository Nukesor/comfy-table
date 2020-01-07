pub mod cell;
pub mod column;
pub mod row;
pub mod styling;
pub mod table;
mod utils;

pub use crate::styling::cell::CellAlignment;
pub use crate::styling::column::Constraint;
pub use crate::styling::table::ContentArrangement;

pub mod prelude {
    pub use crate::cell::Cell;
    pub use crate::row::Row;
    pub use crate::styling::cell::CellAlignment;
    pub use crate::table::Table;
}
