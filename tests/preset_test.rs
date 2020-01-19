use comfy_table::prelude::*;
use comfy_table::styling::presets::*;

#[test]
fn presets_work() {
    let mut table = Table::new();
    table.set_header(&vec!["Header1", "Header2", "Header3"]);
    table.add_row(&vec![
        "One One",
        "One Two",
        "One Three",
    ]);
    table.add_row(&vec![
        "One One",
        "One Two",
        "One Three",
    ]);

    table.load_preset(ASCII_NO_BORDERS);
    let expected = concat!(
        " Header1 | Header2 | Header3   \n",
        "===============================\n",
        " One One | One Two | One Three \n",
        "---------+---------+-----------\n",
        " One One | One Two | One Three ",
    );
    assert_eq!(&table.to_string(), expected);


    table.load_preset(ASCII_BORDERS_ONLY);
    let expected = "
+-------------------------------+
| Header1   Header2   Header3   |
+===============================+
| One One   One Two   One Three |
|                               |
| One One   One Two   One Three |
+-------------------------------+";
    assert_eq!("\n".to_string() + &table.to_string(), expected);


    table.load_preset(ASCII_HORIZONTAL_BORDERS_ONLY);
    let expected = concat!(
        "-------------------------------\n",
        " Header1   Header2   Header3   \n",
        "===============================\n",
        " One One   One Two   One Three \n",
        "-------------------------------\n",
        " One One   One Two   One Three \n",
        "-------------------------------",
    );
    assert_eq!(&table.to_string(), expected);

    table.load_preset(UTF8_FULL);
    let expected = "
┌─────────┬─────────┬───────────┐
│ Header1 ┆ Header2 ┆ Header3   │
╞═════════╪═════════╪═══════════╡
│ One One ┆ One Two ┆ One Three │
├╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌┤
│ One One ┆ One Two ┆ One Three │
└─────────┴─────────┴───────────┘";
    assert_eq!("\n".to_string() + &table.to_string(), expected);

    table.load_preset(UTF8_BORDERS_ONLY);
    let expected = "
┌───────────────────────────────┐
│ Header1   Header2   Header3   │
╞═══════════════════════════════╡
│ One One   One Two   One Three │
│ One One   One Two   One Three │
└───────────────────────────────┘";
    assert_eq!("\n".to_string() + &table.to_string(), expected);
}
