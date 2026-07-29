use comfy_table::{presets::*, *};
use pretty_assertions::assert_eq;

fn get_preset_table() -> Table {
    let mut table = Table::new();
    table
        .set_header(vec!["Header1", "Header2", "Header3"])
        .add_row(vec!["One One", "One Two", "One Three"])
        .add_row(vec!["One One", "One Two", "One Three"]);

    table
}

#[test]
fn utf8_round_corners() {
    let mut table = get_preset_table();
    table.load_style(UTF8_FULL.with_rounded_corners());
    let expected = "
╭─────────┬─────────┬───────────╮
│ Header1 ┆ Header2 ┆ Header3   │
╞═════════╪═════════╪═══════════╡
│ One One ┆ One Two ┆ One Three │
├╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌┤
│ One One ┆ One Two ┆ One Three │
╰─────────┴─────────┴───────────╯";

    println!("{table}");
    assert_eq!(expected, "\n".to_string() + &table.to_string());
}

#[test]
fn utf8_solid_inner_borders() {
    let mut table = get_preset_table();
    table.load_style(UTF8_FULL.with_solid_inner_borders());
    let expected = "
┌─────────┬─────────┬───────────┐
│ Header1 │ Header2 │ Header3   │
╞═════════╪═════════╪═══════════╡
│ One One │ One Two │ One Three │
├─────────┼─────────┼───────────┤
│ One One │ One Two │ One Three │
└─────────┴─────────┴───────────┘";

    println!("{table}");
    assert_eq!(expected, "\n".to_string() + &table.to_string());
}
