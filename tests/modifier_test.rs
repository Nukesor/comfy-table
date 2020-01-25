use comfy_table::prelude::*;
use comfy_table::style::presets::*;
use comfy_table::style::modifiers::*;

fn get_preset_table() -> Table {
    let mut table = Table::new();
    table.set_header(&vec!["Header1", "Header2", "Header3"]);
    table.add_row(&vec!["One One", "One Two", "One Three"]);
    table.add_row(&vec!["One One", "One Two", "One Three"]);

    table
}


#[test]
fn utf8_full() {
    let mut table = get_preset_table();
    table.load_preset(UTF8_FULL);
    table.apply_modifier(UTF8_ROUND_CORNERS);
    let expected = "
╭─────────┬─────────┬───────────╮
│ Header1 ┆ Header2 ┆ Header3   │
╞═════════╪═════════╪═══════════╡
│ One One ┆ One Two ┆ One Three │
├╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌┤
│ One One ┆ One Two ┆ One Three │
╰─────────┴─────────┴───────────╯";

    println!("{}", table.to_string());
    assert_eq!("\n".to_string() + &table.to_string(), expected);
}
