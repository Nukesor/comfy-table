use pretty_assertions::assert_eq;

use comfy_table::*;

#[test]
fn simple_colspan() {
    let mut table = Table::new();
    table
        .set_header(vec![
            Cell::new("Header1").set_colspan(2),
            Cell::new("Header3"),
        ])
        .add_row(vec![
            Cell::new("Spans 2 cols").set_colspan(2),
            Cell::new("Normal cell"),
        ]);

    let expected = "
+----------+----------+-------------+
| Header1             | Header3     |
+===================================+
| Spans 2 cols        | Normal cell |
+----------+----------+-------------+";
    assert_eq!(expected, "\n".to_string() + &table.to_string());
}

#[test]
fn simple_rowspan() {
    let mut table = Table::new();
    table
        .set_header(vec!["Header1", "Header2", "Header3"])
        .add_row(vec![
            Cell::new("Spans 2 rows").set_rowspan(2),
            Cell::new("Cell 2"),
            Cell::new("Cell 3"),
        ])
        .add_row(vec![
            // First position is occupied by rowspan above, so we only add 2 cells
            Cell::new("Cell 2 (row 2)"),
            Cell::new("Cell 3 (row 2)"),
        ]);

    let expected = "
+--------------+----------------+----------------+
| Header1      | Header2        | Header3        |
+================================================+
| Spans 2 rows | Cell 2         | Cell 3         |
|              +----------------+----------------|
|              | Cell 2 (row 2) | Cell 3 (row 2) |
+--------------+----------------+----------------+";
    assert_eq!(expected, "\n".to_string() + &table.to_string());
}

#[test]
fn combined_colspan_rowspan() {
    let mut table = Table::new();
    table
        .set_header(vec!["Header1", "Header2", "Header3", "Header4"])
        .add_row(vec![
            Cell::new("Spans 2x2").set_colspan(2).set_rowspan(2),
            Cell::new("Cell 3"),
            Cell::new("Cell 4"),
        ])
        .add_row(vec![
            // First 2 positions are occupied by rowspan above
            Cell::new("Cell 3 (row 2)"),
            Cell::new("Cell 4 (row 2)"),
        ]);

    let expected = "
+---------+---------+----------------+----------------+
| Header1 | Header2 | Header3        | Header4        |
+=====================================================+
| Spans 2x2         | Cell 3         | Cell 4         |
|                   +----------------+----------------|
|                   | Cell 3 (row 2) | Cell 4 (row 2) |
+---------+---------+----------------+----------------+";
    assert_eq!(expected, "\n".to_string() + &table.to_string());
}

#[test]
fn multiple_spans_in_row() {
    let mut table = Table::new();
    table
        .set_header(vec!["H1", "H2", "H3", "H4", "H5"])
        .add_row(vec![
            Cell::new("Span 2").set_colspan(2),
            Cell::new("Normal"),
            Cell::new("Span 2").set_colspan(2),
        ]);

    let expected = "
+------+------+--------+------+------+
| H1   | H2   | H3     | H4   | H5   |
+====================================+
| Span 2      | Normal | Span 2      |
+------+------+--------+------+------+";
    assert_eq!(expected, "\n".to_string() + &table.to_string());
}

#[test]
fn spans_in_header() {
    let mut table = Table::new();
    table
        .set_header(vec![
            Cell::new("Header spans 2").set_colspan(2),
            Cell::new("Header3"),
        ])
        .add_row(vec![
            Cell::new("Cell 1"),
            Cell::new("Cell 2"),
            Cell::new("Cell 3"),
        ])
        .add_row(vec![
            Cell::new("Cell 1"),
            Cell::new("Cell 2"),
            Cell::new("Cell 3"),
        ]);

    let expected = "
+-----------+-----------+---------+
| Header spans 2        | Header3 |
+=================================+
| Cell 1    | Cell 2    | Cell 3  |
|-----------+-----------+---------|
| Cell 1    | Cell 2    | Cell 3  |
+-----------+-----------+---------+";
    assert_eq!(expected, "\n".to_string() + &table.to_string());
}

