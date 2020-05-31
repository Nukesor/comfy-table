use crate::column::Column;
use crate::style::ColumnConstraint::*;
use crate::style::{CellAlignment, ColumnConstraint, ContentArrangement};
use crate::table::Table;
use crate::utils::borders::{
    should_draw_left_border, should_draw_right_border, should_draw_vertical_lines,
};

/// This struct is ONLY used during when calling table.to_string()
/// It's used to store intermediate results, information on how to
/// arrange the table and other convenience variables for calculating
/// a lot of stuff.
/// This also exists to prevent changes on the original
/// Column struct while preparing to print the table as a string.
#[derive(Debug)]
pub struct ColumnDisplayInfo {
    pub padding: (u16, u16),
    /// The max amount of characters over all lines in this column
    max_content_width: u16,
    /// The actual allowed content width after arrangement
    content_width: u16,
    /// Flag that determines, if the content_width for this column
    /// has already been freezed.
    fixed: bool,
    /// A constraint that should be considered during dynamic arrangement
    pub constraint: Option<ColumnConstraint>,
    /// The content alignment of cells in this column
    pub cell_alignment: Option<CellAlignment>,
}

impl ColumnDisplayInfo {
    fn new(column: &Column) -> Self {
        ColumnDisplayInfo {
            padding: column.padding,
            max_content_width: column.max_content_width,
            content_width: 0,
            fixed: false,
            constraint: None::<ColumnConstraint>,
            cell_alignment: column.cell_alignment,
        }
    }
    pub fn padding_width(&self) -> u16 {
        self.padding.0 + self.padding.1
    }

    pub fn content_width(&self) -> u16 {
        self.content_width
    }

    pub fn set_content_width(&mut self, width: u16) {
        // Don't allow content widths of 0.
        if width == 0 {
            self.content_width = 1;

            return;
        }
        self.content_width = width;
    }

    pub fn max_width(&self) -> u16 {
        self.max_content_width + self.padding.0 + self.padding.1
    }

    pub fn width(&self) -> u16 {
        self.content_width + self.padding.0 + self.padding.1
    }

    /// Return the remaining value after subtracting the padding width.
    fn without_padding(&self, width: u16) -> u16 {
        let padding = self.padding_width();
        // Default minimum content width has to be 1
        if padding >= width {
            return 1;
        }

        width - padding
    }
}

/// Determine the width of each column depending on the content of the given table.
/// The results uses Option<usize>, since users can choose to hide columns.
pub fn arrange_content(table: &Table) -> Vec<ColumnDisplayInfo> {
    let table_width = table.get_table_width();
    let mut display_infos = Vec::new();
    for column in table.columns.iter() {
        let mut info = ColumnDisplayInfo::new(column);

        if let Some(constraint) = column.constraint.as_ref() {
            evaluate_constraint(&mut info, constraint, table_width);
        }

        display_infos.push(info);
    }

    // Fallback to Disabled, if we don't have any information on how wide the table should be.
    if table_width.is_none() {
        disabled_arrangement(&mut display_infos);
        return display_infos;
    }

    match &table.arrangement {
        ContentArrangement::Disabled => disabled_arrangement(&mut display_infos),
        ContentArrangement::Dynamic => {
            dynamic_arrangement(table, &mut display_infos, table_width.unwrap());
        }
    }

    display_infos
}

/// Look at given constraints of a column and populate the ColumnDisplayInfo depending on those.
fn evaluate_constraint(
    info: &mut ColumnDisplayInfo,
    constraint: &ColumnConstraint,
    table_width: Option<u16>,
) {
    match constraint {
        ContentWidth => {
            info.set_content_width(info.max_content_width);
            info.fixed = true;
        }
        Width(width) => {
            let width = info.without_padding(*width);
            info.set_content_width(width);
            info.fixed = true;
        }
        MinWidth(min_width) => {
            // In case a min_width is specified, we can already fix the size of the column
            // right now (since we already know the max_content_width.
            if info.max_width() <= *min_width {
                let width = info.without_padding(*min_width);
                info.set_content_width(width);
                info.fixed = true;
            }
        }
        MaxWidth(max_width) => info.constraint = Some(MaxWidth(*max_width)),
        Percentage(percent) => {
            if let Some(table_width) = table_width {
                let mut width = (table_width as i32 * *percent as i32 / 100) as u16;
                width = info.without_padding(width as u16);
                info.set_content_width(width);
                info.fixed = true;
            }
        }
        MinPercentage(percent) => {
            if let Some(table_width) = table_width {
                let min_width = (table_width as i32 * *percent as i32 / 100) as u16;
                if info.max_width() <= min_width {
                    let width = info.without_padding(min_width);
                    info.set_content_width(width);
                    info.fixed = true;
                }
            }
        }
        MaxPercentage(percent) => {
            if let Some(table_width) = table_width {
                let max_width = (table_width as i32 * *percent as i32 / 100) as u16;
                info.constraint = Some(MaxWidth(max_width));
            }
        }
    }
}

