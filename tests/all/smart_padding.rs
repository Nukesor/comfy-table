use comfy_table::ColumnConstraint::*;
use comfy_table::*;
use pretty_assertions::assert_eq;

const DEFAULT_TABLE_WIDTH: Option<u16> = Some(100);

fn init_table(
    table: &mut Table,
    headers: Vec<&str>,
    arrangement: ContentArrangement,
    table_width: Option<u16>,
    smart_padding_width: u16,
) {
    table
        .set_content_arrangement(arrangement)
        .load_preset(comfy_table::presets::NOTHING)
        .set_style(comfy_table::TableComponent::HeaderLines, '-')
        .set_style(comfy_table::TableComponent::MiddleHeaderIntersections, ' ')
        .set_header(headers)
        .set_smart_padding_width(smart_padding_width);

    if let Some(table_width) = table_width {
        table.set_width(table_width);
    }

    // remove padding
    for column in table.column_iter_mut() {
        column.set_padding((0, 0));
    }
}

#[test]
fn login_example() {
    let mut table = Table::new();
    init_table(
        &mut table,
        vec!["  ", "Line", "User", "Idle", "Location"],
        ContentArrangement::Dynamic,
        DEFAULT_TABLE_WIDTH,
        1,
    );

    table
        .add_row(vec!["", "1 con 0", "root", "07:29:04", "-"])
        .add_row(vec!["", "2 vty 3", "root", "09:12:23", "10.243.214.227"])
        .add_row(vec!["", "3 vty 5", "root", "07:09:36", "10.243.214.212"])
        .add_row(vec![
            "*",
            "4 vty 7",
            "admin",
            "00:00:00",
            "fdfd:5c41:712d:d08e:2ce2:7eff:fea5:ae5",
        ]);

    table
        .column_mut(1)
        .unwrap()
        .set_cell_alignment(CellAlignment::Center);

    println!("{table}");

    let expected = [
        "     Line   User   Idle      Location                              ",
        "-- -------- ------ --------- --------------------------------------",
        "   1 con 0  root   07:29:04  -                                     ",
        "   2 vty 3  root   09:12:23  10.243.214.227                        ",
        "   3 vty 5  root   07:09:36  10.243.214.212                        ",
        "*  4 vty 7  admin  00:00:00  fdfd:5c41:712d:d08e:2ce2:7eff:fea5:ae5",
    ]
    .join("\n");

    assert_eq!(table.to_string(), expected);
}

fn basic_padding_test(arrangement: ContentArrangement, table_width: Option<u16>) {
    let mut table = Table::new();
    init_table(
        &mut table,
        vec![
            "Left", "Hidden", "Right", "Left", "Center", "Right", "Center",
        ],
        arrangement,
        table_width,
        1,
    );

    #[cfg(feature = "tty")]
    if table_width.is_none() {
        // this makes sure table.width() returns None
        table.force_no_tty();
    }

    table.add_row(vec![
        "left", "hidden", "right", "left", "center", "right", "center",
    ]);

    table
        .column_mut(1)
        .unwrap()
        .set_constraint(ColumnConstraint::Hidden);
    table
        .column_mut(2)
        .unwrap()
        .set_cell_alignment(CellAlignment::Right);
    table
        .column_mut(4)
        .unwrap()
        .set_cell_alignment(CellAlignment::Center);
    table
        .column_mut(5)
        .unwrap()
        .set_cell_alignment(CellAlignment::Right);
    table
        .column_mut(6)
        .unwrap()
        .set_cell_alignment(CellAlignment::Center);

    println!("{table}");

    let expected = [
        "Left  Right Left  Center  Right  Center",
        "----- ----- ----- ------ ------ -------",
        "left  right left  center  right  center",
    ]
    .join("\n");

    assert_eq!(table.to_string(), expected);
}

#[test]
fn basic_padding_dynamic() {
    basic_padding_test(ContentArrangement::Dynamic, DEFAULT_TABLE_WIDTH);
}

#[test]
fn basic_padding_disabled() {
    basic_padding_test(ContentArrangement::Disabled, DEFAULT_TABLE_WIDTH);
}

#[test]
fn basic_padding_disabled_no_width() {
    // this is mainly for code coverage - no table width + no tty -> no width limit
    basic_padding_test(ContentArrangement::Disabled, None);
}

#[test]
fn table_width_limit() {
    // verify smart padding is limited by table width
    let mut table = Table::new();

    init_table(
        &mut table,
        vec!["Header1", "Header2", "Header3"],
        ContentArrangement::Dynamic,
        Some(23),
        1,
    );
    table.add_row(vec!["header1", "header2", "header3"]);

    // no extra padding since the table has reached its limit
    let expected = [
        "Header1 Header2 Header3",
        "------- ------- -------",
        "header1 header2 header3",
    ]
    .join("\n");

    assert_eq!(table.to_string(), expected);
}

