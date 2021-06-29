use criterion::{criterion_group, criterion_main, Criterion};

use comfy_table::presets::UTF8_FULL;
use comfy_table::*;

fn build_readme_table() {
    let mut table = Table::new();
    table.load_preset(UTF8_FULL)
        .set_content_arrangement(ContentArrangement::Dynamic)
        .set_table_width(80)
        .set_header(vec![
            Cell::new("Header1").add_attribute(Attribute::Bold),
            Cell::new("Header2").fg(Color::Green),
            Cell::new("Header3"),
        ])
        .add_row(vec![
            Cell::new("This is a bold text").add_attribute(Attribute::Bold),
            Cell::new("This is a green text").fg(Color::Green),
            Cell::new("This one has black background").bg(Color::Black),
        ])
        .add_row(vec![
            Cell::new("Blinky boi").add_attribute(Attribute::SlowBlink),
            Cell::new("This table's content is dynamically arranged. The table is exactly 80 characters wide.\nHere comes a reallylongwordthatshoulddynamicallywrap"),
            Cell::new("COMBINE ALL THE THINGS")
            .fg(Color::Green)
            .bg(Color::Black)
            .add_attributes(vec![
                Attribute::Bold,
                Attribute::SlowBlink,
            ])
        ]);

    // Build the table.
    let _ = table.lines();
}

pub fn bench_build_readme_table(crit: &mut Criterion) {
    crit.bench_function("Readme table", |b| b.iter(|| build_readme_table()));
}

criterion_group!(benches, bench_build_readme_table);
criterion_main!(benches);
