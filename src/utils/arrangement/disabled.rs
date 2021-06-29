use super::constraints::get_max_constraint;
use super::helper::*;
use super::{ColumnDisplayInfo, DisplayInfos};
use crate::Table;

/// Dynamic arrangement is disabled.
/// Apply all non-relative constraints, and set the width of all remaining columns to the
/// respective max content width.
pub fn arrange(table: &Table, infos: &mut DisplayInfos, visible_columns: usize) {
    for column in table.columns.iter() {
        if infos.contains_key(&column.index) {
            continue;
        }

        let mut width = column.get_max_content_width();

        // Reduce the width, if a column has longer content than the specified MaxWidth constraint.
        if let Some(max_width) =
            get_max_constraint(table, &column.constraint, None, visible_columns)
        {
            if max_width < width {
                width = absolute_width_with_padding(column, max_width);
            }
        }

        let info = ColumnDisplayInfo::new(column, width);
        infos.insert(column.index, info);
    }
}
