use std::collections::BTreeMap;

use super::ColumnDisplayInfo;
use crate::style::ContentArrangement;
use crate::table::Table;

mod constraints;
mod disabled;
mod dynamic;
mod helper;

type DisplayInfos = BTreeMap<usize, ColumnDisplayInfo>;

/// Determine the width of each column depending on the content of the given table.
/// The results uses Option<usize>, since users can choose to hide columns.
pub(crate) fn arrange_content(table: &Table) -> Vec<ColumnDisplayInfo> {
    let table_width = table.get_table_width().map(usize::from);
    let mut infos = BTreeMap::new();

    let visible_columns = helper::count_visible_columns(&table.columns);
    for column in table.columns.iter() {
        if column.constraint.is_some() {
            constraints::evaluate(table, column, &mut infos, table_width, visible_columns);
        }
    }
    #[cfg(feature = "debug")]
    println!("After initial constraints: {infos:#?}");

    // Fallback to `ContentArrangement::Disabled`, if we don't have any information
    // on how wide the table should be.
    let table_width = if let Some(table_width) = table_width {
        table_width
    } else {
        disabled::arrange(table, &mut infos, visible_columns);
        return infos.into_iter().map(|(_, info)| info).collect();
    };

    match &table.arrangement {
        ContentArrangement::Disabled => disabled::arrange(table, &mut infos, visible_columns),
        ContentArrangement::Dynamic | ContentArrangement::DynamicFullWidth => {
            dynamic::arrange(table, &mut infos, table_width);
        }
    }

    infos.into_iter().map(|(_, info)| info).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_disabled_arrangement() {
        let mut table = Table::new();
        table.set_header(&vec!["head", "head", "head"]);
        table.add_row(&vec!["__", "fivef", "sixsix"]);

        let display_infos = arrange_content(&table);

        // The width should be the width of the rows + padding
        let widths: Vec<u16> = display_infos.iter().map(ColumnDisplayInfo::width).collect();
        assert_eq!(widths, vec![6, 7, 8]);
    }
}
