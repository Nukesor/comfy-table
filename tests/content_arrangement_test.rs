use comfy_table::{ContentArrangement, Table};

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
| is a   | is an |      |
| very   | other |      |
| long   | veryl |      |
| line   | ongte |      |
| with a | xtwit |      |
| lot of | hlong |      |
| text   | words |      |
|        | text  |      |
|--------+-------+------|
| This   | Now   | smol |
| is ano | let\'s |      |
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
