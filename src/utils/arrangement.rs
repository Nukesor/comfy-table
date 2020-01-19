use ::termion::terminal_size;

use crate::column::Column;
use crate::styling::cell::CellAlignment;
use crate::styling::column::Constraint;
use crate::styling::column::Constraint::*;
use crate::styling::table::ContentArrangement;
use crate::table::Table;

/// This is used to store various styling options for a specific column
/// The struct is only created temporarily during the drawing process
pub struct ColumnDisplayInfo {
    pub padding: (u16, u16),
    /// The max amount of characters over all lines in this column
    max_content_width: u16,
    /// Determine, whether the width attribute should be used.
    /// If true, the column has fixed width.
    fixed: bool,
    pub width: u16,
    /// A constraint that should be considered during automatic
    pub constraint: Option<Constraint>,
    /// Determine, whether this column should be hidden (ignored)
    pub hidden: bool,
    /// Determine, whether this column should be hidden (ignored)
    pub cell_alignment: Option<CellAlignment>,
}

impl ColumnDisplayInfo {
    fn new(column: &Column) -> Self {
        ColumnDisplayInfo {
            padding: column.padding,
            max_content_width: column.max_content_width,
            width: 0,
            fixed: false,
            constraint: None,
            hidden: false,
            cell_alignment: column.cell_alignment,
        }
    }
}

/// Determine the width of each column depending on the content of the given table.
/// The results uses Option<usize>, since users can choose to hide columns.
pub fn arrange_content(table: &Table) -> Vec<ColumnDisplayInfo> {
    let (term_width, _) = terminal_size().unwrap();

    let mut display_infos = Vec::new();
    for column in table.columns.iter() {
        let mut info = ColumnDisplayInfo::new(column);
        if let Some(constraint) = column.constraint.as_ref() {
            evaluate_constraint(&mut info, constraint, term_width);
        }

        display_infos.push(info);
    }

    match &table.arrangement {
        ContentArrangement::Disabled => disabled_arrangement(&mut display_infos),
        _ => (),
        //        ContentArrangement::Automatic => automatic_arrangement(&mut display_infos, term_width),
    }

    display_infos
}

/// Look at given constraints of a column and populate the ColumnDisplayInfo depending on those.
fn evaluate_constraint(info: &mut ColumnDisplayInfo, constraint: &Constraint, term_width: u16) {
    match constraint {
        Hidden => info.hidden = true,
        Width(width) => {
            info.width = *width;
            info.fixed = true;
        }
        Percentage(percent) => {
            let width = term_width * percent / 100;
            info.width = width;
            info.fixed = true;
        }
        ContentWidth => {
            info.width = info.max_content_width + info.padding.0 + info.padding.1;
            info.fixed = true;
        }
        MaxWidth(max_width) => info.constraint = Some(MaxWidth(*max_width)),
        MinWidth(min_width) => info.constraint = Some(MinWidth(*min_width)),
    }
}

/// If automatic arrangement is disabled, simply set the width of all columns
/// to the respective max content width.
fn disabled_arrangement(infos: &mut Vec<ColumnDisplayInfo>) {
    for info in infos.iter_mut() {
        if !info.fixed {
            info.width = info.max_content_width + info.padding.0 + info.padding.1;
            info.fixed = true;
        }
    }
}

//fn automatic_arrangement(infos: &mut Vec<ColumnDisplayInfo>, term_width: u16) {
//    let mut remaining_width = term_width;
//    for column in table.columns.iter_mut() {
//        // A fix width is enforced for this column
//        if let Some(constraint) = column.get_constraint().clone() {
//            match constraint {
//                Width(width) => {
//                    remaining_width -= width;
//                }
//                Ignore => (),
//                Percentage(percentage) => {
//                    let width = term_width * 100/percentage;
//                    remaining_width -= width;
//                }
//                _ => ()
//            }
//        } else if column.max_content_width < default_column_width {
//            let width = column.max_content_width;
//        }
//    }
//}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_disabled_arrangement() {
        let mut table = Table::new();
        table.set_header(&vec!["head", "head", "head"]);
        table.add_row(&vec!["four", "fivef", "sixsix"]);

        let display_infos = arrange_content(&table);
        // The max_ width should just be copied from the column
        let max_widths: Vec<u16> = display_infos.iter().map(|info| info.max_content_width).collect();
        assert_eq!(max_widths, vec![4, 5, 6]);

        // In default mode without any constraints
        let widths: Vec<u16> = display_infos.iter().map(|info| info.width).collect();
        assert_eq!(widths, vec![6, 7, 8]);
    }
}
