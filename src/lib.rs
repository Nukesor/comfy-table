pub mod cell;
pub mod column;
pub mod row;
pub mod styling;
mod utils;
pub mod table;


pub use crate::styling::cell::CellAlignment;
pub use crate::styling::table::ContentArrangement;

mod prelude {
    pub use crate::table::Table;
    pub use crate::row::Row;
    pub use crate::cell::Cell;
}
