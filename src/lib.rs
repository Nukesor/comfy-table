#![forbid(unsafe_code)]
mod cell;
mod column;
mod row;
mod style;
mod table;
#[cfg(feature = "integration_test")]
/// We publicly expose the internal [utils] module for our integration tests.
/// There's some logic we need from inside here.
/// The API inside of this isn't considered stable and shouldnt' be used.
pub mod utils;
#[cfg(not(feature = "integration_test"))]
mod utils;

pub use crate::cell::{Cell, Cells};
pub use crate::column::Column;
pub use crate::row::Row;
pub use crate::table::{ColumnCellIter, Table};
pub use style::*;

#[cfg(doctest)]
doc_comment::doctest!("../README.md");
