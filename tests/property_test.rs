use comfy_table::*;
use comfy_table::presets::UTF8_FULL;
use comfy_table::modifiers::UTF8_ROUND_CORNERS;
use ::proptest::prelude::*;

fn content_arrangement() -> impl Strategy<Value = ContentArrangement> {
  prop_oneof![
    Just(ContentArrangement::Disabled),
    Just(ContentArrangement::Dynamic),
  ]
}

fn cell_alignment() -> impl Strategy<Value = CellAlignment> {
  prop_oneof![
    Just(CellAlignment::Left),
    Just(CellAlignment::Right),
    Just(CellAlignment::Center),
  ]
}


fn column_constraint() -> impl Strategy<Value = ColumnConstraint> {
  prop_oneof![
    Just(ColumnConstraint::ContentWidth),
    any::<u16>().prop_map(ColumnConstraint::Width),
    any::<u16>().prop_map(ColumnConstraint::MinWidth),
    any::<u16>().prop_map(ColumnConstraint::MaxWidth),
    (0u16..100u16).prop_map(ColumnConstraint::Percentage),
    (0u16..100u16).prop_map(ColumnConstraint::MinPercentage),
    (0u16..100u16).prop_map(ColumnConstraint::MaxPercentage),
  ]
}

prop_compose! {
    fn dimensions()(columns in 1u16..10u16, rows in 1u16..10u16)
                    -> (u16, u16) {
       (columns, rows)
   }
}

fn columns_and_rows() -> impl Strategy<Value = (Vec<Vec<String>>, Vec<ColumnConstraint>, Vec<CellAlignment>)> {
    dimensions().prop_flat_map(|(column_count, row_count)| {
        let mut rows = Vec::new();
        let mut alignments = Vec::new();
        for _i in 0..row_count {
            for _j in 0..column_count {
                alignments.push(cell_alignment());
            }
            rows.push(::proptest::collection::vec(".*", 0..column_count as usize));
        }
        let mut constraints = Vec::new();
        for _i in 0..column_count {
            constraints.push(column_constraint());
        }

        (rows, constraints, alignments)
    })
}


prop_compose! {
    fn table()
        (arrangement in content_arrangement(),
        table_width in 0..500u16,
        (rows, constraints, alignments) in columns_and_rows()) -> Table {
        let mut table = Table::new();

        let mut alignments = alignments.iter();
        for row in rows.iter() {
            // Convert a vector of Strings to a vector of Cells and
            // set the content alignment for each cell
            let row: Vec<Cell> = row.iter().map(|content| {
                Cell::new(content.clone())
                    .set_alignment(*alignments.next().unwrap())
            }).collect();
            // Add the row to the table
            table.add_row(row);
        }

        table.set_constraints(constraints)
            .set_table_width(table_width)
            .set_content_arrangement(arrangement)
            .load_preset(UTF8_FULL)
            .apply_modifier(UTF8_ROUND_CORNERS);
        table
    }
}


proptest! {
    #![proptest_config({
        let mut config = ProptestConfig::with_cases(1000);
        config.max_shrink_iters = 16000;
        config
    })]
    #[test]
    fn random_tables(table in table()) {
        let formatted = table.to_string();

        let lines: Vec<&str> = formatted.split_terminator('\n').collect();

        let mut line_iter = lines.iter();
        let line_length = if let Some(line) = line_iter.next() {
            line.chars().count()
        } else {
            0
        };

        for line in line_iter {
            if line.chars().count() != line_length {
                return Err(TestCaseError::Fail("Each line of a printed table has to have the same length!".into()))
            }
        }
    }
}