#[test]
fn spans_with_multiline_content() {
    let mut table = Table::new();
    table
        .set_header(vec!["Header1", "Header2"])
        .add_row(vec![
            Cell::new("This is a\nmulti-line\ncell content").set_colspan(2),
        ])
        .add_row(vec![Cell::new("Cell 1"), Cell::new("Cell 2")]);

    let expected = "
+---------+---------+
| Header1 | Header2 |
+===================+
| This is a         |
| multi-line        |
| cell content      |
|-------------------|
| Cell 1  | Cell 2  |
+---------+---------+";
    assert_eq!(expected, "\n".to_string() + &table.to_string());
}

#[test]
fn spans_with_different_alignments() {
    let mut table = Table::new();
    table
        .set_header(vec!["H1", "H2", "H3"])
        .add_row(vec![
            Cell::new("Left")
                .set_colspan(2)
                .set_alignment(CellAlignment::Left),
            Cell::new("Right"),
        ])
        .add_row(vec![
            Cell::new("Center")
                .set_colspan(2)
                .set_alignment(CellAlignment::Center),
            Cell::new("Right"),
        ])
        .add_row(vec![
            Cell::new("Right")
                .set_colspan(2)
                .set_alignment(CellAlignment::Right),
            Cell::new("Right"),
        ]);

    let expected = "
+------+------+-------+
| H1   | H2   | H3    |
+=====================+
| Left        | Right |
|-------------+-------|
|    Center   | Right |
|-------------+-------|
|       Right | Right |
+------+------+-------+";
    assert_eq!(expected, "\n".to_string() + &table.to_string());
}

#[cfg(feature = "tty")]
#[test]
fn spans_with_styling() {
    let mut table = Table::new();
    table.set_header(vec!["H1", "H2", "H3"]).add_row(vec![
        Cell::new("Styled span")
            .set_colspan(2)
            .fg(Color::Red)
            .add_attribute(Attribute::Bold),
        Cell::new("Normal"),
    ]);
    table.force_no_tty();

    let expected = "
+---------+--------+--------+
| H1      | H2     | H3     |
+===========================+
| Styled span      | Normal |
+---------+--------+--------+";
    assert_eq!(expected, "\n".to_string() + &table.to_string());
}

#[test]
fn span_at_table_boundaries() {
    let mut table = Table::new();
    table
        .set_header(vec!["H1", "H2"])
        .add_row(vec![Cell::new("Spans all").set_colspan(2)]);

    let expected = "
+--------+-------+
| H1     | H2    |
+================+
| Spans all      |
+--------+-------+";
    assert_eq!(expected, "\n".to_string() + &table.to_string());
}

#[test]
fn large_colspan() {
    let mut table = Table::new();
    table
        .set_header(vec!["H1", "H2", "H3", "H4", "H5"])
        .add_row(vec![Cell::new("Spans 5 cols").set_colspan(5)]);

    let expected = "
+-----+-----+-----+-----+----+
| H1  | H2  | H3  | H4  | H5 |
+============================+
| Spans 5 cols               |
+-----+-----+-----+-----+----+";
    assert_eq!(expected, "\n".to_string() + &table.to_string());
}

#[test]
fn large_rowspan() {
    let mut table = Table::new();
    table
        .set_header(vec!["H1", "H2", "H3"])
        .add_row(vec![
            Cell::new("Spans 3 rows").set_rowspan(3),
            Cell::new("Cell 2"),
            Cell::new("Cell 3"),
        ])
        .add_row(vec![
            // First position is occupied by rowspan above
            Cell::new("Cell 2 (row 2)"),
            Cell::new("Cell 3 (row 2)"),
        ])
        .add_row(vec![
            // First position is still occupied by rowspan
            Cell::new("Cell 2 (row 3)"),
            Cell::new("Cell 3 (row 3)"),
        ]);

    let expected = "
+--------------+----------------+----------------+
| H1           | H2             | H3             |
+================================================+
| Spans 3 rows | Cell 2         | Cell 3         |
|              +----------------+----------------|
|              | Cell 2 (row 2) | Cell 3 (row 2) |
|              +----------------+----------------|
|              | Cell 2 (row 3) | Cell 3 (row 3) |
+--------------+----------------+----------------+";
    assert_eq!(expected, "\n".to_string() + &table.to_string());
}

