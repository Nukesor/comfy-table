# Comfy-table

[![GitHub Actions Workflow](https://github.com/Nukesor/comfy-table/workflows/Tests/badge.svg)](https://github.com/Nukesor/comfy-table/actions)
[![docs](https://docs.rs/comfy-table/badge.svg)](https://docs.rs/comfy-table/)
[![license](http://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/nukesor/comfy-table/blob/master/LICENSE)
[![Crates.io](https://img.shields.io/crates/v/comfy-table.svg)](https://crates.io/crates/comfy-table)
[![codecov](https://codecov.io/gh/nukesor/comfy-table/branch/master/graph/badge.svg)](https://codecov.io/gh/nukesor/comfy-table)
[![Patreon](https://github.com/Nukesor/images/blob/master/patreon-donate-blue.svg)](https://www.patreon.com/nukesor)
[![Paypal](https://github.com/Nukesor/images/blob/master/paypal-donate-blue.svg)](https://www.paypal.me/arnebeer/)

![comfy-table](https://raw.githubusercontent.com/Nukesor/images/master/comfy_table.gif)

<!--- [![dependency status](https://deps.rs/repo/github/nukesor/comfy-table/status.svg)](https://deps.rs/repo/github/nukesor/comfy-table) -->

Comfy-table tries to provide utility for building beautiful tables, while being easy to use.

Features:

- Dynamic arrangement of content to a specific width.
- Content styling for terminals (Colors, Bold, Blinking, etc.).
- Presets and preset modifiers to get you started.
- Pretty much every part of the table is customizable (borders, lines, padding, alignment).
- Constraints on columns that allow some additional control over how to arrange content.
- Cross plattform (Linux, macOS, Windows).

## Guide-lines

Comfy-table is supposed to be minimalistic.
A fixed set of features that just work, for a simple use-case:

- Normal tables (columns, rows, one cell per column/row).
- Dynamic arrangement of content to a given width.
- Some kind of manual intervention in the arrangement process.

If you come up with an idea or an improvement, that fits into the current scope of the project, feel free to create an issue :)!

Some things however will most likely not be added to the project, since they drastically increase the complexity of the library or are only used by a very small number of people.

Such features are:

- Nested tables
- Cells that span over multiple columns/rows
- CSV to table conversion and vice versa

## Examples

```rust
use comfy_table::Table;

fn main() {
    let mut table = Table::new();
    table
        .set_header(vec!["Header1", "Header2", "Header3"])
        .add_row(vec![
            "This is a text",
            "This is another text",
            "This is the third text",
        ])
        .add_row(vec![
            "This is another text",
            "Now\nadd some\nmulti line stuff",
            "This is awesome",
        ]);

    println!("{}", table);
}
```

Create a very basic table.\
This table will become as wide as your content, nothing fancy happening here.

```text,ignore
+----------------------+----------------------+------------------------+
| Header1              | Header2              | Header3                |
+======================================================================+
| This is a text       | This is another text | This is the third text |
|----------------------+----------------------+------------------------|
| This is another text | Now                  | This is awesome        |
|                      | add some             |                        |
|                      | multi line stuff     |                        |
+----------------------+----------------------+------------------------+
```

### More Features

```rust
use comfy_table::*;
use comfy_table::presets::UTF8_FULL;
use comfy_table::modifiers::UTF8_ROUND_CORNERS;

fn main() {
    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL)
        .apply_modifier(UTF8_ROUND_CORNERS)
        .set_content_arrangement(ContentArrangement::Dynamic)
        .set_table_width(40)
        .set_header(vec!["Header1", "Header2", "Header3"])
        .add_row(vec![
            Cell::new("Center aligned").set_alignment(CellAlignment::Center),
            Cell::new("This is another text"),
            Cell::new("This is the third text"),
        ])
        .add_row(vec![
            "This is another text",
            "Now\nadd some\nmulti line stuff",
            "This is awesome",
        ]);

    // Set the default alignment for the third column to right
    let column = table.get_column_mut(2).expect("Our table has three columns");
    column.set_cell_alignment(CellAlignment::Right);

    println!("{}", table);
}
```

Create a table with UTF8 styling, and apply a modifier, which gives the table round corners.\
Additionall the content will dynamically wrap to maintain a given table width.\
If the table width isn't explicitely set, the terminal size will be used, if this is executed in a terminal.

On top of this, we set the default alignment for the right column to `Right` and the Alignment of the left top cell to `Center`.


```text,ignore
╭────────────┬────────────┬────────────╮
│ Header1    ┆ Header2    ┆    Header3 │
╞════════════╪════════════╪════════════╡
│  This is a ┆ This is    ┆    This is │
│    text    ┆ another    ┆  the third │
│            ┆ text       ┆       text │
├╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌┤
│ This is    ┆ Now        ┆    This is │
│ another    ┆ add some   ┆    awesome │
│ text       ┆ multi line ┆            │
│            ┆ stuff      ┆            │
╰────────────┴────────────┴────────────╯
```

### Styling

```rust
use comfy_table::*;
use comfy_table::presets::UTF8_FULL;

fn main() {
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

    println!("{}", table);
}
```

This code generates the table that can be seen at the top of this Readme.

## Code Examples

There is an example folder containing a few examples.
To run an example, run it with `run --example`. E.g.:

``` cargo run --example readme_table ```

If you're looking for more information, take a look at the [tests folder](https://github.com/Nukesor/comfy-table/tree/master/tests).\
There is a test for almost every feature including a visual view for each resulting table.

## Feedback

This is my first Rust library! If you have some suggestions on how to improve this library please create an issue. I'm always open to constructive criticism and eager to learn how to do this properly!
