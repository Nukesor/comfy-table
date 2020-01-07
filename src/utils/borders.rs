use crate::styling::table::{Component, TableStyle};
use crate::utils::arrangement::ColumnDisplayInfo;

pub fn draw_borders(
    content: Vec<Vec<Vec<String>>>,
    table_style: &TableStyle,
    display_info: &Vec<ColumnDisplayInfo>,
) -> Vec<String> {
    let mut lines = Vec::new();
    if should_draw_top_border(table_style) {
        lines.push(draw_top_border(table_style, display_info));
    }

    lines.append(&mut draw_content(content, table_style, display_info));

    if should_draw_top_border(table_style) {
        lines.push(draw_bottom_border(table_style, display_info));
    }

    lines
}

pub fn draw_top_border(table_style: &TableStyle, display_info: &Vec<ColumnDisplayInfo>) -> String {
    let top_left_corner = table_style.style_or_default(Component::TopLeftCorner);
    let top_border = table_style.style_or_default(Component::TopBorder);
    let top_border_intersection = table_style.style_or_default(Component::TopBorderIntersections);
    let top_right_corner = table_style.style_or_default(Component::TopRightCorner);

    let mut line = String::new();
    // We only need the top left corner, if we need to draw a left border
    if should_draw_left_border(table_style) {
        line += &top_left_corner;
    }

    // Add the top border lines depending on column width
    // Also add the border intersections, if we haven't arrived at the last element yet
    let mut iter = display_info.iter().peekable();
    while let Some(info) = iter.next() {
        line += &top_border.repeat(info.width as usize);
        if iter.peek().is_some() {
            line += &top_border_intersection;
        }
    }

    // We only need the top right corner, if we need to draw a right border
    if should_draw_right_border(table_style) {
        line += &top_right_corner;
    }

    line
}


pub fn draw_content(
    content: Vec<Vec<Vec<String>>>,
    table_style: &TableStyle,
    display_info: &Vec<ColumnDisplayInfo>,
) -> Vec<String> {
    let lines = Vec::new();


    lines
}


pub fn draw_bottom_border(table_style: &TableStyle, display_info: &Vec<ColumnDisplayInfo>) -> String {
    let bottom_left_corner = table_style.style_or_default(Component::BottomLeftCorner);
    let bottom_border = table_style.style_or_default(Component::BottomBorder);
    let bottom_border_intersection = table_style.style_or_default(Component::BottomBorderIntersections);
    let bottom_right_corner = table_style.style_or_default(Component::BottomRightCorner);

    let mut line = String::new();
    // We only need the bottom left corner, if we need to draw a left border
    if should_draw_left_border(table_style) {
        line += &bottom_left_corner;
    }

    // Add the bottom border lines depending on column width
    // Also add the border intersections, if we haven't arrived at the last element yet
    let mut iter = display_info.iter().peekable();
    while let Some(info) = iter.next() {
        line += &bottom_border.repeat(info.width as usize);
        if iter.peek().is_some() {
            line += &bottom_border_intersection;
        }
    }

    // We only need the bottom right corner, if we need to draw a right border
    if should_draw_right_border(table_style) {
        line += &bottom_right_corner;
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
