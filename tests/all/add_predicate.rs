use pretty_assertions::assert_eq;

use comfy_table::*;

#[test]
fn add_predicate_true() {
    let mut table = Table::new();
    table
        .set_header(&vec!["Header1", "Header2", "Header3"])
        .add_row(&vec![
            "This is a text",
            "This is another text",
            "This is the third text",
        ])
        .add_row_if(
            || true,
            &vec![
                "This is another text",
                "Now\nadd some\nmulti line stuff",
                "This is awesome",
            ],
        );

    println!("{table}");
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
fn add_predicate_false() {
    let mut table = Table::new();
    table
        .set_header(&vec!["Header1", "Header2", "Header3"])
        .add_row(&vec![
            "This is a text",
            "This is another text",
            "This is the third text",
        ])
        .add_row_if(
            || false,
            &vec![
                "This is another text",
                "Now\nadd some\nmulti line stuff",
                "This is awesome",
            ],
        );

    println!("{table}");
    let expected = "
+----------------+----------------------+------------------------+
| Header1        | Header2              | Header3                |
+================================================================+
| This is a text | This is another text | This is the third text |
+----------------+----------------------+------------------------+";
    assert_eq!("\n".to_string() + &table.to_string(), expected);
}

#[test]
fn add_predicate_mixed() {
    let mut table = Table::new();
    table
        .set_header(&vec!["Header1", "Header2", "Header3"])
        .add_row(&vec![
            "This is a text",
            "This is another text",
            "This is the third text",
        ])
        .add_row_if(
            || false,
            &vec!["I won't get displayed", "Me neither", "Same here!"],
        )
        .add_row_if(
            || true,
            &vec![
                "This is another text",
                "Now\nadd some\nmulti line stuff",
                "This is awesome",
            ],
        );

    println!("{table}");
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
