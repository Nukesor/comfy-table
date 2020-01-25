use crate::table::Table;
use crate::style::table::Component;
use crate::utils::arrangement::ColumnDisplayInfo;

pub fn draw_borders(
    table: &Table,
    rows: Vec<Vec<Vec<String>>>,
    display_info: &Vec<ColumnDisplayInfo>,
) -> Vec<String> {
    let mut lines = Vec::new();
    if should_draw_top_border(table) {
        lines.push(draw_top_border(table, display_info));
    }

    lines.append(&mut draw_rows(rows, table, display_info));

    if should_draw_bottom_border(table) {
        lines.push(draw_bottom_border(table, display_info));
    }

    lines
}

fn draw_top_border(table: &Table, display_info: &Vec<ColumnDisplayInfo>) -> String {
    let left_corner = table.style_or_default(Component::TopLeftCorner);
    let top_border = table.style_or_default(Component::TopBorder);
    let border_intersection = table.style_or_default(Component::TopBorderIntersections);
    let right_corner = table.style_or_default(Component::TopRightCorner);

    let mut line = String::new();
    // We only need the top left corner, if we need to draw a left border
    if should_draw_left_border(table) {
        line += &left_corner;
    }

    // Add the top border lines depending on column width
    // Also add the border intersections, if we haven't arrived at the last element yet
    let mut iter = display_info.iter().peekable();
    while let Some(info) = iter.next() {
        line += &top_border.repeat(info.width as usize);
        if iter.peek().is_some() {
            line += &border_intersection;
        }
    }

    // We only need the top right corner, if we need to draw a right border
    if should_draw_right_border(table) {
        line += &right_corner;
    }

    line
}

fn draw_rows(
    rows: Vec<Vec<Vec<String>>>,
    table: &Table,
    display_info: &Vec<ColumnDisplayInfo>,
) -> Vec<String> {
    let mut lines = Vec::new();
    // Iterate over all rows
    let mut row_iter = rows.iter().enumerate().peekable();
    while let Some((row_index, row)) = row_iter.next() {
        // Concatenate the line parts and insert the vertical borders if needed
        for line_parts in row.iter() {
            lines.push(embed_line(line_parts, table));
        }

        // Draw the horizontal header line if desired, otherwise continue to the next iteration
        if row_index == 0 && table.header.is_some() {
            if should_draw_header(table) {
                lines.push(draw_horizontal_lines(table, display_info, true));
            }
            continue;
        }

        // Draw a horizontal line, if we desired and if we aren't in the last row of the table.
        if row_iter.peek().is_some() && should_draw_horizontal_lines(table) {
            lines.push(draw_horizontal_lines(table, display_info, false));
        }
    }

    lines
}

// Takes the parts of a single line, surrounds them with borders and adds vertical lines.
fn embed_line(line_parts: &Vec<String>, table: &Table) -> String {
    let vertical_lines = table.style_or_default(Component::VerticalLines);
    let left_border = table.style_or_default(Component::LeftBorder);
    let right_border = table.style_or_default(Component::RightBorder);

    let mut line = String::new();
    if should_draw_left_border(table) {
        line += &left_border;
    }

    let mut part_iter = line_parts.iter().peekable();
    while let Some(part) = part_iter.next() {
        line += part;
        if should_draw_vertical_lines(table) && part_iter.peek().is_some() {
            line += &vertical_lines;
        } else if should_draw_right_border(table) && !part_iter.peek().is_some() {
            line += &right_border;
        }
    }

    line
}

