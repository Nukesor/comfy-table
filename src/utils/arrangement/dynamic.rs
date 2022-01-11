use unicode_width::UnicodeWidthStr;

use super::constraints::get_max_constraint;
use super::constraints::get_min_constraint;
use super::helper::*;
use super::{ColumnDisplayInfo, DisplayInfos};
use crate::style::*;
use crate::utils::formatting::content_split::split_line;
use crate::{Column, Table};

/// Try to find the best fit for a given content and table_width
///
/// 1. Determine the amount of available space, after applying fixed columns, padding and borders.
/// 2. Check if there are any columns that require less space than the average
///    remaining space for remaining columns. (This includes the MaxWidth Constraint).
/// 3. Take those columns, fix their size and add the surplus in space to the remaining space.
/// 4. Repeat step 2-3 until no columns with smaller size than average remaining space are left.
/// 5. Now that we know how much space we have to work with, we have to check again for
///    LowerBoundary constraints. If there are any columns that have a higher LowerBoundary,
///    we have to fix that column to this size.
/// 6. At this point, the remaining spaces is equally distributed between all columns.
///    It get's a little tricky now. Check the documentation of [optimize_space_after_split]
///    for more information.
/// 7. Divide the remaining space in relatively equal chunks.
///
/// This breaks when:
///
/// 1. A user assigns more space to a few columns than there is on the terminal
/// 2. A user provides more than 100% column width over a few columns.
pub fn arrange(table: &Table, infos: &mut DisplayInfos, table_width: usize) {
    let visible_columns = count_visible_columns(&table.columns);

    // Step 1
    // Find out how much space there is left.
    let remaining_width: usize =
        available_content_width(table, infos, visible_columns, table_width);

    //println!(
    //    "Table width: {}, Start remaining width {}",
    //    table_width, remaining_width
    //);

    // Step 2-4.
    // Find all columns that require less space than the average.
    // Returns the remaining available width and the amount of remaining columns that need handling
    let (mut remaining_width, mut remaining_columns) =
        find_columns_less_than_average(table, infos, table_width, remaining_width, visible_columns);

    // Step 5.
    //
    // Iterate through all undecided columns and enforce LowerBoundary constraints, if they're
    // bigger than the current average space.
    if remaining_columns > 0 {
        // TODO: Refactor once destructuring assignments are stable.
        let (width, columns) = enforce_lower_boundary_constraints(
            table,
            infos,
            table_width,
            remaining_width,
            remaining_columns,
            visible_columns,
        );
        remaining_width = width;
        remaining_columns = columns;
    }

    //{
    //    println!("After less than average: {:#?}", infos);
    //    println!(
    //        "Remaining width {}, column {}",
    //        remaining_width, remaining_columns
    //    );
    //}

    // Step 6
    // All remaining columns should get an equal amount of remaining space.
    // However, we check if we can save some space after the content has been split.
    //
    // We only do this if there are remaining columns.
    if remaining_columns > 0 {
        // This is where Step 5 happens.
        let (width, columns) = optimize_space_after_split(
            table,
            &table.columns,
            infos,
            remaining_width,
            remaining_columns,
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

    // Early exit and one branch of Part 7.
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

    // Step 7. Equally distribute the remaining_width to all remaining columns
    // If we have less than one space per remaining column, give at least one space per column
    if remaining_width < remaining_columns {
        remaining_width = remaining_columns;
    }

    distribute_remaining_space(&table.columns, infos, remaining_width, remaining_columns);

    //println!("After distribute: {:#?}", infos);
}

/// Step 1
///
/// This function calculates the amount of remaining space that can be distributed between
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
/// Parameters:
/// - `table_width`: The absolute amount of available space.
/// - `remaining_width`: This is the amount of space that isn't yet reserved by any other column.
///                      We need this to determine the average space each column has left.
///                      Any columns that needs less than this average receives a fixed width.
///                      The leftover space can then be used for the other columns.
/// - `visible_columns`: All visible columns that should be displayed.
///
/// Returns:
/// `(remaining_width: usize, remaining_columns: u16)`
fn find_columns_less_than_average(
    table: &Table,
    infos: &mut DisplayInfos,
    table_width: usize,
    mut remaining_width: usize,
    visible_coulumns: usize,
) -> (usize, usize) {
    let mut found_smaller = true;
    let mut remaining_columns = count_remaining_columns(visible_coulumns, infos);
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

        for column in table.columns.iter() {
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
            if let Some(max_width) = get_max_constraint(
                table,
                &column.constraint,
                Some(table_width),
                visible_coulumns,
            ) {
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

/// Step 5
///
/// Determine, whether there are any columns that have a higher LowerBoundary than the currently
/// available average width.
///
/// These columns will then get fixed to the specified amount.
fn enforce_lower_boundary_constraints(
    table: &Table,
    infos: &mut DisplayInfos,
    table_width: usize,
    mut remaining_width: usize,
    mut remaining_columns: usize,
    visible_columns: usize,
) -> (usize, usize) {
    let mut average_space = remaining_width / remaining_columns;
    for column in table.columns.iter() {
        // Ignore hidden columns
        // We already checked this column, skip it
        if infos.contains_key(&column.index) {
            continue;
        }

        // Check whether the column has a LowerBoundary constraint.
        let min_width = if let Some(min_width) = get_min_constraint(
            table,
            &column.constraint,
            Some(table_width),
            visible_columns,
        ) {
            min_width
        } else {
            continue;
        };

        // Only proceed if the average spaces is smaller than the specified lower boundary.
        if average_space >= min_width.into() {
            continue;
        }

        // This column would get smaller than the specified lower boundary.
        // Fix its width!!!
        let width = absolute_width_with_padding(column, min_width);
        let info = ColumnDisplayInfo::new(column, width);
        infos.insert(column.index, info);

        // Continue with new recalculated width
        remaining_width = remaining_width.saturating_sub(width.into());
        remaining_columns -= 1;
        if remaining_columns == 0 {
            break;
        }
        average_space = remaining_width / remaining_columns;
        continue;
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
    table: &Table,
    columns: &[Column],
    infos: &mut DisplayInfos,
    mut remaining_width: usize,
    mut remaining_columns: usize,
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

        let delimiter = get_delimiter(table, column, cell);

        // Create a temporary ColumnDisplayInfo with the average space as width.
        // That way we can simulate how the splitted text will look like.
        let info = ColumnDisplayInfo::new(column, average_space.try_into().unwrap_or(u16::MAX));

        // Iterate over each line and split it into multiple lines, if necessary.
        // Newlines added by the user will be preserved.
        for line in cell.content.iter() {
            if line.width() > average_space {
                let mut splitted = split_line(line, &info, delimiter);
                column_lines.append(&mut splitted);
            } else {
                column_lines.push(line.into());
            }
        }
    }

    // Get the longest line, default to length 0 if no lines exist.
    column_lines
        .iter()
        .map(|line| line.width())
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
