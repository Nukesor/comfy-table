use comfy_table::{Table, Row, Cell};
use console::Style;

fn main() {
    let mut table = Table::new();
    table.load_preset(comfy_table::presets::NOTHING);
    
    let mut row = Row::new();
    row.add_cell(Cell::new(
        "123 cell1"
    ));
    row.add_cell(Cell::new(
        "cell2"
    ));

    table.add_row(row);

    let mut row = Row::new();
    row.add_cell(Cell::new(
        format!("cell{}", console::style("3").bold().red())
    ));
    row.add_cell(Cell::new(
        "cell4"
    ));
    table.add_row(row);

    println!("{}", table);   
}