#[test]
fn rowspan_with_multiline() {
    let mut table = Table::new();
    table
        .set_header(vec!["H1", "H2", "H3"])
        .add_row(vec![
            Cell::new("Multi\nline\nrowspan").set_rowspan(2),
            Cell::new("Cell 2"),
            Cell::new("Cell 3"),
        ])
        .add_row(vec![
            // First position is occupied by rowspan above
            Cell::new("Cell 2 (row 2)"),
            Cell::new("Cell 3 (row 2)"),
        ]);

    let expected = "
+---------+----------------+----------------+
| H1      | H2             | H3             |
+===========================================+
| Multi   | Cell 2         | Cell 3         |
| line    |                |                |
| rowspan |                |                |
|         +----------------+----------------|
|         | Cell 2 (row 2) | Cell 3 (row 2) |
+---------+----------------+----------------+";
    assert_eq!(expected, "\n".to_string() + &table.to_string());
}

#[test]
fn complex_table_with_multiple_spans() {
    let mut table = Table::new();
    table
        .set_header(vec![
            Cell::new("Header 1-2").set_colspan(2),
            Cell::new("Header 3-4").set_colspan(2),
        ])
        .add_row(vec![
            Cell::new("Rowspan 1-2").set_rowspan(2),
            Cell::new("Cell 2"),
            Cell::new("Colspan 2").set_colspan(2),
        ])
        .add_row(vec![
            // First position is occupied by rowspan above
            Cell::new("Cell 2 (row 2)"),
            Cell::new("Cell 3"),
            Cell::new("Cell 4"),
        ]);

    let expected = "
+--------------+-----------------+---------+---------+
| Header 1-2                     | Header 3-4        |
+====================================================+
| Rowspan 1-2  | Cell 2          | Colspan 2         |
|              +-----------------+-------------------|
|              | Cell 2 (row 2)  | Cell 3  | Cell 4  |
+--------------+-----------------+---------+---------+";
    assert_eq!(expected, "\n".to_string() + &table.to_string());
}

// Phase 8.2: Integration Tests

#[test]
fn spans_with_utf8_preset() {
    let mut table = Table::new();
    table
        .load_preset(presets::UTF8_FULL)
        .set_header(vec![
            Cell::new("Header 1-2").set_colspan(2),
            Cell::new("Header3"),
        ])
        .add_row(vec![
            Cell::new("Spans 2 cols").set_colspan(2),
            Cell::new("Normal"),
        ]);

    let output = table.to_string();
    // UTF8 preset should use different border characters
    assert!(output.contains("┌") || output.contains("│") || output.contains("└"));
}

#[test]
fn spans_with_dynamic_arrangement() {
    let mut table = Table::new();
    table
        .set_content_arrangement(ContentArrangement::Dynamic)
        .set_width(40)
        .set_header(vec![
            Cell::new("Header 1-2").set_colspan(2),
            Cell::new("Header3"),
        ])
        .add_row(vec![
            Cell::new("Spans 2 cols with long content").set_colspan(2),
            Cell::new("Normal"),
        ]);

    let expected = "
+--------------+---------+---------+
| Header 1-2             | Header3 |
+==================================+
| Spans 2 cols with long | Normal  |
| content                |         |
+--------------+---------+---------+";
    assert_eq!(expected, "\n".to_string() + &table.to_string());
}

#[test]
fn spans_with_column_constraints() {
    let mut table = Table::new();
    table
        .set_header(vec![
            Cell::new("Header 1-2").set_colspan(2),
            Cell::new("Header3"),
        ])
        .add_row(vec![
            Cell::new("Spans 2 cols").set_colspan(2),
            Cell::new("Normal"),
        ]);

    // Set constraints on columns
    use comfy_table::Width::Fixed;
    table
        .column_mut(0)
        .unwrap()
        .set_constraint(ColumnConstraint::UpperBoundary(Fixed(10)));
    table
        .column_mut(1)
        .unwrap()
        .set_constraint(ColumnConstraint::LowerBoundary(Fixed(5)));
    table
        .column_mut(2)
        .unwrap()
        .set_constraint(ColumnConstraint::Absolute(Fixed(8)));

    let expected = "
+----------+----------+--------+
| Header 1-2          | Header |
|                     | 3      |
+==============================+
| Spans 2 cols        | Normal |
+----------+----------+--------+";
    assert_eq!(expected, "\n".to_string() + &table.to_string());
}

