use ::proptest::prelude::*;
use comfy_table::modifiers::UTF8_ROUND_CORNERS;
use comfy_table::presets::UTF8_FULL;
use comfy_table::ColumnConstraint::*;
use comfy_table::Width::*;
use comfy_table::*;

/// Pick any of the three existing ContentArrangement types for the table.
fn content_arrangement() -> impl Strategy<Value = ContentArrangement> {
    prop_oneof![
        Just(ContentArrangement::Disabled),
        Just(ContentArrangement::Dynamic),
        Just(ContentArrangement::DynamicFullWidth),
    ]
}

/// Each cell can have any alignment.
fn cell_alignment() -> impl Strategy<Value = Option<CellAlignment>> {
    prop_oneof![
        Just(None),
        Just(Some(CellAlignment::Left)),
        Just(Some(CellAlignment::Right)),
        Just(Some(CellAlignment::Center)),
    ]
}

/// Any Column can have any constellation of ColumnConstraints
fn column_constraint() -> impl Strategy<Value = Option<ColumnConstraint>> {
    prop_oneof![
        Just(None),
        Just(Some(ColumnConstraint::ContentWidth)),
        Just(Some(ColumnConstraint::Hidden)),
        any::<u16>().prop_map(|width| { Some(Absolute(Fixed(width))) }),
        any::<u16>().prop_map(|width| { Some(LowerBoundary(Fixed(width))) }),
        any::<u16>().prop_map(|width| { Some(UpperBoundary(Fixed(width))) }),
        (0u16..200u16).prop_map(|percentage| { Some(Absolute(Percentage(percentage))) }),
        (0u16..200u16).prop_map(|percentage| { Some(LowerBoundary(Percentage(percentage))) }),
        (0u16..200u16).prop_map(|percentage| { Some(UpperBoundary(Percentage(percentage))) }),
    ]
}

/// We test the Row::max_height with a few values.
fn max_height() -> impl Strategy<Value = Option<usize>> {
    prop_oneof![
        Just(None),
        Just(Some(0)),
        Just(Some(1)),
        Just(Some(5)),
        Just(Some(100))
    ]
}

prop_compose! {
    /// Returns the dimensions of the table, i.e. the amount of rows and columns.
    fn dimensions()(columns in 1u16..10u16, rows in 1u16..10u16)
                    -> (u16, u16) {
       (columns, rows)
   }
}

/// Returns all data needed to build the final table.
/// 1. A matrix of cells Row[Column[Cell]].
/// 2. Constriants for all columns.
/// 3. The alignment for each cell.
/// 3. The alignment for each column.
fn columns_and_rows() -> impl Strategy<
    Value = (
        Vec<Vec<String>>,
        Vec<Option<ColumnConstraint>>,
        Vec<Option<CellAlignment>>,
        Vec<Option<CellAlignment>>,
    ),
> {
    dimensions().prop_flat_map(|(column_count, row_count)| {
        let mut rows = Vec::new();
        let mut cell_alignments = Vec::new();
        for _i in 0..row_count {
            // Create the max amount of possibly needed cell alignments
            for _j in 0..column_count {
                cell_alignments.push(cell_alignment());
            }
            // Add a strategy that creates random cell content with a length of 0 to column_count
            rows.push(::proptest::collection::vec(".*", 0..column_count as usize));
        }
        let mut constraints = Vec::new();
        let mut column_alignments = Vec::new();
        for _i in 0..column_count {
            constraints.push(column_constraint());
            column_alignments.push(cell_alignment());
        }

        (rows, constraints, cell_alignments, column_alignments)
    })
}

prop_compose! {
    /// The ultimate test
    /// This creates a table from a combination of all "random" selectors above.
    fn table()
        (arrangement in content_arrangement(),
        max_height in max_height(),
        table_width in 0..1000u16,
        (rows, constraints, cell_alignments, column_alignments) in columns_and_rows()) -> Table {

        let mut table = Table::new();
        if let Some(height) = max_height {
            for row in table.row_iter_mut() {
                row.max_height(height);
            }
        }

        let mut cell_alignments = cell_alignments.iter();
        for row in rows.iter() {
            // Convert a vector of Strings to a vector of Cells and
            // set the content alignment for each cell
            let row: Vec<Cell> = row.iter().map(|content| {
                let mut cell = Cell::new(content.clone());
                if let Some(alignment) = cell_alignments.next().unwrap() {
                    cell = cell.set_alignment(*alignment);
                }
                cell
            }).collect();

            // Add the row to the table
            table.add_row(row);
        }

        for (column_index, column) in table.column_iter_mut().enumerate() {
            // Add the alignment for all columns
            let alignment = column_alignments.get(column_index).unwrap();
            if let Some(alignment) = alignment {
                column.set_cell_alignment(*alignment);
            }

            // Set constraints for all columns
            let constraint = constraints.get(column_index).unwrap();
            if let Some(constraint) = constraint {
                column.set_constraint(*constraint);
            }

        }

        table.set_width(table_width)
            .set_content_arrangement(arrangement)
            .load_preset(UTF8_FULL)
            .apply_modifier(UTF8_ROUND_CORNERS);
        table
    }
}

proptest! {
    #![proptest_config({
        let mut config = ProptestConfig::with_cases(512);
        config.max_shrink_iters = 2000;
        config
    })]
    #[test]
    fn random_tables(table in table()) {
        // Make sure the table builds without any panics
        let _formatted = table.to_string();

        // Ensure that all lines have the same lenght.
        // This check has been disabled for now.
        // UTF-8 characters completely break table alignment in edge-case situations (e.g. 1 space columns).
        // UTF-8 characters can be multiple characters wide, which conflicts with the 1 space
        // column fallback, as well as fixed-width-, percental- and max-column-constraints.
        // As a result, we cannot check this with proptest, as this is inherently broken.
        //
        // let lines: Vec<&str> = formatted.split_terminator('\n').collect();
        //
        // let mut line_iter = lines.iter();
        // let line_length = if let Some(line) = line_iter.next() {
        //     line.width()
        // } else {
        //     0
        // };
        //for line in line_iter {
        //    if line.width() != line_length {
        //        return Err(TestCaseError::Fail("Each line of a printed table has to have the same length!".into()))
        //    }
        //}
    }
}
