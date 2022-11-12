use comfy_table::Table;
use unicode_width::UnicodeWidthStr;

mod alignment_test;
#[cfg(feature = "tty")]
mod combined_test;
mod constraints_test;
mod content_arrangement_test;
mod custom_delimiter_test;
mod hidden_test;
#[cfg(feature = "ansi")]
mod inner_style_test;
mod modifiers_test;
mod padding_test;
mod presets_test;
mod property_test;
mod simple_test;
#[cfg(feature = "tty")]
mod styling_test;
mod utf_8_characters;

pub fn assert_table_line_width(table: &Table, count: usize) {
    for line in table.lines() {
        assert_eq!(line.width(), count);
    }
}