#[test]
fn spans_with_hidden_columns() {
    let mut table = Table::new();
    table
        .set_header(vec![
            Cell::new("Header1"),
            Cell::new("Header2"),
            Cell::new("Header3"),
            Cell::new("Header4"),
        ])
        .add_row(vec![
            Cell::new("Spans 2 cols").set_colspan(2),
            Cell::new("Normal"),
            Cell::new("Normal2"),
        ]);

    // Hide the second column (which is part of the colspan)
    table
        .column_mut(1)
        .unwrap()
        .set_constraint(ColumnConstraint::Hidden);

    let expected = "
+---------+---------+---------+
| Header1 | Header3 | Header4 |
+=============================+
| Spans 2 | Normal  | Normal2 |
| cols    |         |         |
+---------+---------+---------+";
    assert_eq!(expected, "\n".to_string() + &table.to_string());
}

#[test]
fn rowspan_with_dynamic_arrangement() {
    let mut table = Table::new();
    table
        .set_content_arrangement(ContentArrangement::Dynamic)
        .set_width(50)
        .set_header(vec![
            Cell::new("Header1"),
            Cell::new("Header2"),
            Cell::new("Header3"),
        ])
        .add_row(vec![
            Cell::new("Spans 2 rows").set_rowspan(2),
            Cell::new("Cell 2"),
            Cell::new("Cell 3"),
        ])
        .add_row(vec![
            Cell::new("Cell 2 (row 2)"),
            Cell::new("Cell 3 (row 2)"),
        ]);

    let expected = "
+--------------+----------------+----------------+
| Header1      | Header2        | Header3        |
+================================================+
| Spans 2 rows | Cell 2         | Cell 3         |
|              +----------------+----------------|
|              | Cell 2 (row 2) | Cell 3 (row 2) |
+--------------+----------------+----------------+";
    assert_eq!(expected, "\n".to_string() + &table.to_string());
}

#[test]
fn combined_spans_with_constraints() {
    let mut table = Table::new();
    table
        .set_header(vec![
            Cell::new("H1"),
            Cell::new("H2"),
            Cell::new("H3"),
            Cell::new("H4"),
        ])
        .add_row(vec![
            Cell::new("2x2 span").set_colspan(2).set_rowspan(2),
            Cell::new("Cell 3"),
            Cell::new("Cell 4"),
        ])
        .add_row(vec![
            Cell::new("Cell 3 (row 2)"),
            Cell::new("Cell 4 (row 2)"),
        ]);

    // Set constraints
    use comfy_table::Width::Fixed;
    table
        .column_mut(0)
        .unwrap()
        .set_constraint(ColumnConstraint::LowerBoundary(Fixed(8)));
    table
        .column_mut(2)
        .unwrap()
        .set_constraint(ColumnConstraint::UpperBoundary(Fixed(10)));

    let expected = "
+--------+-------+----------+----------------+
| H1     | H2    | H3       | H4             |
+============================================+
| 2x2 span       | Cell 3   | Cell 4         |
|                +----------+----------------|
|                | Cell 3   | Cell 4 (row 2) |
|                | (row 2)  |                |
+--------+-------+----------+----------------+";
    assert_eq!(expected, "\n".to_string() + &table.to_string());
}

#[test]
fn rowspan_with_alignment() {
    let mut table = Table::new();
    table
        .set_header(vec!["H1", "H2", "H3"])
        .add_row(vec![
            Cell::new("Left")
                .set_rowspan(2)
                .set_alignment(CellAlignment::Left),
            Cell::new("Cell 2"),
            Cell::new("Cell 3"),
        ])
        .add_row(vec![
            Cell::new("Cell 2 (row 2)"),
            Cell::new("Cell 3 (row 2)"),
        ])
        .add_row(vec![
            Cell::new("Center")
                .set_rowspan(2)
                .set_alignment(CellAlignment::Center),
            Cell::new("Cell 2"),
            Cell::new("Cell 3"),
        ])
        .add_row(vec![
            Cell::new("Cell 2 (row 2)"),
            Cell::new("Cell 3 (row 2)"),
        ])
        .add_row(vec![
            Cell::new("Right")
                .set_rowspan(2)
                .set_alignment(CellAlignment::Right),
            Cell::new("Cell 2"),
            Cell::new("Cell 3"),
        ])
        .add_row(vec![
            Cell::new("Cell 2 (row 2)"),
            Cell::new("Cell 3 (row 2)"),
        ]);

    let expected = "
+--------+----------------+----------------+
| H1     | H2             | H3             |
+==========================================+
| Left   | Cell 2         | Cell 3         |
|        +----------------+----------------|
|        | Cell 2 (row 2) | Cell 3 (row 2) |
|--------+----------------+----------------|
| Center | Cell 2         | Cell 3         |
|        +----------------+----------------|
|        | Cell 2 (row 2) | Cell 3 (row 2) |
|--------+----------------+----------------|
|  Right | Cell 2         | Cell 3         |
|        +----------------+----------------|
|        | Cell 2 (row 2) | Cell 3 (row 2) |
+--------+----------------+----------------+";
    assert_eq!(expected, "\n".to_string() + &table.to_string());
}