// The horizontal line that separates between rows.
fn draw_horizontal_lines(
    table: &Table,
    display_info: &Vec<ColumnDisplayInfo>,
    header: bool,
) -> String {
    let (left_intersection, horizontal_lines, middle_intersection, right_intersection) = if header {
        (
            table.style_or_default(Component::LeftHeaderIntersection),
            table.style_or_default(Component::HeaderLines),
            table.style_or_default(Component::MiddleHeaderIntersections),
            table.style_or_default(Component::RightHeaderIntersection),
        )
    } else {
        (
            table.style_or_default(Component::LeftBorderIntersections),
            table.style_or_default(Component::HorizontalLines),
            table.style_or_default(Component::MiddleIntersections),
            table.style_or_default(Component::RightBorderIntersections),
        )
    };

    let mut line = String::new();
    // We only need the bottom left corner, if we need to draw a left border
    if should_draw_left_border(table) {
        line += &left_intersection;
    }

    // Add the bottom border lines depending on column width
    // Also add the border intersections, if we haven't arrived at the last element yet
    let mut iter = display_info.iter().peekable();
    while let Some(info) = iter.next() {
        line += &horizontal_lines.repeat(info.width as usize);
        if iter.peek().is_some() {
            line += &middle_intersection;
        }
    }

    // We only need the bottom right corner, if we need to draw a right border
    if should_draw_right_border(table) {
        line += &right_intersection;
    }

    line
}

fn draw_bottom_border(table: &Table, display_info: &Vec<ColumnDisplayInfo>) -> String {
    let left_corner = table.style_or_default(Component::BottomLeftCorner);
    let bottom_border = table.style_or_default(Component::BottomBorder);
    let middle_intersection = table.style_or_default(Component::BottomBorderIntersections);
    let right_corner = table.style_or_default(Component::BottomRightCorner);

    let mut line = String::new();
    // We only need the bottom left corner, if we need to draw a left border
    if should_draw_left_border(table) {
        line += &left_corner;
    }

    // Add the bottom border lines depending on column width
    // Also add the border intersections, if we haven't arrived at the last element yet
    let mut iter = display_info.iter().peekable();
    while let Some(info) = iter.next() {
        line += &bottom_border.repeat(info.width as usize);
        if iter.peek().is_some() {
            line += &middle_intersection;
        }
    }

    // We only need the bottom right corner, if we need to draw a right border
    if should_draw_right_border(table) {
        line += &right_corner;
    }

    line
}

fn should_draw_top_border(table: &Table) -> bool {
    if table.style_exists(Component::TopLeftCorner)
        || table.style_exists(Component::TopBorder)
        || table.style_exists(Component::TopBorderIntersections)
        || table.style_exists(Component::TopRightCorner)
    {
        return true;
    }

    false
}

fn should_draw_bottom_border(table: &Table) -> bool {
    if table.style_exists(Component::BottomLeftCorner)
        || table.style_exists(Component::BottomBorder)
        || table.style_exists(Component::BottomBorderIntersections)
        || table.style_exists(Component::BottomRightCorner)
    {
        return true;
    }

    false
}

fn should_draw_left_border(table: &Table) -> bool {
    if table.style_exists(Component::TopLeftCorner)
        || table.style_exists(Component::LeftBorder)
        || table.style_exists(Component::LeftBorderIntersections)
        || table.style_exists(Component::LeftHeaderIntersection)
        || table.style_exists(Component::BottomLeftCorner)
    {
        return true;
    }

    false
}

fn should_draw_right_border(table: &Table) -> bool {
    if table.style_exists(Component::TopRightCorner)
        || table.style_exists(Component::RightBorder)
        || table.style_exists(Component::RightBorderIntersections)
        || table.style_exists(Component::RightHeaderIntersection)
        || table.style_exists(Component::BottomRightCorner)
    {
        return true;
    }

    false
}

fn should_draw_horizontal_lines(table: &Table) -> bool {
    if table.style_exists(Component::LeftBorderIntersections)
        || table.style_exists(Component::HorizontalLines)
        || table.style_exists(Component::MiddleIntersections)
        || table.style_exists(Component::RightBorderIntersections)
    {
        return true;
    }

    false
}

fn should_draw_vertical_lines(table: &Table) -> bool {
    if table.style_exists(Component::TopBorderIntersections)
        || table.style_exists(Component::MiddleHeaderIntersections)
        || table.style_exists(Component::VerticalLines)
        || table.style_exists(Component::MiddleIntersections)
        || table.style_exists(Component::BottomBorderIntersections)
    {
        return true;
    }

    false
}

fn should_draw_header(table: &Table) -> bool {
    if table.style_exists(Component::LeftHeaderIntersection)
        || table.style_exists(Component::HeaderLines)
        || table.style_exists(Component::MiddleHeaderIntersections)
        || table.style_exists(Component::RightHeaderIntersection)
    {
        return true;
    }

    false
}
