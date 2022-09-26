use comfy_table::{Table, Row, Cell, ContentArrangement};
use console::Style;

fn main() {
    let mut table = Table::new();
    table.load_preset(comfy_table::presets::NOTHING);
    table.set_content_arrangement(ContentArrangement::Dynamic);
    table.set_width(90);

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
        format!(r"cell sys-devices-pci0000:00-0000:00:010:07:00.1-usb2-2\x2d1-2\x2d1.3-2\x2d1.3:1.0-host2-target2:0:0-2:0:0:1-block-sdb{}", console::style(".device").bold().red())
    ));
    row.add_cell(Cell::new(
        "cell4 asdfasfsad asdfasdf sad fas df asdf as df asdf    asdfasdfasdfasdfasdfasdfa dsfa sdf asdf asd f asdf as df sadf asd fas df "
    ));
    table.add_row(row);

    let mut row = Row::new();
    row.add_cell(Cell::new(
        "cell5"
    ));
    row.add_cell(Cell::new(
        "cell6"
    ));
    table.add_row(row);

    println!("{}", table);   
}