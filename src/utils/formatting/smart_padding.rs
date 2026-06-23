use crate::Table;
use crate::style::CellAlignment;
use crate::style::ContentArrangement;
use crate::style::TableComponent;
use crate::utils::ColumnDisplayInfo;
use crate::utils::arrangement::constraint;
use crate::utils::arrangement::helper::*;

/*
When table has a whitespace as border, it can look quite cramped,
especially when the cells have text with whitespaces. For example, below
is a sample output of a certain table:

     Line  User  Idle     Location
-- ------- ----- -------- --------------------------------------
   1 con 0 root  07:29:04 -
   2 vty 3 root  09:12:23 10.243.214.227
   3 vty 5 root  07:09:36 10.243.214.212
*  4 vty 7 admin 00:00:00 fdfd:5c41:712d:d08e:2ce2:7eff:fea5:ae5

As you can see most of the columns (especially the second and third) are
separated by just a single whitespace which makes it rather hard to read.

The idea of smart padding is to detect that two adjacent cells are not
separated by enough whitespace, in which case we add one more whitespace
between the two columns. With smart_padding_width=1 the above table would
look like:

     Line   User   Idle      Location
-- -------- ------ --------- --------------------------------------
   1 con 0  root   07:29:04  -
   2 vty 3  root   09:12:23  10.243.214.227
   3 vty 5  root   07:09:36  10.243.214.212
*  4 vty 7  admin  00:00:00  fdfd:5c41:712d:d08e:2ce2:7eff:fea5:ae5

It basically tries to make sure texts between adjacent cells have at
least specified number of whitespaces (excluding the border).

Smart padding does not take effect if one of the condition is true:
1. There is no extra space left
2. The left column is right aligned and the right column is left aligned
3. The vertical border is not ' '
 */

// this is similar to available_content_width in util/arrangement/dynamic.rs
fn available_width(table: &Table, infos: &Vec<&mut ColumnDisplayInfo>) -> u16 {
    let mut width = match table.width() {
        Some(width) => width as usize,
        // Disabled arrangement, in which case treat it as MAX
        None => return u16::MAX,
    };
    let visible_columns = infos.len();
    let border_count = count_border_columns(table, visible_columns);
    width = width.saturating_sub(border_count);

    // Remove all column widths; note all hidden columns are already filtered
    for info in infos.iter() {
        width = width.saturating_sub(info.width().into());
    }

    u16::try_from(width).unwrap()
}

// This returns current column display width (including padding)
fn current_column_width(info: &ColumnDisplayInfo) -> u16 {
    info.padding.0 + info.padding.1 + info.content_width
}

// return how much more padding is available for each column if max_size is specified
fn get_column_max_padding_widths(
    table: &Table,
    all_infos: &[ColumnDisplayInfo],
    visible_columns: usize,
) -> Vec<Option<u16>> {
    let mut max_padding_widths = Vec::with_capacity(visible_columns);
    for (column, info) in table.columns.iter().zip(all_infos.iter()) {
        if info.is_hidden {
            continue;
        }
        let max_width = constraint::max(table, &column.constraint, visible_columns);
        let max_padding_width = max_width.map(|v| v.saturating_sub(current_column_width(info)));
        max_padding_widths.push(max_padding_width);
    }
    max_padding_widths
}

// Whether we can pad this column on the left or right side
fn can_pad_column(info: &ColumnDisplayInfo, position: CellAlignment) -> bool {
    info.cell_alignment.unwrap_or(CellAlignment::Left) != position
}

// Given two adjacent cells, calculate the number of whitespaces between their texts.
// This does not include the border inbetween.
fn count_boundary_spaces(left: &str, right: &str) -> u16 {
    let trailing = left.chars().rev().take_while(|c| c.is_whitespace()).count();
    let leading = right.chars().take_while(|c| c.is_whitespace()).count();
    u16::try_from(trailing + leading).unwrap()
}

// Given a column_index, return extra padding we need to put between the two columns
fn compare_adjacent_columns(
    table: &Table,
    content: &[Vec<Vec<String>>],
    display_infos: &[&mut ColumnDisplayInfo],
    column_index: usize,
    max_padding_width: u16,
) -> u16 {
    if !(can_pad_column(display_infos[column_index], CellAlignment::Right)
        || can_pad_column(display_infos[column_index + 1], CellAlignment::Left))
    {
        return 0;
    }

    // find the most padding we need between any two adjacent cells
    let mut padding_width = 0;

    for (row_index, row) in content.iter().enumerate() {
        if row_index == 0 && table.header().is_some() {
            continue;
        }
        for sub_row in row.iter() {
            let cell_left = &sub_row[column_index];
            let cell_right = &sub_row[column_index + 1];

            let current_padding = count_boundary_spaces(cell_left, cell_right);
            let extra_padding = u16::saturating_sub(table.smart_padding_width(), current_padding);
            if extra_padding >= max_padding_width {
                // we'll need the max padding anyway
                return max_padding_width;
            }
            padding_width = padding_width.max(extra_padding);
        }
    }

    padding_width
}

