use pretty_assertions::assert_eq;

use comfy_table::{ColumnConstraint, ContentArrangement, Row, Table};

#[test]
fn simple_dynamic_table() {
    let mut table = Table::new();
    table.set_header(&vec!["Header1", "Header2", "Head"])
        .set_content_arrangement(ContentArrangement::Dynamic)
        .set_table_width(25)
        .add_row(&vec![
            "This is a very long line with a lot of text",
            "This is anotherverylongtextwithlongwords text",
            "smol",
        ])
        .add_row(&vec![
            "This is another text",
            "Now let's\nadd a really long line in the middle of the cell \n and add more multi line stuff",
            "smol",
        ]);

    println!("{}", table.to_string());
    let expected = "
+--------+-------+------+
| Header | Heade | Head |
| 1      | r2    |      |
+=======================+
| This   | This  | smol |
| is a   | is    |      |
| very   | anoth |      |
| long   | erver |      |
| line   | ylong |      |
| with a | textw |      |
| lot of | ithlo |      |
| text   | ngwor |      |
|        | ds    |      |
|        | text  |      |
|--------+-------+------|
| This   | Now   | smol |
| is ano | let's |      |
| ther   | add a |      |
| text   | reall |      |
|        | y     |      |
|        | long  |      |
|        | line  |      |
|        | in    |      |
|        | the   |      |
|        | middl |      |
|        | e of  |      |
|        | the   |      |
|        | cell  |      |
|        | and   |      |
|        | add   |      |
|        | more  |      |
|        | multi |      |
|        | line  |      |
|        | stuff |      |
+--------+-------+------+";
    assert_eq!("\n".to_string() + &table.to_string(), expected);
}

#[test]
fn table_with_truncate() {
    let mut table = Table::new();
    let mut first_row: Row = Row::from(vec![
        "This is a very long line with a lot of text",
        "This is anotherverylongtextwithlongwords text",
        "smol",
    ]);
    first_row.max_height(4);

    let mut second_row = Row::from(vec![
        "Now let's\nadd a really long line in the middle of the cell \n and add more multi line stuff",
        "This is another text",
        "smol",
    ]);
    second_row.max_height(4);

    table
        .set_header(&vec!["Header1", "Header2", "Head"])
        .set_content_arrangement(ContentArrangement::Dynamic)
        .set_table_width(35)
        .add_row(first_row)
        .add_row(second_row);

    // The first column will be wider than 6 chars.
    // The second column's content is wider than 6 chars. There should be a '...'.
    let second_column = table.get_column_mut(1).unwrap();
    second_column.set_constraint(ColumnConstraint::Width(8));

    // The third column's content is less than 6 chars width. There shouldn't be a '...'.
    let third_column = table.get_column_mut(2).unwrap();
    third_column.set_constraint(ColumnConstraint::Width(7));

    println!("{}", table.to_string());
    let expected = "
+----------------+--------+-------+
| Header1        | Header | Head  |
|                | 2      |       |
+=================================+
| This is a very | This   | smol  |
| long line with | is ano |       |
| a lot of text  | therve |       |
|                | ryl... |       |
|----------------+--------+-------|
| Now let's      | This   | smol  |
| add a really   | is ano |       |
| long line in   | ther   |       |
| the middle ... | text   |       |
+----------------+--------+-------+";
    assert_eq!("\n".to_string() + &table.to_string(), expected);
}

#[test]
/// This table checks the scenario, where a column has a big max_width, but a lot of the assigned
/// space doesn't get used after splitting the lines. This happens mostly when there are
/// many long words in a single column.
/// The remaining space should rather be distributed to other cells.
fn distribute_space_after_split() {
    let mut table = Table::new();
    table
        .set_header(&vec!["Header1", "Header2", "Head"])
        .set_content_arrangement(ContentArrangement::Dynamic)
        .set_table_width(80)
        .add_row(&vec![
            "This is a very long line with a lot of text",
            "This is text with a anotherverylongtexttesttest",
            "smol",
        ]);

    println!("{}", table.to_string());
    let expected = "
+-----------------------------------------+-----------------------------+------+
| Header1                                 | Header2                     | Head |
+==============================================================================+
| This is a very long line with a lot of  | This is text with a         | smol |
| text                                    | anotherverylongtexttesttest |      |
+-----------------------------------------+-----------------------------+------+";
    assert_eq!("\n".to_string() + &table.to_string(), expected);
}

#[test]
fn unused_space_after_split() {
    let mut table = Table::new();
    table
        .set_header(&vec!["Header1"])
        .set_content_arrangement(ContentArrangement::Dynamic)
        .set_table_width(50)
        .add_row(&vec!["This is text with a anotherverylongtexttesttestaa"]);

    println!("{}", table.to_string());
    let expected = "
+-------------------------------+
| Header1                       |
+===============================+
| This is text with a           |
| anotherverylongtexttesttestaa |
+-------------------------------+";
    assert_eq!("\n".to_string() + &table.to_string(), expected);
}

#[test]
fn dynamic_full_width() {
    let mut table = Table::new();
    table
        .set_header(&vec!["Header1"])
        .set_content_arrangement(ContentArrangement::DynamicFullWidth)
        .set_table_width(50)
        .add_row(&vec!["This is text with a anotherverylongtexttesttestaa"]);

    println!("{}", table.to_string());
    let expected = "
+------------------------------------------------+
| Header1                                        |
+================================================+
| This is text with a                            |
| anotherverylongtexttesttestaa                  |
+------------------------------------------------+";
    let table_text = table.to_string();
    assert_eq!("\n".to_string() + &table_text, expected);

    // Assert
    assert!(table_text.split('\n').all(|line| line.len() == 50));
}
