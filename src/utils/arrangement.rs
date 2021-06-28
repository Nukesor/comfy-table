use std::collections::BTreeMap;
use std::convert::TryInto;

use super::borders::{
    should_draw_left_border, should_draw_right_border, should_draw_vertical_lines,
};
use super::ColumnDisplayInfo;

use crate::style::ColumnConstraint::*;
use crate::table::Table;
use crate::{style::*, Cell, Column};

type DisplayInfos = BTreeMap<usize, ColumnDisplayInfo>;

/// The ColumnDisplayInfo works with a fixed value for content width.
/// However, if a column is supposed to get a absolute width, we have to make sure that
/// the padding on top of the content width doesn't get larger than the specified absolute width.
///
/// For this reason, we take the targeted width, subtract the column's padding and make sure that
/// the content width is always a minimum of 1
fn absolute_width_with_padding(column: &Column, width: u16) -> u16 {
    let (left, right) = column.padding;
    let mut content_width = i32::from(width) - i32::from(left) - i32::from(right);
    if content_width <= 0 {
        content_width = 1
    }

    content_width.try_into().unwrap_or(u16::MAX)
}

/// Return the amount of visible columns
fn count_visible_columns(columns: &[Column]) -> usize {
    columns.iter().filter(|column| !column.is_hidden()).count()
}

/// Return the amount of visible columns that haven't been checked yet.
///
/// - `column_count` is the total amount of columns that are visible, calculated
///   with [count_visible_columns].
/// - `infos` are all columns that have already been fixed in size or are hidden.
fn count_remaining_columns(column_count: usize, infos: &DisplayInfos) -> usize {
    column_count - infos.iter().filter(|(_, info)| !info.is_hidden).count()
}

fn count_border_columns(table: &Table, visible_columns: usize) -> usize {
    let mut lines = 0;
    // Remove space occupied by borders from remaining_width
    if should_draw_left_border(table) {
        lines += 1;
    }
    if should_draw_right_border(table) {
        lines += 1;
    }
    if should_draw_vertical_lines(table) {
        lines += visible_columns.saturating_sub(1);
    }

    lines
}

pub fn get_delimiter(cell: &Cell, column: &Column, table: &Table) -> char {
    // Determine, which delimiter should be used
    if let Some(delimiter) = cell.delimiter {
        delimiter
    } else if let Some(delimiter) = column.delimiter {
        delimiter
    } else if let Some(delimiter) = table.delimiter {
        delimiter
    } else {
        ' '
    }
}

/// Determine the width of each column depending on the content of the given table.
/// The results uses Option<usize>, since users can choose to hide columns.
pub(crate) fn arrange_content(table: &Table) -> Vec<ColumnDisplayInfo> {
    let table_width = table.get_table_width().map(|width| usize::from(width));
    let mut infos = BTreeMap::new();

    let visible_columns = count_visible_columns(&table.columns);
    for column in table.columns.iter() {
        if column.constraint.is_some() {
            evaluate_constraint(&table, column, &mut infos, table_width, visible_columns);
        }
    }
    //println!("After initial constraints: {:#?}", infos);

    // Fallback to `ContentArrangement::Disabled`, if we don't have any information
    // on how wide the table should be.
    let table_width = match table_width {
        Some(table_width) => table_width,
        None => {
            disabled_arrangement(&table.columns, &mut infos);
            return infos.into_iter().map(|(_, info)| info).collect();
        }
    };

    match &table.arrangement {
        ContentArrangement::Disabled => disabled_arrangement(&table.columns, &mut infos),
        ContentArrangement::Dynamic | ContentArrangement::DynamicFullWidth => {
            dynamic_arrangement(table, &mut infos, table_width);
        }
    }

    infos.into_iter().map(|(_, info)| info).collect()
}

