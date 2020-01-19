use comfy_table::prelude::*;

#[test]
fn basic_table_works() {
    let header = Row::from(&vec!["Header1", "Header2", "Header3"]);
    let mut table = Table::new();
    table.set_header(header);

    let row = Row::from(&vec!["This is a text", "This is another text", "This is the third text"]);
    table.add_row(row);

    let row = Row::from(&vec!["This is another text", "asdfaes", "ag;aiufe"]);
    table.add_row(row);

    print!("{}", table.to_string());
}