#[test]
fn combined_span_with_alignment() {
    let mut table = Table::new();
    table
        .set_header(vec!["H1", "H2", "H3", "H4"])
        .add_row(vec![
            Cell::new("Left 2x2")
                .set_colspan(2)
                .set_rowspan(2)
                .set_alignment(CellAlignment::Left),
            Cell::new("Cell 3"),
            Cell::new("Cell 4"),
        ])
        .add_row(vec![
            Cell::new("Cell 3 (row 2)"),
            Cell::new("Cell 4 (row 2)"),
        ])
        .add_row(vec![
            Cell::new("Center 2x2")
                .set_colspan(2)
                .set_rowspan(2)
                .set_alignment(CellAlignment::Center),
            Cell::new("Cell 3"),
            Cell::new("Cell 4"),
        ])
        .add_row(vec![
            Cell::new("Cell 3 (row 2)"),
            Cell::new("Cell 4 (row 2)"),
        ]);

    let expected = "
+--------+--------+----------------+----------------+
| H1     | H2     | H3             | H4             |
+===================================================+
| Left 2x2        | Cell 3         | Cell 4         |
|                 +----------------+----------------|
|                 | Cell 3 (row 2) | Cell 4 (row 2) |
|--------+--------+----------------+----------------|
|    Center 2x2   | Cell 3         | Cell 4         |
|                 +----------------+----------------|
|                 | Cell 3 (row 2) | Cell 4 (row 2) |
+--------+--------+----------------+----------------+";
    assert_eq!(expected, "\n".to_string() + &table.to_string());
}

#[cfg(feature = "tty")]
#[test]
fn rowspan_with_styling() {
    let mut table = Table::new();
    table
        .set_header(vec!["H1", "H2", "H3"])
        .add_row(vec![
            Cell::new("Red Bold")
                .set_rowspan(2)
                .fg(Color::Red)
                .add_attribute(Attribute::Bold),
            Cell::new("Cell 2"),
            Cell::new("Cell 3"),
        ])
        .add_row(vec![
            Cell::new("Cell 2 (row 2)"),
            Cell::new("Cell 3 (row 2)"),
        ])
        .add_row(vec![
            Cell::new("Green BG")
                .set_rowspan(2)
                .bg(Color::Green)
                .add_attribute(Attribute::Underlined),
            Cell::new("Cell 2"),
            Cell::new("Cell 3"),
        ])
        .add_row(vec![
            Cell::new("Cell 2 (row 2)"),
            Cell::new("Cell 3 (row 2)"),
        ]);

    table.force_no_tty();
    let expected = "
+----------+----------------+----------------+
| H1       | H2             | H3             |
+============================================+
| Red Bold | Cell 2         | Cell 3         |
|          +----------------+----------------|
|          | Cell 2 (row 2) | Cell 3 (row 2) |
|----------+----------------+----------------|
| Green BG | Cell 2         | Cell 3         |
|          +----------------+----------------|
|          | Cell 2 (row 2) | Cell 3 (row 2) |
+----------+----------------+----------------+";
    assert_eq!(expected, "\n".to_string() + &table.to_string());
}

#[cfg(feature = "tty")]
#[test]
fn combined_span_with_styling() {
    let mut table = Table::new();
    table
        .set_header(vec!["H1", "H2", "H3", "H4"])
        .add_row(vec![
            Cell::new("Styled 2x2")
                .set_colspan(2)
                .set_rowspan(2)
                .fg(Color::Cyan)
                .bg(Color::Black)
                .add_attribute(Attribute::Bold),
            Cell::new("Cell 3"),
            Cell::new("Cell 4"),
        ])
        .add_row(vec![
            Cell::new("Cell 3 (row 2)"),
            Cell::new("Cell 4 (row 2)"),
        ]);

    table.force_no_tty();
    let expected = "
+--------+--------+----------------+----------------+
| H1     | H2     | H3             | H4             |
+===================================================+
| Styled 2x2      | Cell 3         | Cell 4         |
|                 +----------------+----------------|
|                 | Cell 3 (row 2) | Cell 4 (row 2) |
+--------+--------+----------------+----------------+";
    assert_eq!(expected, "\n".to_string() + &table.to_string());
}

