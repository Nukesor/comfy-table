use criterion::{criterion_group, criterion_main, Criterion};

use comfy_table::presets::UTF8_FULL;
use comfy_table::Boundary::*;
use comfy_table::ColumnConstraint::*;
use comfy_table::*;

fn build_huge_table() {
    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL)
        .set_content_arrangement(ContentArrangement::DynamicFullWidth)
        .set_table_width(400)
        .set_header(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);

    // Create a 10x10 grid
    for row_index in 0..10 {
        let mut row = Vec::new();
        for column in 0..10 {
            row.push("SomeWord ".repeat((column + row_index) % 10))
        }
        table.add_row(row);
    }

    table.set_constraints(vec![
        UpperBoundary(Fixed(20)),
        LowerBoundary(Fixed(40)),
        Absolute(Fixed(5)),
        Absolute(Percentage(3)),
        Absolute(Percentage(3)),
        Boundaries {
            lower: Fixed(30),
            upper: Percentage(10),
        },
    ]);

    // Build the table.
    let _ = table.lines();
}

pub fn bench_build_huge_table(crit: &mut Criterion) {
    crit.bench_function("Huge table", |b| b.iter(|| build_huge_table()));
}

criterion_group!(benches, bench_build_huge_table);
criterion_main!(benches);
