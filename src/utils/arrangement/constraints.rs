use std::convert::TryInto;

use super::helper::*;
use super::{ColumnDisplayInfo, DisplayInfos};
use crate::style::{ColumnConstraint, ColumnConstraint::*};
use crate::{Column, Table};

/// Look at given constraints of a column and check if some of them can be resolved at the very
/// beginning.
///
/// For example:
/// - We get an absolute width.
/// - MinWidth constraints on columns, whose content is garantueed to be smaller than the specified
///     minimal width.
/// - The Column is supposed to be hidden.
pub fn evaluate(
    table: &Table,
    column: &Column,
    infos: &mut DisplayInfos,
    table_width: Option<usize>,
    visible_columns: usize,
) {
    match column.constraint {
        Some(ContentWidth) => {
            let info = ColumnDisplayInfo::new(column, column.max_content_width);
            infos.insert(column.index, info);
        }
        Some(Width(width)) => {
            // The column should get always get a fixed width.
            let width = absolute_width_with_padding(column, width);
            let info = ColumnDisplayInfo::new(column, width);
            infos.insert(column.index, info);
        }
        Some(MinWidth(min_width)) => {
            // In case a min_width is specified, we may already fix the size of the column.
            // We do this, if we know that the content is smaller than the min size.
            if column.get_max_width() <= min_width {
                let width = absolute_width_with_padding(column, min_width);
                let info = ColumnDisplayInfo::new(column, width);
                infos.insert(column.index, info);
            }
        }
        Some(Percentage(percent)) => {
            // The column should always get a fixed percentage.
            if let Some(table_width) = table_width {
                // Get the table width minus borders
                let width =
                    table_width.saturating_sub(count_border_columns(table, visible_columns));

                // Calculate the percentage of that width.
                let mut width = (width * usize::from(percent) / 100)
                    .try_into()
                    .unwrap_or(u16::MAX);

                // Set the width to that fixed percentage.
                width = absolute_width_with_padding(column, width);
                let info = ColumnDisplayInfo::new(column, width);
                infos.insert(column.index, info);
            }
        }
        Some(MinPercentage(percent)) => {
            // In case a min_percentage_width is specified, we may already fix the size of the column.
            // We do this, if we know that the content is smaller than the min size.
            if let Some(table_width) = table_width {
                // Get the table width minus borders
                let width =
                    table_width.saturating_sub(count_border_columns(table, visible_columns));

                // Calculate the percentage of that width.
                let mut width = (width * usize::from(percent) / 100)
                    .try_into()
                    .unwrap_or(u16::MAX);

                // Set the width to that fixed percentage.
                width = absolute_width_with_padding(column, width);
                if column.get_max_width() <= width {
                    let info = ColumnDisplayInfo::new(column, width);
                    infos.insert(column.index, info);
                }
            }
        }
        Some(Hidden) => {
            let mut info = ColumnDisplayInfo::new(column, column.max_content_width);
            info.is_hidden = true;
            infos.insert(column.index, info);
        }
        _ => {}
    }
}

/// A little wrapper, which resolves MaxPercentage constraints to their actual MaxWidth value for
/// the current table and terminal width.
pub fn get_max_constraint(
    table: &Table,
    constraint: &Option<ColumnConstraint>,
    table_width: usize,
    visible_columns: usize,
) -> Option<ColumnConstraint> {
    match constraint {
        Some(MaxWidth(width)) => Some(MaxWidth(*width)),
        Some(MaxPercentage(percent)) => {
            // Get the table width minus borders.
            let width = table_width.saturating_sub(count_border_columns(table, visible_columns));

            // Calculate the absolute value in actual columns.
            let width = (width * usize::from(*percent) / 100)
                .try_into()
                .unwrap_or(u16::MAX);
            Some(MaxWidth(width))
        }
        _ => None,
    }
}