#[test]
fn colspan_with_column_alignment() {
    let mut table = Table::new();
    table.set_header(vec!["H1", "H2", "H3", "H4"]).add_row(vec![
        Cell::new("Spans 2 cols").set_colspan(2),
        Cell::new("Cell 3"),
        Cell::new("Cell 4"),
    ]);

    // Set column alignment for first two columns (which are spanned)
    table
        .column_mut(0)
        .unwrap()
        .set_cell_alignment(CellAlignment::Center);
    table
        .column_mut(1)
        .unwrap()
        .set_cell_alignment(CellAlignment::Right);

    let expected = "
+---------+---------+--------+--------+
|    H1   |      H2 | H3     | H4     |
+=====================================+
|    Spans 2 cols   | Cell 3 | Cell 4 |
+---------+---------+--------+--------+";
    assert_eq!(expected, "\n".to_string() + &table.to_string());
}

#[cfg(feature = "tty")]
#[test]
fn multiple_styled_spans() {
    let mut table = Table::new();
    table
        .set_header(vec!["H1", "H2", "H3", "H4", "H5"])
        .add_row(vec![
            Cell::new("Red").set_colspan(2).fg(Color::Red),
            Cell::new("Green").set_colspan(2).fg(Color::Green),
            Cell::new("Blue").fg(Color::Blue),
        ]);

    table.force_no_tty();
    let expected = "
+-----+----+------+-----+------+
| H1  | H2 | H3   | H4  | H5   |
+==============================+
| Red      | Green      | Blue |
+-----+----+------+-----+------+";
    assert_eq!(expected, "\n".to_string() + &table.to_string());
}

#[cfg(feature = "tty")]
#[test]
fn span_with_multiple_attributes() {
    let mut table = Table::new();
    table
        .set_header(vec!["H1", "H2", "H3"])
        .add_row(vec![
            Cell::new("Bold Underlined")
                .set_colspan(2)
                .add_attribute(Attribute::Bold)
                .add_attribute(Attribute::Underlined),
            Cell::new("Normal"),
        ])
        .add_row(vec![
            Cell::new("Blink Italic")
                .set_rowspan(2)
                .add_attribute(Attribute::SlowBlink)
                .add_attribute(Attribute::Italic),
            Cell::new("Cell 2"),
            Cell::new("Cell 3"),
        ])
        .add_row(vec![
            Cell::new("Cell 2 (row 2)"),
            Cell::new("Cell 3 (row 2)"),
        ]);

    table.force_no_tty();
    let expected = "
+--------------+----------------+----------------+
| H1           | H2             | H3             |
+================================================+
| Bold Underlined               | Normal         |
|-------------------------------+----------------|
| Blink Italic | Cell 2         | Cell 3         |
|              +----------------+----------------|
|              | Cell 2 (row 2) | Cell 3 (row 2) |
+--------------+----------------+----------------+";
    assert_eq!(expected, "\n".to_string() + &table.to_string());
}

#[test]
fn span_with_alignment_and_multiline() {
    let mut table = Table::new();
    table
        .set_header(vec!["H1", "H2", "H3"])
        .add_row(vec![
            Cell::new("Left\nMultiline")
                .set_colspan(2)
                .set_alignment(CellAlignment::Left),
            Cell::new("Right"),
        ])
        .add_row(vec![
            Cell::new("Center\nMultiline")
                .set_colspan(2)
                .set_alignment(CellAlignment::Center),
            Cell::new("Right"),
        ]);

    let expected = "
+--------+-------+-------+
| H1     | H2    | H3    |
+========================+
| Left           | Right |
| Multiline      |       |
|----------------+-------|
|     Center     | Right |
|    Multiline   |       |
+--------+-------+-------+";
    assert_eq!(expected, "\n".to_string() + &table.to_string());
}

