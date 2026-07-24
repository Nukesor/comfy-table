use unicode_width::UnicodeWidthStr;

use super::{ColumnDisplayInfo, DisplayInfos, constraint, helper::*};
use crate::{Column, Table, style::*, utils::formatting::content_split::split_line};

/// State for the dynamic width-arrangement algorithm.
///
/// In contrast to the other layout algorithms, the dynamic codepath is by far the most complex, as
/// we have to perform various calculations, fixations of columns and at times multiple
/// optimization passes to find a good layout.
///
/// It also keeps track of our "budget" in the form of `remaining_width`.
struct ArrangementState<'a> {
    table: &'a Table,
    /// Widths of columns that have been decided so far (including hidden ones).
    infos: &'a mut DisplayInfos,
    /// Max content width per column, indexed by `column.index`.
    max_content_widths: &'a [u16],
    /// Content width that isn't yet claimed by any decided column.
    remaining_width: usize,
    /// Number of visible columns without a decided width.
    remaining_columns: usize,
    /// Constant total number of visible columns.
    visible_columns: usize,
}

impl<'a> ArrangementState<'a> {
    /// Step 1
    ///
    /// Initialize the state and calculate the amount of remaining space that can be
    /// distributed between all remaining columns.
    ///
    /// Take the current terminal width and
    /// - Subtract borders
    /// - Subtract padding
    /// - Subtract columns that already have a fixed width.
    fn new(
        table: &'a Table,
        infos: &'a mut DisplayInfos,
        visible_columns: usize,
        table_width: usize,
        max_content_widths: &'a [u16],
    ) -> Self {
        let border_count = count_border_columns(table, visible_columns);
        let mut remaining_width = table_width.saturating_sub(border_count);

        // Subtract all paddings from the remaining width.
        for column in table.columns.iter() {
            if infos.contains_key(&column.index) {
                continue;
            }

            // Remove the fixed padding for each column
            let (left, right) = column.padding;
            remaining_width = remaining_width.saturating_sub((left + right).into());
        }

        // Remove all already fixed sizes from the remaining_width.
        for info in infos.values() {
            if info.is_hidden {
                continue;
            }

            remaining_width = remaining_width.saturating_sub(info.width().into());
        }

        let remaining_columns = count_remaining_columns(visible_columns, infos);

        Self {
            table,
            infos,
            max_content_widths,
            remaining_width,
            remaining_columns,
            visible_columns,
        }
    }

    /// The average remaining space per undecided column.
    ///
    /// Returns 0 if no undecided columns remain.
    fn average_space(&self) -> usize {
        self.remaining_width
            .checked_div(self.remaining_columns)
            .unwrap_or(0)
    }

    /// Whether this column already has a decided width.
    fn is_decided(&self, column: &Column) -> bool {
        self.infos.contains_key(&column.index)
    }

    /// Fix `column` to `content_width` and adjust `remaining_width` and `remaining_columns` accordingly.
    ///
    /// Returns `false` when no undecided columns remain afterwards.
    fn fix_column(&mut self, column: &Column, content_width: u16) -> bool {
        let info = ColumnDisplayInfo::new(column, content_width);
        self.infos.insert(column.index, info);

        self.remaining_width = self.remaining_width.saturating_sub(content_width.into());
        self.remaining_columns -= 1;

        self.remaining_columns > 0
    }
}

