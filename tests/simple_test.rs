use pretty_assertions::assert_eq;

use comfy_table::prelude::*;
use comfy_table::style::presets::*;

#[test]
fn simple_table() {
    let mut table = Table::new();
    table
        .set_header(&vec!["Header1", "Header2", "Header3"])
        .add_row(&vec![
            "This is a text",
            "This is another text",
            "This is the third text",
        ])
        .add_row(&vec![
            "This is another text",
            "Now\nadd some\nmulti line stuff",
            "This is awesome",
        ]);

    println!("{}", table.to_string());
    let expected = "
+----------------------+----------------------+------------------------+
| Header1              | Header2              | Header3                |
+======================================================================+
| This is a text       | This is another text | This is the third text |
|----------------------+----------------------+------------------------|
| This is another text | Now                  | This is awesome        |
|                      | add some             |                        |
|                      | multi line stuff     |                        |
+----------------------+----------------------+------------------------+";
    assert_eq!("\n".to_string() + &table.to_string(), expected);
}

#[test]
fn missing_column_table() {
    let mut table = Table::new();
    table
        .set_header(&vec!["Header1", "Header2", "Header3"])
        .add_row(&vec!["One One", "One Two", "One Three"])
        .add_row(&vec!["Two One", "Two Two"])
        .add_row(&vec!["Three One"]);

    println!("{}", table.to_string());
    let expected = "
+-----------+---------+-----------+
| Header1   | Header2 | Header3   |
+=================================+
| One One   | One Two | One Three |
|-----------+---------+-----------|
| Two One   | Two Two |           |
|-----------+---------+-----------|
| Three One |         |           |
+-----------+---------+-----------+";
    assert_eq!("\n".to_string() + &table.to_string(), expected);
}