#[test]
fn colspan_with_custom_borders() {
    let mut table = Table::new();
    table
        .set_header(vec![
            Cell::new("Header1").set_colspan(2),
            Cell::new("Header3"),
        ])
        .add_row(vec![
            Cell::new("Spans 2 cols").set_colspan(2),
            Cell::new("Normal"),
        ]);

    // Set custom border characters
    use comfy_table::TableComponent::*;
    table
        .set_style(LeftBorder, '║')
        .set_style(RightBorder, '║')
        .set_style(TopLeftCorner, '╔')
        .set_style(TopRightCorner, '╗')
        .set_style(BottomLeftCorner, '╚')
        .set_style(BottomRightCorner, '╝')
        .set_style(TopBorder, '═')
        .set_style(BottomBorder, '═')
        .set_style(HeaderLines, '═')
        .set_style(VerticalLines, '║')
        .set_style(TopBorderIntersections, '╦')
        .set_style(BottomBorderIntersections, '╩')
        .set_style(LeftHeaderIntersection, '╠')
        .set_style(RightHeaderIntersection, '╣')
        .set_style(MiddleHeaderIntersections, '╬');

    let expected = "
╔══════════╦══════════╦═════════╗
║ Header1             ║ Header3 ║
╠═════════════════════╬═════════╣
║ Spans 2 cols        ║ Normal  ║
╚══════════╩══════════╩═════════╝";
    assert_eq!(expected, "\n".to_string() + &table.to_string());
}

#[test]
fn rowspan_with_custom_separators() {
    let mut table = Table::new();
    table
        .set_header(vec!["H1", "H2", "H3"])
        .add_row(vec![
            Cell::new("Spans 2 rows").set_rowspan(2),
            Cell::new("Cell 2"),
            Cell::new("Cell 3"),
        ])
        .add_row(vec![
            Cell::new("Cell 2 (row 2)"),
            Cell::new("Cell 3 (row 2)"),
        ]);

    // Set custom separator characters
    use comfy_table::TableComponent::*;
    table
        .set_style(RightBorder, '┤')
        .set_style(VerticalLines, '│')
        .set_style(MiddleIntersections, '┼')
        .set_style(LeftBorderIntersections, '├')
        .set_style(RightBorderIntersections, '┤');

    let expected = "
+--------------+----------------+----------------+
| H1           │ H2             │ H3             ┤
+================================================+
| Spans 2 rows │ Cell 2         │ Cell 3         ┤
├              ┼----------------┼----------------┤
|              │ Cell 2 (row 2) │ Cell 3 (row 2) ┤
+--------------+----------------+----------------+";
    assert_eq!(expected, "\n".to_string() + &table.to_string());
}

#[test]
fn combined_span_with_custom_borders_and_separators() {
    let mut table = Table::new();
    table
        .set_header(vec!["H1", "H2", "H3", "H4"])
        .add_row(vec![
            Cell::new("2x2 span").set_colspan(2).set_rowspan(2),
            Cell::new("Cell 3"),
            Cell::new("Cell 4"),
        ])
        .add_row(vec![
            Cell::new("Cell 3 (row 2)"),
            Cell::new("Cell 4 (row 2)"),
        ]);

    // Set custom border and separator characters
    use comfy_table::TableComponent::*;
    table
        .set_style(LeftBorder, '│')
        .set_style(RightBorder, '│')
        .set_style(TopLeftCorner, '┌')
        .set_style(TopRightCorner, '┐')
        .set_style(BottomLeftCorner, '└')
        .set_style(BottomRightCorner, '┘')
        .set_style(TopBorder, '─')
        .set_style(BottomBorder, '─')
        .set_style(HeaderLines, '═')
        .set_style(VerticalLines, '│')
        .set_style(TopBorderIntersections, '┬')
        .set_style(BottomBorderIntersections, '┴')
        .set_style(MiddleIntersections, '┼')
        .set_style(LeftBorderIntersections, '├')
        .set_style(RightBorderIntersections, '┤')
        .set_style(LeftHeaderIntersection, '╞')
        .set_style(RightHeaderIntersection, '╡')
        .set_style(MiddleHeaderIntersections, '╪');

    let expected = "
┌───────┬───────┬────────────────┬────────────────┐
│ H1    │ H2    │ H3             │ H4             │
╞═══════╪═══════╪════════════════╪════════════════╡
│ 2x2 span      │ Cell 3         │ Cell 4         │
├               ┼----------------┼----------------┤
│               │ Cell 3 (row 2) │ Cell 4 (row 2) │
└───────┴───────┴────────────────┴────────────────┘";
    assert_eq!(expected, "\n".to_string() + &table.to_string());
}