/// Try to find the best fit for a given content and table_width
///
/// 1. Determine the amount of available space after applying fixed columns, padding, and borders.
/// 2. Now that we know how much space we have to work with, we have to check again for
///    LowerBoundary constraints. If there are any columns that have a higher LowerBoundary, we have
///    to fix that column to this size.
/// 3. Check if there are any columns that require less space than the average remaining space for
///    the remaining columns. (This includes the MaxWidth constraint).
/// 4. Take those columns, fix their size and add the surplus in space to the remaining space.
/// 5. Repeat step 2-3 until no columns with smaller size than average remaining space are left.
/// 6. At this point, the remaining spaces is equally distributed between all columns. It get's a
///    little tricky now. Check the documentation of [optimize_space_after_split] for more
///    information.
/// 7. Divide the remaining space in relatively equal chunks.
///
/// This breaks when:
///
/// 1. A user assigns fixed sizes to a few columns, which are larger than the terminal when
///    combined.
/// 2. A user provides more than 100% column width across a few columns.
pub fn arrange(
    table: &Table,
    infos: &mut DisplayInfos,
    visible_columns: usize,
    table_width: usize,
    max_content_widths: &[u16],
) {
    // Step 1
    // Find out how much space there is left.
    let mut state = ArrangementState::new(
        table,
        infos,
        visible_columns,
        table_width,
        max_content_widths,
    );

    #[cfg(feature = "_debug")]
    println!(
        "dynamic::arrange: Table width: {table_width}, Start remaining width {}",
        state.remaining_width
    );
    #[cfg(feature = "_debug")]
    println!("dynamic::arrange: Max content widths: {max_content_widths:#?}");

    // Step 2.
    //
    // Iterate through all undecided columns and enforce LowerBoundary constraints, if they're
    // bigger than the current average space.
    if state.remaining_columns > 0 {
        enforce_lower_boundary_constraints(&mut state);
    }

    // Step 3-5.
    // Find all columns that require less space than the average.
    find_columns_that_fit_into_average(&mut state);

    #[cfg(feature = "_debug")]
    {
        println!("After less than average: {:#?}", state.infos);
        println!(
            "Remaining width {}, column {}",
            state.remaining_width, state.remaining_columns
        );
    }

    // Step 6
    // All remaining columns should get an equal amount of remaining space.
    // However, we check if we can save some space after the content has been split.
    //
    // We only do this if there are remaining columns.
    if state.remaining_columns > 0 {
        // This is where Step 5 happens.
        optimize_space_after_split(&mut state);
    }

    #[cfg(feature = "_debug")]
    {
        println!("dynamic::arrange: After optimize: {:#?}", state.infos);
        println!(
            "dynamic::arrange: Remaining width {}, column {}",
            state.remaining_width, state.remaining_columns
        );
    }

    // Early exit and one branch of Part 7.
    //
    // All columns have been successfully assigned a width.
    // However, in case the user specified that the full terminal width should always be fully
    // utilized, we have to equally distribute the remaining space across all columns.
    if state.remaining_columns == 0 {
        if state.remaining_width > 0
            && matches!(table.arrangement, ContentArrangement::DynamicFullWidth)
        {
            use_full_width(&mut state);
            #[cfg(feature = "_debug")]
            println!("dynamic::arrange: After full width: {:#?}", state.infos);
        }
        return;
    }

    // Step 7. Equally distribute the remaining_width to all remaining columns
    // If we have less than one space per remaining column, give at least one space per column
    if state.remaining_width < state.remaining_columns {
        state.remaining_width = state.remaining_columns;
    }

    distribute_remaining_space(&mut state);

    #[cfg(feature = "_debug")]
    println!("dynamic::arrange: After distribute: {:#?}", state.infos);
}

