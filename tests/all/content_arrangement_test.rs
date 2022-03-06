use pretty_assertions::assert_eq;

use comfy_table::ColumnConstraint::*;
use comfy_table::Width::*;
use comfy_table::{ContentArrangement, Row, Table};
use unicode_width::UnicodeWidthStr;

fn assert_table_lines(table: &Table, count: usize) {
    for line in table.lines() {
        assert_eq!(line.width(), count);
    }
}

#[test]
/// Test the robustnes of the dynamic table arangement.
fn simple_dynamic_table() {
    let mut table = Table::new();
    table.set_header(&vec!["Header1", "Header2", "Head"])
        .set_content_arrangement(ContentArrangement::Dynamic)
        .set_width(25)
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

    println!("{table}");
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
    println!("{expected}");
    assert!(table.lines().all(|line| line.width() == 25));
    assert_eq!("\n".to_string() + &table.to_string(), expected);
}

#[test]
/// Individual rows can be configured to have a max height.
/// Everything beyond that line height should be truncated.
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
        .set_width(35)
        .add_row(first_row)
        .add_row(second_row);

    // The first column will be wider than 6 chars.
    // The second column's content is wider than 6 chars. There should be a '...'.
    let second_column = table.column_mut(1).unwrap();
    second_column.set_constraint(Absolute(Fixed(8)));

    // The third column's content is less than 6 chars width. There shouldn't be a '...'.
    let third_column = table.column_mut(2).unwrap();
    third_column.set_constraint(Absolute(Fixed(7)));

    println!("{table}");
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
    println!("{expected}");
    assert_table_lines(&table, 35);
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
        .set_width(80)
        .add_row(&vec![
            "This is a very long line with a lot of text",
            "This is text with a anotherverylongtexttesttest",
            "smol",
        ]);

    println!("{table}");
    let expected = "
+-----------------------------------------+-----------------------------+------+
| Header1                                 | Header2                     | Head |
+==============================================================================+
| This is a very long line with a lot of  | This is text with a         | smol |
| text                                    | anotherverylongtexttesttest |      |
+-----------------------------------------+-----------------------------+------+";
    println!("{expected}");

    assert_table_lines(&table, 80);
    assert_eq!("\n".to_string() + &table.to_string(), expected);
}

#[test]
/// A single column get's split and a lot of the available isn't used afterward.
/// The remaining space should be cut away, making the table more compact.
fn unused_space_after_split() {
    let mut table = Table::new();
    table
        .set_header(&vec!["Header1"])
        .set_content_arrangement(ContentArrangement::Dynamic)
        .set_width(30)
        .add_row(&vec!["This is text with a anotherverylongtext"]);

    println!("{table}");
    let expected = "
+---------------------+
| Header1             |
+=====================+
| This is text with a |
| anotherverylongtext |
+---------------------+";
    println!("{expected}");
    assert_table_lines(&table, 23);
    assert_eq!("\n".to_string() + &table.to_string(), expected);
}

#[test]
/// The full width of a table should be used, even if the space isn't used.
fn dynamic_full_width_after_split() {
    let mut table = Table::new();
    table
        .set_header(&vec!["Header1"])
        .set_content_arrangement(ContentArrangement::DynamicFullWidth)
        .set_width(50)
        .add_row(&vec!["This is text with a anotherverylongtexttesttestaa"]);

    println!("{table}");
    let expected = "
+------------------------------------------------+
| Header1                                        |
+================================================+
| This is text with a                            |
| anotherverylongtexttesttestaa                  |
+------------------------------------------------+";
    println!("{expected}");
    assert_table_lines(&table, 50);
    assert_eq!("\n".to_string() + &table.to_string(), expected);
}

#[test]
/// This table checks the scenario, where a column has a big max_width, but a lot of the assigned
/// space isn't used after splitting the lines.
/// The remaining space should rather distributed between all cells.
fn dynamic_full_width() {
    let mut table = Table::new();
    table
        .set_header(&vec!["Header1", "Header2", "smol"])
        .set_content_arrangement(ContentArrangement::DynamicFullWidth)
        .set_width(80)
        .add_row(&vec!["This is a short line", "small", "smol"]);

    println!("{table}");
    let expected = "
+-----------------------------------+----------------------+-------------------+
| Header1                           | Header2              | smol              |
+==============================================================================+
| This is a short line              | small                | smol              |
+-----------------------------------+----------------------+-------------------+";
    println!("{expected}");
    assert_table_lines(&table, 80);
    assert_eq!("\n".to_string() + &table.to_string(), expected);
}