/// If dynamic arrangement is disabled, simply set the width of all columns
/// to the respective max content width.
fn disabled_arrangement(infos: &mut Vec<ColumnDisplayInfo>) {
    for info in infos.iter_mut() {
        if info.fixed {
            continue;
        }

        if let Some(ColumnConstraint::MaxWidth(max_width)) = info.constraint {
            if max_width < info.max_width() {
                let width = info.without_padding(max_width);
                info.set_content_width(width);
                info.fixed = true;
                continue;
            }
        }
        info.set_content_width(info.max_content_width);
        info.fixed = true;
    }
}

/// Try to find the best fit for a given content and table_width
///
/// 1. Determine all Columns that already have a fixed width and subtract it from remaining_width.\
/// 2. Check if there are any columns that require less space than the average
///    remaining space for remaining columns. (This includes the MaxWidth Constraint)
/// 3. Take those columns, fix their size and add the surplus in space to the remaining space
/// 4. Repeat step 2-3 until no columns with smaller size than average remaining space are left.
/// 5. Divide the remaining space in relatively equal chunks.
///
/// This breaks when:
///
/// 1. A user assigns more space to a few columns than there is on the terminal
/// 2. A user provides more than 100% column width over a few columns.
fn dynamic_arrangement(table: &Table, infos: &mut Vec<ColumnDisplayInfo>, table_width: u16) {
    // Convert to i32 to handle negative values in case we work with a very small terminal
    let mut remaining_width = table_width as i32;

    // Remove space occupied by borders from remaining_width
    if should_draw_left_border(table) {
        remaining_width -= 1;
    }
    if should_draw_right_border(table) {
        remaining_width -= 1;
    }
    if should_draw_vertical_lines(table) {
        remaining_width -= infos.len() as i32 - 1;
    }

    // All columns that have have been checked.
    let mut checked = Vec::new();

    // Step 1. Remove all already fixed sizes from the remaining_width
    for (id, info) in infos.iter().enumerate() {
        // This info already has a fixed width (by Constraint)
        // Subtract width from remaining_width and add to checked.
        if info.fixed {
            remaining_width -= info.width() as i32;
            checked.push(id);
        }
    }

    // Step 2-4. Find all columns that require less space than the average
    let mut found_smaller = true;
    while found_smaller {
        found_smaller = false;
        let remaining_columns = infos.len() - checked.len();

        // There are no columns left to check. Proceed to the next step
        if remaining_columns == 0 {
            break;
        }

        let average_space = remaining_width / remaining_columns as i32;
        // We have no space left, the terminal is either tiny or the other columns are huge.
        if average_space <= 0 {
            break;
        }

        for (id, info) in infos.iter_mut().enumerate() {
            // We already checked this column, skip it
            if checked.contains(&id) {
                continue;
            }

            // The column has a smaller MaxWidth Constraint than the average remaining space
            // and a higher max_content_width than it's constraint.
            // Fix the column width to max_width and mark it as checked.
            if let Some(ColumnConstraint::MaxWidth(max_width)) = info.constraint {
                if max_width as i32 <= average_space && info.max_width() >= max_width {
                    let width = info.without_padding(max_width);
                    info.set_content_width(width);
                    info.fixed = true;

                    remaining_width -= info.width() as i32;
                    checked.push(id);
                    found_smaller = true;
                    continue;
                }
            }

            // The column has a smaller max_content_width than the average space.
            // Fix the width to max_content_width and mark it as checked
            if (info.max_width() as i32) < average_space {
                info.set_content_width(info.max_content_width);
                info.fixed = true;

                remaining_width -= info.width() as i32;
                checked.push(id);
                found_smaller = true;
            }
        }
    }

    // Step 5. Equally distribute the remaining_width to all remaining columns
    let remaining_columns = infos.len() - checked.len();
    // We already managed to fix all.
    if remaining_columns == 0 {
        return;
    }

    // If we have less than one space per remaining column, give at least
    // one space per column
    if remaining_width < remaining_columns as i32 {
        remaining_width = remaining_columns as i32;
    }
    // Convert back to u16. We don't need the negative value handling any longer.
    let remaining_width = remaining_width as u16;

    let average_space = remaining_width / remaining_columns as u16;
    // Since we do integer division, there is most likely a little bit of lost space.
    // Calculate and try to distribute it as fair as possible (from left to right).
    let mut excess = remaining_width - (average_space * remaining_columns as u16);

    for (id, info) in infos.iter_mut().enumerate() {
        // We already checked this column, skip it
        if checked.contains(&id) {
            continue;
        }
        // Distribute the excess until nothing is left
        let mut width = if excess > 0 {
            excess -= 1;
            average_space + 1
        } else {
            average_space
        };

        width = info.without_padding(width);

        info.set_content_width(width);
        info.fixed = true;
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
        // The max_ width should just be copied from the column
        let max_widths: Vec<u16> = display_infos
            .iter()
            .map(|info| info.max_content_width)
            .collect();
        assert_eq!(max_widths, vec![4, 5, 6]);

        // In default mode without any constraints
        let widths: Vec<u16> = display_infos.iter().map(|info| info.width()).collect();
        assert_eq!(widths, vec![6, 7, 8]);
    }
}
