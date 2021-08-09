use pretty_assertions::assert_eq;

use comfy_table::presets::*;
use comfy_table::*;

fn get_preset_table() -> Table {
    let mut table = Table::new();
    table
        .set_header(&vec!["Hello", "there"])
        .add_row(&vec!["a", "b"])
        .add_row(&vec!["c", "d"]);

    table
}

#[test]
fn test_ascii_full() {
    let mut table = get_preset_table();
    table.load_preset(ASCII_FULL);
    println!("{}", table.to_string());
    let expected = "
+-------+-------+
| Hello | there |
+===============+
| a     | b     |
|-------+-------|
| c     | d     |
+-------+-------+";
    println!("{}", expected);
    assert_eq!("\n".to_string() + &table.to_string(), expected);
}

#[test]
fn test_ascii_no_borders() {
    let mut table = get_preset_table();
    table.load_preset(ASCII_NO_BORDERS);
    println!("{}", table.to_string());
    let expected = "
 Hello | there
===============
 a     | b
-------+-------
 c     | d";
    println!("{}", expected);
    assert_eq!("\n".to_string() + &table.trim_fmt(), expected);
}

#[test]
fn test_ascii_borders_only() {
    let mut table = get_preset_table();
    table.load_preset(ASCII_BORDERS_ONLY);
    println!("{}", table.to_string());
    let expected = "
+---------------+
| Hello   there |
+===============+
| a       b     |
|               |
| c       d     |
+---------------+";
    println!("{}", expected);
    assert_eq!("\n".to_string() + &table.to_string(), expected);
}

#[test]
fn test_ascii_borders_only_condensed() {
    let mut table = get_preset_table();
    table.load_preset(ASCII_BORDERS_ONLY_CONDENSED);
    println!("{}", table.to_string());
    let expected = "
+---------------+
| Hello   there |
+===============+
| a       b     |
| c       d     |
+---------------+";
    println!("{}", expected);
    assert_eq!("\n".to_string() + &table.to_string(), expected);
}

#[test]
fn test_ascii_horizontal_only() {
    let mut table = get_preset_table();
    table.load_preset(ASCII_HORIZONTAL_ONLY);
    println!("{}", table.to_string());
    let expected = "
---------------
 Hello   there
===============
 a       b
---------------
 c       d
---------------";
    println!("{}", expected);
    assert_eq!("\n".to_string() + &table.trim_fmt(), expected);
}

#[test]
fn test_ascii_markdown() {
    let mut table = get_preset_table();
    table.load_preset(ASCII_MARKDOWN);
    println!("{}", table.to_string());
    let expected = "
| Hello | there |
|-------|-------|
| a     | b     |
| c     | d     |";
    println!("{}", expected);
    assert_eq!("\n".to_string() + &table.trim_fmt(), expected);
}

#[test]
fn test_utf8_full() {
    let mut table = get_preset_table();
    table.load_preset(UTF8_FULL);
    println!("{}", table.to_string());
    let expected = "
┌───────┬───────┐
│ Hello ┆ there │
╞═══════╪═══════╡
│ a     ┆ b     │
├╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌┤
│ c     ┆ d     │
└───────┴───────┘";
    println!("{}", expected);
    assert_eq!("\n".to_string() + &table.trim_fmt(), expected);
}

#[test]
fn test_utf8_no_borders() {
    let mut table = get_preset_table();
    table.load_preset(UTF8_NO_BORDERS);
    println!("{}", table.to_string());
    let expected = "
 Hello ┆ there
═══════╪═══════
 a     ┆ b
╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌
 c     ┆ d";
    println!("{}", expected);
    assert_eq!("\n".to_string() + &table.trim_fmt(), expected);
}

#[test]
fn test_utf8_horizontal_only() {
    let mut table = get_preset_table();
    table.load_preset(UTF8_HORIZONTAL_ONLY);
    println!("{}", table.to_string());
    let expected = "
───────────────
 Hello   there
═══════════════
 a       b
───────────────
 c       d
───────────────";
    println!("{}", expected);
    assert_eq!("\n".to_string() + &table.trim_fmt(), expected);
}

#[test]
fn test_nothing() {
    let mut table = get_preset_table();
    table.load_preset(NOTHING);
    println!("{}", table.to_string());
    let expected = "
 Hello  there
 a      b
 c      d";
    println!("{}", expected);
    assert_eq!("\n".to_string() + &table.trim_fmt(), expected);
}