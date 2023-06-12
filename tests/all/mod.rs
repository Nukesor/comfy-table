use comfy_table::Table;
use unicode_width::UnicodeWidthStr;

mod add_predicate;
mod alignment_test;
#[cfg(feature = "tty")]
mod combined_test;
mod constraints_test;
mod content_arrangement_test;
mod custom_delimiter_test;
mod hidden_test;
#[cfg(feature = "custom_styling")]
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
    // this function is only used in tests
    for line in table.lines().unwrap() {
        assert_eq!(line.width(), count);
    }
}