// Update all cells in a certain column to add an extra padding
fn update_column_padding(
    content: &mut [Vec<Vec<String>>],
    display_infos: &mut [&mut ColumnDisplayInfo],
    max_padding_widths: &mut [Option<u16>],
    column_index: usize,
    padding_position: CellAlignment, // Left or Right
    padding_needed: u16,
) -> u16 {
    let display_info = &display_infos[column_index];
    if !can_pad_column(display_info, padding_position) {
        return 0;
    }

    let padding = padding_needed.min(max_padding_widths[column_index].unwrap_or(u16::MAX));

    let padding_str = " ".repeat(usize::from(padding));

    // adjust the padding to make the header line consistent
    display_infos[column_index].content_width += padding;
    if let Some(max_padding) = &mut max_padding_widths[column_index] {
        *max_padding = max_padding.saturating_sub(padding);
    }

    for row in content.iter_mut() {
        for sub_row in row.iter_mut() {
            match padding_position {
                CellAlignment::Left => {
                    sub_row[column_index] = format!("{}{}", padding_str, sub_row[column_index]);
                }
                CellAlignment::Right => {
                    sub_row[column_index] = format!("{}{}", sub_row[column_index], padding_str);
                }
                _ => {
                    unreachable!("Invalid padding position: {:?}", padding_position); // coverage:ignore-line
                }
            }
        }
    }
    padding
}

pub fn smart_pad_content(
    table: &Table,
    content: &mut [Vec<Vec<String>>],
    all_infos: &mut [ColumnDisplayInfo],
) {
    // Skip it for DynamicFullWidth as we most likely have enough padding
    match &table.arrangement {
        ContentArrangement::Disabled => (),
        ContentArrangement::Dynamic => (),
        _ => return,
    }

    if table.style_or_default(TableComponent::VerticalLines) != " " {
        // if we have vertical border, no need for this padding
        return;
    }

    if content.is_empty() || content[0].is_empty() {
        return;
    }

    // calculate the max width for each column has it may limit how much padding each
    // column might get
    let mut max_padding_widths =
        get_column_max_padding_widths(table, all_infos, content[0][0].len());

    // content only has visible data, while display_infos has hidden columns.
    // to be able to index into them correctly, let's just filter them.
    let mut display_infos: Vec<&mut ColumnDisplayInfo> = all_infos
        .iter_mut()
        .filter(|info| !info.is_hidden)
        .collect();

    assert_eq!(content[0][0].len(), display_infos.len());

    // get the overall available width for padding
    let mut remaining_width = available_width(table, &display_infos);

    #[cfg(feature = "_debug")]
    println!(
        "smartpad: available width {} visible columns {}",
        remaining_width,
        display_infos.len()
    );

    for column_index in 0..display_infos.len() - 1 {
        if remaining_width == 0 {
            return;
        }

        let max_padding_width = remaining_width
            .min(max_padding_widths[column_index].unwrap_or(u16::MAX))
            .max(max_padding_widths[column_index + 1].unwrap_or(u16::MAX));

        // how much more we can still pad
        let mut padding_needed = compare_adjacent_columns(
            table,
            content,
            &display_infos,
            column_index,
            max_padding_width,
        );

        if padding_needed == 0 {
            continue;
        }

        // We can either pad the left column or the right column
        // prioritize left/right aligned columns (if possible)
        // because padding center aligned column can distort the
        // alignment.

        let left_align = display_infos[column_index]
            .cell_alignment
            .unwrap_or(CellAlignment::Left);
        let right_align = display_infos[column_index + 1]
            .cell_alignment
            .unwrap_or(CellAlignment::Left);

        // By default pad left column first
        let mut pad_order = [0, 1];

        // we deprioritize center aligned as it makes the output a bit weird
        if left_align != CellAlignment::Left && right_align == CellAlignment::Right {
            pad_order[0] = 1;
            pad_order[1] = 0;
        }

        for pad_index in pad_order.iter() {
            #[cfg(feature = "_debug")]
            println!(
                "smartpad: column {} padding needed {}",
                column_index + *pad_index,
                padding_needed
            );

            let pad_position = if *pad_index == 0 {
                CellAlignment::Right
            } else {
                CellAlignment::Left
            };

            let padding_done = update_column_padding(
                content,
                &mut display_infos,
                &mut max_padding_widths,
                column_index + *pad_index,
                pad_position,
                padding_needed,
            );

            remaining_width -= padding_done;
            padding_needed -= padding_done;
        }
    }
}