#[test]
fn column_width_limit() {
    let mut table = Table::new();
    init_table(
        &mut table,
        vec![
            "SinglePad",
            "Lower",
            "Upper",
            "DoublePad",
            "Both",
            "Absolute",
        ],
        ContentArrangement::Dynamic,
        DEFAULT_TABLE_WIDTH,
        1,
    );

    table
        .add_row(vec![
            "singlepad",
            "lower",
            "upper",
            "doublepad",
            "both",
            "absolute",
        ])
        .set_constraints(vec![
            UpperBoundary(Width::Fixed(10)), // doesn't matter
            LowerBoundary(Width::Fixed(5)),
            UpperBoundary(Width::Fixed(5)),
            UpperBoundary(Width::Percentage(90)), // doesn't matter
            Boundaries {
                lower: Width::Fixed(2),
                upper: Width::Fixed(4),
            },
            Absolute(Width::Fixed(8)),
        ]);

    table
        .column_mut(3)
        .unwrap()
        .set_cell_alignment(CellAlignment::Center);

    table
        .column_mut(5)
        .unwrap()
        .set_cell_alignment(CellAlignment::Right);

    println!("{table}");

    let expected = [
        "SinglePad  Lower  Upper  DoublePad  Both Absolute",
        "---------- ------ ----- ----------- ---- --------",
        "singlepad  lower  upper  doublepad  both absolute",
    ]
    .join("\n");

    assert_eq!(table.to_string(), expected);
}

#[test]
fn big_header() {
    let mut table = Table::new();
    init_table(
        &mut table,
        vec!["Big Header 1", "Big Header 2"],
        ContentArrangement::Dynamic,
        DEFAULT_TABLE_WIDTH,
        1,
    );

    table
        .add_row(vec!["1", "a"])
        .add_row(vec!["2", "b"])
        .add_row(vec!["3", "c"])
        .add_row(vec!["4", "d"]);

    println!("{table}");

    // No extra space for header
    let expected = [
        "Big Header 1 Big Header 2",
        "------------ ------------",
        "1            a           ",
        "2            b           ",
        "3            c           ",
        "4            d           ",
    ]
    .join("\n");

    assert_eq!(table.to_string(), expected);
}

#[test]
fn excessive_padding() {
    // test smart_padding_width > 1 and with existing static padding
    let mut table = Table::new();

    init_table(
        &mut table,
        vec!["Max=8", "Max=10", "Max=7"],
        ContentArrangement::Dynamic,
        DEFAULT_TABLE_WIDTH,
        4,
    );

    // add static padding
    for column in table.column_iter_mut() {
        column.set_padding((1, 1));
    }

    // set max size for the first two columns
    table.set_constraints(vec![
        UpperBoundary(Width::Fixed(8)),
        UpperBoundary(Width::Fixed(10)),
        UpperBoundary(Width::Fixed(7)),
    ]);
    // set the last column to be right aligned
    table
        .column_mut(1)
        .unwrap()
        .set_cell_alignment(CellAlignment::Center);
    table
        .column_mut(2)
        .unwrap()
        .set_cell_alignment(CellAlignment::Right);

    table.add_row(vec!["word8", "word10", "word7"]);

    // Max=8 gets no extra padding on the right
    // Max=10 gets one extra padding on the left and one on the right
    // Max=7 gets no extra padding
    let expected = [
        " Max=8     Max=10    Max=7 ",
        "-------- ---------- -------",
        " word8     word10    word7 ",
    ]
    .join("\n");

    assert_eq!(table.to_string(), expected);
}

#[test]
fn no_op_empty_table() {
    // test empty table
    let mut table = Table::new();

    init_table(
        &mut table,
        vec![],
        ContentArrangement::Dynamic,
        DEFAULT_TABLE_WIDTH,
        1,
    );

    assert_eq!(table.to_string(), "");
}

#[test]
fn no_op_dynamic_full_width() {
    // test DynamicFullWidth works as expected
    let mut table = Table::new();

    init_table(
        &mut table,
        vec!["Header1", "Header2"],
        ContentArrangement::DynamicFullWidth,
        Some(20),
        5,
    );

    table.add_row(vec!["header1", "header2"]);

    let expected = [
        "Header1    Header2  ",
        "---------- ---------",
        "header1    header2  ",
    ]
    .join("\n");

    assert_eq!(table.to_string(), expected);
}

#[test]
fn no_op_border_not_space() {
    // test smart-padding does not take effect when the border isn't space
    let mut table = Table::new();

    init_table(
        &mut table,
        vec!["Header1", "Header2"],
        ContentArrangement::Dynamic,
        DEFAULT_TABLE_WIDTH,
        10,
    );

    table.set_style(comfy_table::TableComponent::VerticalLines, '|');

    table.add_row(vec!["header1", "header2"]);

    let expected = ["Header1|Header2", "------- -------", "header1|header2"].join("\n");

    assert_eq!(table.to_string(), expected);
}
