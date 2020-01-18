use crate::styling::table::{Component, TableStyle};
use crate::utils::arrangement::ColumnDisplayInfo;

pub fn draw_borders(
    rows: Vec<Vec<Vec<String>>>,
    table_style: &TableStyle,
    display_info: &Vec<ColumnDisplayInfo>,
) -> Vec<String> {
    let mut lines = Vec::new();
    if should_draw_top_border(table_style) {
        lines.push(draw_top_border(table_style, display_info));
    }

    lines.append(&mut draw_rows(rows, table_style, display_info));

    if should_draw_bottom_border(table_style) {
        lines.push(draw_bottom_border(table_style, display_info));
    }

    lines
}

fn draw_top_border(table_style: &TableStyle, display_info: &Vec<ColumnDisplayInfo>) -> String {
    let left_corner = table_style.style_or_default(Component::TopLeftCorner);
    let top_border = table_style.style_or_default(Component::TopBorder);
    let border_intersection = table_style.style_or_default(Component::TopBorderIntersections);
    let right_corner = table_style.style_or_default(Component::TopRightCorner);

    let mut line = String::new();
    // We only need the top left corner, if we need to draw a left border
    if should_draw_left_border(table_style) {
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
    if should_draw_right_border(table_style) {
        line += &right_corner;
    }

    line
}

fn draw_rows(
    rows: Vec<Vec<Vec<String>>>,
    table_style: &TableStyle,
    display_info: &Vec<ColumnDisplayInfo>,
) -> Vec<String> {
    let mut lines = Vec::new();
    // Iterate over all rows
    let mut row_iter = rows.iter().peekable();
    while let Some(row) = row_iter.next() {
        // Concatenate the line parts and insert the vertical borders if needed
        for line_parts in row.iter() {
            lines.push(embed_line(line_parts, table_style, display_info));
        }

        // Draw a horizontal line, if we should and if we aren't in the last line of the table.
        if row_iter.peek().is_some() && should_draw_horizontal_lines(table_style) {
            lines.push(draw_horizontal_lines(table_style, display_info));
        }
    }

    lines
}

// Takes the parts of a single line, surrounds them with borders and adds vertical lines.
fn embed_line(
    line_parts: &Vec<String>,
    table_style: &TableStyle,
    display_info: &Vec<ColumnDisplayInfo>,
) -> String {
    let vertical_lines = table_style.style_or_default(Component::VerticalLines);
    let left_border = table_style.style_or_default(Component::LeftBorder);
    let right_border = table_style.style_or_default(Component::RightBorder);

    let mut line = String::new();
    if should_draw_left_border(table_style) {
        line += &left_border;
    }

    let mut part_iter = line_parts.iter().peekable();
    while let Some(part) = part_iter.next() {
        line += part;
        if (should_draw_vertical_lines(table_style) && part_iter.peek().is_some()) {
            line += &vertical_lines;
        } else if should_draw_right_border(table_style) && !part_iter.peek().is_some() {
            line += &right_border;
        }
    }

    line
}

// The horizontal line that separates between rows.
fn draw_horizontal_lines(
    table_style: &TableStyle,
    display_info: &Vec<ColumnDisplayInfo>,
) -> String {
    let left_intersection = table_style.style_or_default(Component::LeftBorderIntersections);
    let horizontal_lines = table_style.style_or_default(Component::HorizontalLines);
    let middle_intersection = table_style.style_or_default(Component::MiddleIntersections);
    let right_intersection = table_style.style_or_default(Component::RightBorderIntersections);

    let mut line = String::new();
    // We only need the bottom left corner, if we need to draw a left border
    if should_draw_left_border(table_style) {
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
    if should_draw_right_border(table_style) {
        line += &right_intersection;
    }

    line
}

fn draw_bottom_border(
    table_style: &TableStyle,
    display_info: &Vec<ColumnDisplayInfo>,
) -> String {
    let left_corner = table_style.style_or_default(Component::BottomLeftCorner);
    let bottom_border = table_style.style_or_default(Component::BottomBorder);
    let middle_intersection =
        table_style.style_or_default(Component::BottomBorderIntersections);
    let right_corner = table_style.style_or_default(Component::BottomRightCorner);

    let mut line = String::new();
    // We only need the bottom left corner, if we need to draw a left border
    if should_draw_left_border(table_style) {
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
    if should_draw_right_border(table_style) {
        line += &right_corner;
    }

    line
}

fn should_draw_top_border(table_style: &TableStyle) -> bool {
    if table_style.style_exists(Component::TopLeftCorner)
        || table_style.style_exists(Component::TopBorder)
        || table_style.style_exists(Component::TopBorderIntersections)
        || table_style.style_exists(Component::TopRightCorner)
    {
        return true;
    }

    false
}

fn should_draw_bottom_border(table_style: &TableStyle) -> bool {
    if table_style.style_exists(Component::BottomLeftCorner)
        || table_style.style_exists(Component::BottomBorder)
        || table_style.style_exists(Component::BottomBorderIntersections)
        || table_style.style_exists(Component::BottomRightCorner)
    {
        return true;
    }

    false
}

fn should_draw_left_border(table_style: &TableStyle) -> bool {
    if table_style.style_exists(Component::TopLeftCorner)
        || table_style.style_exists(Component::LeftBorder)
        || table_style.style_exists(Component::LeftBorderIntersections)
        || table_style.style_exists(Component::BottomLeftCorner)
    {
        return true;
    }

    false
}

fn should_draw_right_border(table_style: &TableStyle) -> bool {
    if table_style.style_exists(Component::TopRightCorner)
        || table_style.style_exists(Component::RightBorder)
        || table_style.style_exists(Component::RightBorderIntersections)
        || table_style.style_exists(Component::BottomRightCorner)
    {
        return true;
    }

    false
}

fn should_draw_horizontal_lines(table_style: &TableStyle) -> bool {
    if table_style.style_exists(Component::LeftBorderIntersections)
        || table_style.style_exists(Component::HorizontalLines)
        || table_style.style_exists(Component::MiddleIntersections)
        || table_style.style_exists(Component::RightBorderIntersections)
    {
        return true;
    }

    false
}

fn should_draw_vertical_lines(table_style: &TableStyle) -> bool {
    if table_style.style_exists(Component::TopBorderIntersections)
        || table_style.style_exists(Component::HeaderMiddleIntersections)
        || table_style.style_exists(Component::VerticalLines)
        || table_style.style_exists(Component::MiddleIntersections)
        || table_style.style_exists(Component::BottomBorderIntersections)
    {
        return true;
    }

    false
}
