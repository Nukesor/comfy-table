#![forbid(unsafe_code)]
mod cell;
mod column;
mod row;
mod style;
mod table;
mod utils;

pub use crate::cell::{Cell, Cells};
pub use crate::column::Column;
pub use crate::row::Row;
pub use crate::table::{ColumnCellIter, Table};
pub use style::*;

#[cfg(doctest)]
doc_comment::doctest!("../README.md");