/// Look at given constraints of a column.
/// Some of these contraints can be resolved at the very beginning
fn evaluate_constraint(
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
                    table_width.saturating_sub(count_border_columns(&table, visible_columns));

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
                    table_width.saturating_sub(count_border_columns(&table, visible_columns));

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

/// If dynamic arrangement is disabled, simply set the width of all columns
/// to the respective max content width.
fn disabled_arrangement(columns: &[Column], infos: &mut DisplayInfos) {
    for column in columns.iter() {
        if infos.contains_key(&column.index) {
            continue;
        }

        let mut width = column.get_max_content_width();

        // Reduce the width, if a column has longer content than the specified MaxWidth constraint.
        if let Some(ColumnConstraint::MaxWidth(max_width)) = column.constraint {
            if max_width < width {
                width = absolute_width_with_padding(column, max_width);
            }
        }

        let info = ColumnDisplayInfo::new(column, width);
        infos.insert(column.index, info);
    }
}

/// Try to find the best fit for a given content and table_width
///
/// 1. Determine the amount of available space, after applying fixed columns, padding and borders.
/// 2. Check if there are any columns that require less space than the average
///    remaining space for remaining columns. (This includes the MaxWidth Constraint).
/// 3. Take those columns, fix their size and add the surplus in space to the remaining space.
/// 4. Repeat step 2-3 until no columns with smaller size than average remaining space are left.
/// 5. At this point, the remaining spaces is equally distributed between all columns.
///    It get's a little tricky now. Check the documentation of [optimize_space_after_split]
///    for more information.
/// 6. Divide the remaining space in relatively equal chunks.
///
/// This breaks when:
///
/// 1. A user assigns more space to a few columns than there is on the terminal
/// 2. A user provides more than 100% column width over a few columns.
fn dynamic_arrangement(table: &Table, infos: &mut DisplayInfos, table_width: usize) {
    let column_count = count_visible_columns(&table.columns);

    // Step 1
    // Find out how much space there is left.
    let remaining_width: usize = available_content_width(table, infos, column_count, table_width);

    //println!(
    //    "Table width: {}, Start remaining width {}",
    //    table_width, remaining_width
    //);

    // Step 2-4.
    // Find all columns that require less space than the average.
    // Returns the remaining available width and the amount of remaining columns that need handling
    let (mut remaining_width, mut remaining_columns) =
        find_columns_less_than_average(remaining_width, column_count, &table.columns, infos);

    //{
    //    println!("After less than average: {:#?}", infos);
    //    println!(
    //        "Remaining width {}, column {}",
    //        remaining_width, remaining_columns
    //    );
    //}

    // Step 5
    // All remaining columns should get an equal amount of remaining space.
    // However, we check if we can save some space after the content has been split.
    //
    // We do this if:
    // 1. If there even are remaining columns.
    // 2. If there's space worth saving (more than two characters per rermaining column).
    if remaining_columns != 0 && remaining_width > (2 * remaining_columns) {
        // This is where Step 5 happens.
        let (width, columns) = optimize_space_after_split(
            remaining_width,
            remaining_columns,
            &table.columns,
            infos,
            table,
        );
        remaining_width = width;
        remaining_columns = columns;
    }

    //{
    //    println!("After optimize: {:#?}", infos);
    //    println!(
    //        "Remaining width {}, column {}",
    //        remaining_width, remaining_columns
    //    );
    //}

    // Early exit and one branch of Part 6.
    //
    // All columns have been successfully assigned a width.
    // However, in case the user specified that the full terminal width should always be fully
    // utilized, we have to equally distribute the remaining space across all columns.
    if remaining_columns == 0 {
        if remaining_width > 0 && matches!(table.arrangement, ContentArrangement::DynamicFullWidth)
        {
            use_full_width(infos, remaining_width);
            //println!("After full width: {:#?}", infos);
        }
        return;
    }

    // Step 6. Equally distribute the remaining_width to all remaining columns
    // If we have less than one space per remaining column, give at least one space per column
    if remaining_width < remaining_columns {
        remaining_width = remaining_columns;
    }

    distribute_remaining_space(&table.columns, infos, remaining_width, remaining_columns);

    //println!("After distribute: {:#?}", infos);
}

/// Step 1
///
/// This function calculates the amount of remaining space that can to be distributed between
/// all remaining columns.
///
/// Take the current terminal width and
/// - Subtract borders
/// - Subtract padding
/// - Subtract columns that already have a fixed width.
///
/// This value is converted to a i32 to handle negative values in case we work with a very small
/// terminal.
fn available_content_width(
    table: &Table,
    infos: &DisplayInfos,
    visible_columns: usize,
    mut width: usize,
) -> usize {
    let border_count = count_border_columns(table, visible_columns);
    width = width.saturating_sub(border_count);

    // Subtract all paddings from the remaining width.
    for column in table.columns.iter() {
        if infos.contains_key(&column.index) {
            continue;
        }
        // Remove the fixed padding for each column
        let (left, right) = column.padding;
        width = width.saturating_sub((left + right).into());
    }

    // Remove all already fixed sizes from the remaining_width.
    for info in infos.values() {
        if info.is_hidden {
            continue;
        }
        width = width.saturating_sub(info.width().into());
    }

    width
}

/// Step 2-4
/// This function is part of the column width calculation process.
/// It checks if there are columns that take less space than there's currently available in average
/// for each column.
///
/// The algorithm is a while loop with a nested for loop.
/// 1. We iterate over all columns and check if there are columns that take less space.
/// 2. If we find one or more such columns, we fix their width and add the surplus space to the
///     remaining space. Due to this step, the average space per column increased. Now some other
///     column might be fixed in width as well.
/// 3. Do step 1 and 2, as long as there are columns left and as long as we find columns
///     that take up less space than the current remaining average.
///
/// Parameters
/// 1. `remaining_width`: This is the amount of space that isn't yet reserved by any other column.
///                         We need this to determine the average space each column has left.
///                         Any columns that needs less than this average receives a fixed width.
///                         The leftover space can then be used for the other columns.
/// 2. `column_count`: The amount of non-yet determined columns. Used to calculate the average space.
/// 3. `infos`: The ColumnDisplayInfos used anywhere else
/// 4. `checked`: These are all columns which have a fixed width and are no longer need checking.
fn find_columns_less_than_average(
    mut remaining_width: usize,
    column_count: usize,
    columns: &[Column],
    infos: &mut DisplayInfos,
) -> (usize, usize) {
    let mut found_smaller = true;
    let mut remaining_columns = count_remaining_columns(column_count, infos);
    while found_smaller {
        found_smaller = false;

        // There are no columns left to check. Proceed to the next step
        if remaining_columns == 0 {
            break;
        }

        let mut average_space = remaining_width / remaining_columns;
        // We have no space left, the terminal is either tiny or the other columns are huge.
        if average_space == 0 {
            break;
        }

        for column in columns.iter() {
            // Ignore hidden columns
            // We already checked this column, skip it
            if infos.contains_key(&column.index) {
                continue;
            }

            // The column has a MaxWidth Constraint.
            // we can fix the column to this max_width and mark it as checked, if these
            // two conditions are met:
            // - The average remaining space is bigger then the MaxWidth constraint.
            // - The actual max content of the column is bigger than the MaxWidth constraint.
            if let Some(ColumnConstraint::MaxWidth(max_width)) = column.constraint {
                // Max/Min constraints always include padding!
                let space_after_padding = average_space + usize::from(column.get_padding_width());

                // Check that both conditions mentioned above are met.
                if usize::from(max_width) <= space_after_padding
                    && column.get_max_width() >= max_width
                {
                    // Save the calculated info, this column has been handled.
                    let width = absolute_width_with_padding(column, max_width);
                    let info = ColumnDisplayInfo::new(column, width);
                    infos.insert(column.index, info);

                    // Continue with new recalculated width
                    remaining_width = remaining_width.saturating_sub(width.into());
                    remaining_columns -= 1;
                    if remaining_columns == 0 {
                        break;
                    }
                    average_space = remaining_width / remaining_columns;
                    found_smaller = true;
                    continue;
                }
            }

            // The column has a smaller max_content_width than the average space.
            // Fix the width to max_content_width and mark it as checked
            if usize::from(column.get_max_content_width()) < average_space {
                let info = ColumnDisplayInfo::new(column, column.get_max_content_width());
                infos.insert(column.index, info);

                // Continue with new recalculated width
                remaining_width = remaining_width.saturating_sub(column.max_content_width.into());
                remaining_columns -= 1;
                if remaining_columns == 0 {
                    break;
                }
                average_space = remaining_width / remaining_columns;
                found_smaller = true;
            }
        }
    }

    (remaining_width, remaining_columns)
}

/// Step 5.
///
/// Some Column's are too big and need to be split.
/// We're now going to simulate how this might look like.
/// The reason for this is the way we're splitting, which is to prefer a split at a delimiter.
/// This can lead to a column needing less space than it was initially assigned.
///
/// Example:
/// A column is allowed to have a width of 10 characters.
/// A cell's content looks like this `sometest sometest`, which is 17 chars wide.
/// After splitting at the default delimiter (space), it looks like this:
/// ```text
/// sometest
/// sometest
/// ```
/// Even though the column required 17 spaces beforehand, it can now be shrunk to 8 chars width.
///
/// By doing this for each column, we can save a lot of space in some edge-cases.
fn optimize_space_after_split(
    mut remaining_width: usize,
    mut remaining_columns: usize,
    columns: &[Column],
    infos: &mut DisplayInfos,
    table: &Table,
) -> (usize, usize) {
    let mut found_smaller = true;
    // Calculate the average space that remains for each column.
    let mut average_space = remaining_width / remaining_columns;

    // Do this as long as we find a smaller column
    while found_smaller {
        found_smaller = false;
        for column in columns.iter() {
            // We already checked this column, skip it
            if infos.contains_key(&column.index) {
                continue;
            }

            let longest_line = get_longest_line_after_split(average_space, column, table);

            // If there's a considerable amount space left after splitting, we freeze the column and
            // set its content width to the calculated post-split width.
            let remaining_space = average_space.saturating_sub(longest_line);
            if remaining_space >= 3 {
                let info =
                    ColumnDisplayInfo::new(column, longest_line.try_into().unwrap_or(u16::MAX));
                infos.insert(column.index, info);

                remaining_width = remaining_width.saturating_sub(longest_line);
                remaining_columns -= 1;
                if remaining_columns == 0 {
                    break;
                }
                average_space = remaining_width / remaining_columns;
                found_smaller = true;
            }
        }
    }

    (remaining_width, remaining_columns)
}

/// Part of Step 5.
///
/// This function simulates the split of a Column's content and returns the longest
/// existing line after the split.
///
/// A lot of this logic is duplicated from the [utils::format::format_row] function.
fn get_longest_line_after_split(average_space: usize, column: &Column, table: &Table) -> usize {
    // Collect all resulting lines of the column in a single vector.
    // That way we can easily determine the longest line afterwards.
    let mut column_lines = Vec::new();

    // Iterate
    for cell in table.column_cells_iter(column.index) {
        // Only look at rows that actually contain this cell.
        let cell = if let Some(cell) = cell {
            cell
        } else {
            continue;
        };

        let delimiter = get_delimiter(cell, column, table);

        // Create a temporary ColumnDisplayInfo with the average space as width.
        // That way we can simulate how the splitted text will look like.
        let info = ColumnDisplayInfo::new(column, average_space.try_into().unwrap_or(u16::MAX));

        // Iterate over each line and split it into multiple lines, if necessary.
        // Newlines added by the user will be preserved.
        for line in cell.content.iter() {
            if (line.chars().count()) > average_space {
                let mut splitted = super::split::split_line(line, &info, delimiter);
                column_lines.append(&mut splitted);
            } else {
                column_lines.push(line.into());
            }
        }
    }

    // Get the longest line, default to length 0 if no lines exist.
    column_lines
        .iter()
        .map(|line| line.len())
        .max()
        .unwrap_or(0)
}

/// Step 6 - First branch
///
/// At this point of time, all columns have been assigned some kind of width!
/// The user wants to utilize the full width of the terminal and there's space left.
///
/// Equally distribute the remaining space between all columns.
fn use_full_width(infos: &mut DisplayInfos, remaining_width: usize) {
    let visible_columns = infos.iter().filter(|(_, info)| !info.is_hidden).count();

    if visible_columns == 0 {
        return;
    }

    // Calculate the amount of average remaining space per column.
    // Since we do integer division, there is most likely a little bit of non equally-divisable space.
    // We then try to distribute it as fair as possible (from left to right).
    let average_space = remaining_width / visible_columns;
    let mut excess = remaining_width - (average_space * visible_columns);

    for (_, info) in infos.iter_mut() {
        // Ignore hidden columns
        if info.is_hidden {
            continue;
        }

        // Distribute the non-divisable excess from left-to right until nothing is left.
        let width = if excess > 0 {
            excess -= 1;
            (average_space + 1).try_into().unwrap_or(u16::MAX)
        } else {
            average_space.try_into().unwrap_or(u16::MAX)
        };

        info.content_width += width;
    }
}

/// Step 6 - Second branch
///
/// Not all columns have a determined width yet -> The content still doesn't fully fit into the
/// given width.
///
/// This function now equally distributes the remaining width between the remaining columns.
fn distribute_remaining_space(
    columns: &[Column],
    infos: &mut DisplayInfos,
    remaining_width: usize,
    remaining_columns: usize,
) {
    // Calculate the amount of average remaining space per column.
    // Since we do integer division, there is most likely a little bit of non equally-divisable space.
    // We then try to distribute it as fair as possible (from left to right).
    let average_space = remaining_width / remaining_columns;
    let mut excess = remaining_width - (average_space * remaining_columns);

    for column in columns.iter() {
        // Ignore hidden columns
        if infos.contains_key(&column.index) {
            continue;
        }

        // Distribute the non-divisable excess from left-to right until nothing is left.
        let width = if excess > 0 {
            excess -= 1;
            (average_space + 1).try_into().unwrap_or(u16::MAX)
        } else {
            average_space.try_into().unwrap_or(u16::MAX)
        };

        let info = ColumnDisplayInfo::new(column, width);
        infos.insert(column.index, info);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_disabled_arrangement() {
        let mut table = Table::new();
        table.set_header(&vec!["head", "head", "head"]);
        table.add_row(&vec!["four", "fivef", "sixsix"]);

        let display_infos = arrange_content(&table);

        // The width should be the width of the rows + padding
        let widths: Vec<u16> = display_infos.iter().map(|info| info.width()).collect();
        assert_eq!(widths, vec![6, 7, 8]);
    }
}
