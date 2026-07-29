use std::iter::repeat_n;

use crate::{table::Table, utils::ColumnDisplayInfo};

pub(crate) fn draw_borders(
    table: &Table,
    rows: &[Vec<Vec<String>>],
    display_info: &[ColumnDisplayInfo],
) -> Vec<String> {
    // We know how many lines there should be. Initialize the vector with the rough correct amount.
    // We might over allocate a bit, but that's better than under allocating.
    let mut lines = if let Some(capacity) = rows.first().map(|lines| lines.len()) {
        // Lines * 2 -> Lines + delimiters
        // + 5 -> header delimiters + header + bottom/top borders
        Vec::with_capacity(capacity * 2 + 5)
    } else {
        Vec::new()
    };

    if table.style.has_top_border() {
        lines.push(draw_top_border(table, display_info));
    }

    draw_rows(&mut lines, rows, table, display_info);

    if table.style.has_bottom_border() {
        lines.push(draw_bottom_border(table, display_info));
    }

    lines
}

fn draw_top_border(table: &Table, display_info: &[ColumnDisplayInfo]) -> String {
    let left_corner = table.style.top_border.left.unwrap_or(' ');
    let top_border = table.style.top_border.fill.unwrap_or(' ');
    let intersection = table.style.top_border.junction.unwrap_or(' ');
    let right_corner = table.style.top_border.right.unwrap_or(' ');

    let mut line = String::new();
    // We only need the top left corner, if we need to draw a left border
    if should_draw_left_border(table) {
        line.push(left_corner);
    }

    // Build the top border line depending on the columns' width.
    // Also add the border intersections.
    let mut first = true;
    for info in display_info.iter() {
        // Only add something, if the column isn't hidden
        if !info.is_hidden {
            if !first {
                line.push(intersection);
            }
            line.extend(repeat_n(top_border, info.width().into()));
            first = false;
        }
    }

    // We only need the top right corner, if we need to draw a right border
    if should_draw_right_border(table) {
        line.push(right_corner);
    }

    line
}

fn draw_rows(
    lines: &mut Vec<String>,
    rows: &[Vec<Vec<String>>],
    table: &Table,
    display_info: &[ColumnDisplayInfo],
) {
    let draw_left_border = should_draw_left_border(table);
    let draw_right_border = should_draw_right_border(table);
    let draw_vertical_lines = should_draw_vertical_lines(table);

    // Iterate over all rows
    let mut row_iter = rows.iter().enumerate().peekable();
    while let Some((row_index, row)) = row_iter.next() {
        // Styling depends on whether we're currently in the header or not.
        let style = if row_index == 0 && table.header.is_some() {
            table.style.header_lines
        } else {
            table.style.content_lines
        };
        let left_border = style.left.unwrap_or(' ');
        let vertical_lines = style.junction.unwrap_or(' ');
        let right_border = style.right.unwrap_or(' ');

        // Concatenate the line parts and insert the vertical borders if needed
        for line_parts in row.iter() {
            let mut line = String::new();
            if draw_left_border {
                line.push(left_border);
            }

            let mut part_iter = line_parts.iter().peekable();
            while let Some(part) = part_iter.next() {
                line += part;
                if part_iter.peek().is_none() && draw_right_border {
                    line.push(right_border);
                } else if part_iter.peek().is_some() && draw_vertical_lines {
                    line.push(vertical_lines);
                }
            }

            lines.push(line);
        }

        // Draw the horizontal header line if desired, otherwise continue to the next iteration
        if row_index == 0 && table.header.is_some() {
            if table.style.has_header_separator() {
                lines.push(draw_horizontal_lines(table, display_info, true));
            }
            continue;
        }

        // Draw a horizontal line, if we desired and if we aren't in the last row of the table.
        if row_iter.peek().is_some() && table.style.has_row_separator() {
            lines.push(draw_horizontal_lines(table, display_info, false));
        }
    }
}

// The horizontal line that separates between rows.
fn draw_horizontal_lines(
    table: &Table,
    display_info: &[ColumnDisplayInfo],
    header: bool,
) -> String {
    // Styling depends on whether we're currently on the header line or not.
    let separator = if header {
        table.style.header_separator
    } else {
        table.style.row_separator
    };
    let left_intersection = separator.left.unwrap_or(' ');
    let horizontal_lines = separator.fill.unwrap_or(' ');
    let middle_intersection = separator.junction.unwrap_or(' ');
    let right_intersection = separator.right.unwrap_or(' ');

    let mut line = String::new();
    // We only need the bottom left corner, if we need to draw a left border
    if should_draw_left_border(table) {
        line.push(left_intersection);
    }

    let draw_vertical_lines = should_draw_vertical_lines(table);

    // Append the middle lines depending on the columns' widths.
    // Also add the middle intersections.
    let mut first = true;
    for info in display_info.iter() {
        // Only add something, if the column isn't hidden
        if !info.is_hidden {
            if !first && draw_vertical_lines {
                line.push(middle_intersection);
            }
            line.extend(repeat_n(horizontal_lines, info.width().into()));
            first = false;
        }
    }

    // We only need the bottom right corner, if we need to draw a right border
    if should_draw_right_border(table) {
        line.push(right_intersection);
    }

    line
}

fn draw_bottom_border(table: &Table, display_info: &[ColumnDisplayInfo]) -> String {
    let left_corner = table.style.bottom_border.left.unwrap_or(' ');
    let bottom_border = table.style.bottom_border.fill.unwrap_or(' ');
    let middle_intersection = table.style.bottom_border.junction.unwrap_or(' ');
    let right_corner = table.style.bottom_border.right.unwrap_or(' ');

    let mut line = String::new();
    // We only need the bottom left corner, if we need to draw a left border
    if should_draw_left_border(table) {
        line.push(left_corner);
    }

    // Add the bottom border lines depending on column width
    // Also add the border intersections.
    let mut first = true;
    for info in display_info.iter() {
        // Only add something, if the column isn't hidden
        if !info.is_hidden {
            if !first {
                line.push(middle_intersection);
            }
            line.extend(repeat_n(bottom_border, info.width().into()));
            first = false;
        }
    }

    // We only need the bottom right corner, if we need to draw a right border
    if should_draw_right_border(table) {
        line.push(right_corner);
    }

    line
}

pub fn should_draw_left_border(table: &Table) -> bool {
    table.style.has_left_border()
}

pub fn should_draw_right_border(table: &Table) -> bool {
    table.style.has_right_border()
}

pub fn should_draw_vertical_lines(table: &Table) -> bool {
    table.style.has_vertical_lines()
}
