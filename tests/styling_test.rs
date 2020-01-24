use ::crossterm::style::{Attribute, Color};
use comfy_table::prelude::*;

#[test]
fn styled_table() {
    let mut table = Table::new();
    table.set_header(vec![
        Cell::new("Header1").add_attribute(Attribute::Bold),
        Cell::new("Header2").fg(Color::Green),
        Cell::new("Header3").bg(Color::Black),
    ]);

    table.add_row(vec![
        Cell::new("This is a bold text").add_attribute(Attribute::Bold),
        Cell::new("This is a green text").fg(Color::Green),
        Cell::new("This one has black background").bg(Color::Black),
    ]);
    table.add_row(vec![
        Cell::new("Blinking boiii").add_attribute(Attribute::SlowBlink),
        Cell::new("Now\nadd some\nmulti line stuff").fg(Color::Cyan),
        Cell::new("COMBINE ALL THE THINGS")
            .fg(Color::Green)
            .bg(Color::Black)
            .add_attribute(Attribute::Bold)
            .add_attribute(Attribute::SlowBlink),
    ]);

    println!("{}", table.to_string());
    let expected = "
+---------------------+----------------------+-------------------------------+
|\u{1b}[1m Header1             \u{1b}[0m|\u{1b}[38;5;10m Header2              \u{1b}[0m|\u{1b}[48;5;0m Header3                       \u{1b}[0m|
+============================================================================+
|\u{1b}[1m This is a bold text \u{1b}[0m|\u{1b}[38;5;10m This is a green text \u{1b}[0m|\u{1b}[48;5;0m This one has black background \u{1b}[0m|
|---------------------+----------------------+-------------------------------|
|\u{1b}[5m Blinking boiii      \u{1b}[0m|\u{1b}[38;5;14m Now                  \u{1b}[0m|\u{1b}[48;5;0m\u{1b}[38;5;10m\u{1b}[1m\u{1b}[5m COMBINE ALL THE THINGS        \u{1b}[0m|
|                     |\u{1b}[38;5;14m add some             \u{1b}[0m|                               |
|                     |\u{1b}[38;5;14m multi line stuff     \u{1b}[0m|                               |
+---------------------+----------------------+-------------------------------+";
    assert_eq!("\n".to_string() + &table.to_string(), expected);
}
