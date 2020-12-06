mod cell;
mod column;
mod row;
mod style;
mod table;
mod utils;

pub use crate::cell::{Cell, ToCell, ToCells};
pub use crate::column::Column;
pub use crate::row::{Row, ToRow};
pub use crate::table::Table;
pub use style::*;

#[cfg(doctest)]
doc_comment::doctest!("../README.md");
