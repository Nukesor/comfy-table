use comfy_table::{Table, ContentArrangement};

#[test]
fn simple_dynamic_table() {
    let mut table = Table::new();
    table.set_header(&vec!["Header1", "Header2", "Header3"]);
    table.set_content_arrangement(ContentArrangement::Dynamic);
    table.set_table_width(25);

    table.add_row(&vec![
        "This is a very long line with a lot of text",
        "This is anotherverylongtextwithlongwords text",
        "smol",
    ]);
    table.add_row(&vec![
        "This is another text",
        "Now let's\nadd a really long line in the middle of the cell \n and add more multi line stuff",
        "smol",
    ]);

    println!("{}", table.to_string());
    let expected = "
+-------+-------+-------+
| Heade | Heade | Heade |
| r1    | r2    | r3    |
+=======================+
| This  | This  | smol  |
| is a  | is an |       |
| very  | other |       |
| long  | veryl |       |
| line  | ongte |       |
| with  | xtwit |       |
| a lot | hlong |       |
| of    | words |       |
| text  | text  |       |
|-------+-------+-------|
| This  | Now   | smol  |
| is an | let's |       |
| other | add a |       |
| text  | reall |       |
|       | y     |       |
|       | long  |       |
|       | line  |       |
|       | in    |       |
|       | the m |       |
|       | iddle |       |
|       | of    |       |
|       | the   |       |
|       | cell  |       |
|       | and   |       |
|       | add   |       |
|       | more  |       |
|       | multi |       |
|       | line  |       |
|       | stuff |       |
+-------+-------+-------+";
    assert_eq!("\n".to_string() + &table.to_string(), expected);
}
