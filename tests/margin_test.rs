use comfy_table::Table;

#[test]
fn fmt_with_margin() {
    let mut t = Table::new();
    t.set_header(&["heading 1", "heading 2", "heading 3"]);
    t.add_row(&["test 1,1", "test 1,2", "test 1,3"]);
    t.add_row(&["test 2,1", "test 2,2", "test 2,3"]);
    t.add_row(&["test 3,1", "test 3,2", "test 3,3"]);

    let actual = t.fmt_with_margin(4);
    let expected = "    +-----------+-----------+-----------+
    | heading 1 | heading 2 | heading 3 |
    +===================================+
    | test 1,1  | test 1,2  | test 1,3  |
    |-----------+-----------+-----------|
    | test 2,1  | test 2,2  | test 2,3  |
    |-----------+-----------+-----------|
    | test 3,1  | test 3,2  | test 3,3  |
    +-----------+-----------+-----------+";
    println!("Expected:\n{}\nActual: \n{}", &expected, &actual);
    assert_eq!(actual, expected);
}