/// Step 2-4
/// This function is part of the column width calculation process.
/// It checks if there are columns that take less space than there's currently available in average
/// for each column.
///
/// The algorithm is a while loop with a nested for loop.
/// 1. We iterate over all columns and check if there are columns that take less space.
/// 2. If we find one or more such columns, we fix their width and add the surplus space to the
///    remaining space. Due to this step, the average space per column increased. Now some other
///    column might be fixed in width as well.
/// 3. Do step 1 and 2, as long as there are columns left and as long as we find columns that take
///    up less space than the current remaining average.
fn find_columns_that_fit_into_average(state: &mut ArrangementState) {
    let mut found_smaller = true;
    while found_smaller {
        found_smaller = false;

        // There are no columns left to check. Proceed to the next step
        if state.remaining_columns == 0 {
            break;
        }

        // We have no space left, the terminal is either tiny or the other columns are huge.
        if state.average_space() == 0 {
            break;
        }

        for column in state.table.columns.iter() {
            // Ignore hidden columns
            // We already checked this column, skip it
            if state.is_decided(column) {
                continue;
            }

            let max_column_width = state.max_content_widths[column.index];

            // The column has a MaxWidth Constraint.
            // we can fix the column to this max_width and mark it as checked if these
            // two conditions are met:
            // - The average remaining space is bigger then the MaxWidth constraint.
            // - The actual max content of the column is bigger than the MaxWidth constraint.
            if let Some(max_width) =
                constraint::max(state.table, &column.constraint, state.visible_columns)
            {
                // Max/Min constraints always include padding!
                let average_space_with_padding =
                    state.average_space() + usize::from(column.padding_width());

                let width_with_padding = max_column_width + column.padding_width();
                // Check that both conditions mentioned above are met.
                if usize::from(max_width) <= average_space_with_padding
                    && width_with_padding >= max_width
                {
                    // Save the calculated info, this column has been handled.
                    let width = absolute_width_with_padding(column, max_width);

                    #[cfg(feature = "_debug")]
                    println!(
                        "dynamic::find_columns_that_fit_into_average: Fixed column {} via MaxWidth constraint with size {}, as it's bigger than average {}",
                        column.index,
                        width,
                        state.average_space()
                    );

                    // Continue with new recalculated width
                    if !state.fix_column(column, width) {
                        break;
                    }

                    found_smaller = true;
                    continue;
                }
            }

            // The column has a smaller or equal max_content_width than the average space.
            // Fix the width to max_content_width and mark it as checked
            if usize::from(max_column_width) <= state.average_space() {
                #[cfg(feature = "_debug")]
                println!(
                    "dynamic::find_columns_that_fit_into_average: Fixed column {} with size {}, as it's smaller than average {}",
                    column.index,
                    max_column_width,
                    state.average_space()
                );

                // Continue with new recalculated width
                if !state.fix_column(column, max_column_width) {
                    break;
                }

                found_smaller = true;
            }
        }
    }
}

