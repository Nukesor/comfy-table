use pretty_assertions::assert_eq;

use comfy_table::*;

#[test]
/// UTF-8 symbols that are longer than a single character are properly handled.
/// This means, that comfy-table detects that they're longer than 1 character and styles/arranges
/// the table accordingly.
fn multi_character_utf8_symbols() {
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
            "✅",
        ]);

    println!("{table}");
    let expected = "
+----------------------+----------------------+------------------------+
| Header1              | Header2              | Header3                |
+======================================================================+
| This is a text       | This is another text | This is the third text |
|----------------------+----------------------+------------------------|
| This is another text | Now                  | ✅                     |
|                      | add some             |                        |
|                      | multi line stuff     |                        |
+----------------------+----------------------+------------------------+";
    assert_eq!("\n".to_string() + &table.to_string(), expected);
}

#[test]
///
fn multi_character_utf8_word_splitting() {
    let mut table = Table::new();
    table
        .set_width(8)
        .set_content_arrangement(ContentArrangement::Dynamic)
        .set_header(&vec!["test"])
        .add_row(&vec!["abc✅def"]);

    println!("{table}");
    let expected = "
+------+
| test |
+======+
| abc  |
| ✅de |
| f    |
+------+";
    println!("{expected}");
    assert_eq!("\n".to_string() + &table.to_string(), expected);
}