/// Step 5
///
/// Determine, whether there are any columns that are allowed to occupy more width than the current
/// `average_space` via a [LowerBoundary] constraint.
///
/// These columns will then get fixed to the width specified in the [LowerBoundary] constraint.
///
/// I.e. if a column has to have at least 10 characters, but the average width left for a column is
/// only 6, we fix the column to this 10 character minimum!
fn enforce_lower_boundary_constraints(state: &mut ArrangementState) {
    // We loop this as long as we found a lower boundary that fixed a column in place.
    // Due to enforced lower boundaries, the `average_space` can shrink, which can result in
    // further lower boundaries being triggered.
    let mut try_again = true;
    while try_again {
        try_again = false;
        for column in state.table.columns.iter() {
            // Ignore hidden columns
            // We already checked this column, skip it
            if state.is_decided(column) {
                continue;
            }

            // Check whether the column has a LowerBoundary constraint.
            let Some(min_width) =
                constraint::min(state.table, &column.constraint, state.visible_columns)
            else {
                continue;
            };

            // Only proceed if the average spaces is smaller than the specified lower boundary.
            if state.average_space() >= min_width.into() {
                continue;
            }

            // This column would get smaller than the specified lower boundary.
            // Fix its width!!!
            let width = absolute_width_with_padding(column, min_width);

            #[cfg(feature = "_debug")]
            println!(
                "dynamic::enforce_lower_boundary_constraints: Fixed column {} to min constraint width {}",
                column.index, width
            );

            // Continue with new recalculated width
            if !state.fix_column(column, width) {
                break;
            }

            try_again = true;
        }
    }
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
fn optimize_space_after_split(state: &mut ArrangementState) {
    let mut found_smaller = true;

    #[cfg(feature = "_debug")]
    println!(
        "dynamic::optimize_space_after_split: Start with average_space {}",
        state.average_space()
    );

    // Do this as long as we find a smaller column
    while found_smaller {
        found_smaller = false;
        for column in state.table.columns.iter() {
            // We already checked this column, skip it
            if state.is_decided(column) {
                continue;
            }

            let average_space = state.average_space();
            let longest_line = longest_line_after_split(average_space, column, state.table);

            #[cfg(feature = "_debug")]
            println!(
                "dynamic::optimize_space_after_split: Longest line after split for column {} is {}",
                column.index, longest_line
            );

            // If there's a considerable amount of space left after splitting, we freeze the column
            // and set its content width to the calculated post-split width.
            let remaining_space = average_space.saturating_sub(longest_line);
            if remaining_space >= 3 {
                if !state.fix_column(column, longest_line.try_into().unwrap_or(u16::MAX)) {
                    break;
                }

                #[cfg(feature = "_debug")]
                println!(
                    "dynamic::optimize_space_after_split: average_space is now {}",
                    state.average_space()
                );
                found_smaller = true;
            }
        }
    }
}

/// Part of Step 5.
///
/// This function simulates the split of a Column's content and returns the length of
/// the longest existing line after the split.
///
/// A lot of this logic is duplicated from the [utils::format::format_row] function.
fn longest_line_after_split(average_space: usize, column: &Column, table: &Table) -> usize {
    // Runtime variable that holds the longest found line length.
    let mut longest = 0;

    for cell in table.column_cells_with_header_iter(column.index) {
        // Only look at rows that actually contain this cell.
        let Some(cell) = cell else { continue };

        let delimiter = delimiter(table, column, cell);

        // Create a temporary ColumnDisplayInfo with the average space as width.
        // That way we can simulate how the split text will look like.
        let info = ColumnDisplayInfo::new(column, average_space.try_into().unwrap_or(u16::MAX));

        // Iterate over each line and split it into multiple lines, if necessary.
        // Newlines added by the user will be preserved.
        for line in cell.content.iter() {
            if line.width() > average_space {
                let parts = split_line(line, &info, delimiter);

                #[cfg(feature = "_debug")]
                println!(
                    "dynamic::longest_line_after_split: Splitting line with width {}. Original:\n    {}\nSplitted:\n    {:?}",
                    line.width(),
                    line,
                    parts
                );

                parts
                    .iter()
                    .for_each(|part| longest = longest.max(part.len()));
            } else {
                longest = longest.max(line.len())
            }
        }
    }

    longest
}

/// Split `width` into `count` relatively equal chunks.
///
/// Since we do integer division, there is most likely a little bit of non equally-divisible
/// space. We then try to distribute it as fair as possible, which means that the first `width % count`
/// chunks get one extra space.
fn even_widths(width: usize, count: usize) -> impl Iterator<Item = u16> {
    let average_space = width / count;
    let excess = width - (average_space * count);

    (0..count).map(move |index| {
        let width = if index < excess {
            average_space + 1
        } else {
            average_space
        };
        width.try_into().unwrap_or(u16::MAX)
    })
}

/// Step 6 - First branch
///
/// At this point of time, all columns have been assigned some kind of width!
/// The user wants to utilize the full width of the terminal and there's space left.
///
/// Equally distribute the remaining space between all columns.
fn use_full_width(state: &mut ArrangementState) {
    let visible_columns = state.infos.values().filter(|info| !info.is_hidden).count();

    if visible_columns == 0 {
        return;
    }

    let mut widths = even_widths(state.remaining_width, visible_columns);
    for info in state.infos.values_mut() {
        // Ignore hidden columns
        if info.is_hidden {
            continue;
        }

        let width = widths.next().expect("One width per visible column");
        info.content_width += width;
    }
}

/// Step 6 - Second branch
///
/// Not all columns have a determined width yet -> The content still doesn't fully fit into the
/// given width.
///
/// This function now equally distributes the remaining width between the remaining columns.
fn distribute_remaining_space(state: &mut ArrangementState) {
    let mut widths = even_widths(state.remaining_width, state.remaining_columns);
    for column in state.table.columns.iter() {
        // Ignore hidden columns
        if state.is_decided(column) {
            continue;
        }

        let width = widths.next().expect("One width per remaining column");
        let info = ColumnDisplayInfo::new(column, width);
        state.infos.insert(column.index, info);
    }
}
