# Comfy-table

[![GitHub Actions Workflow](https://github.com/Nukesor/comfy-table/workflows/Tests/badge.svg)](https://github.com/Nukesor/comfy-table/actions)
[![docs](https://docs.rs/comfy-table/badge.svg)](https://docs.rs/comfy-table/)
[![license](http://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/nukesor/comfy-table/blob/main/LICENSE)
[![Crates.io](https://img.shields.io/crates/v/comfy-table.svg)](https://crates.io/crates/comfy-table)
[![codecov](https://codecov.io/gh/nukesor/comfy-table/branch/main/graph/badge.svg)](https://codecov.io/gh/nukesor/comfy-table)

![comfy-table](https://raw.githubusercontent.com/Nukesor/images/main/comfy_table.gif)

<!--- [![dependency status](https://deps.rs/repo/github/nukesor/comfy-table/status.svg)](https://deps.rs/repo/github/nukesor/comfy-table) -->

Comfy-table is designed as a library for building beautiful tables, while being easy to use.

### Features

- Dynamic arrangement of content depending on a given width.
- ANSI content styling for terminals (Colors, Bold, Blinking, etc.).
- Styling Presets and preset modifiers to get you started.
- Pretty much every part of the table is customizable (borders, lines, padding, alignment).
- Constraints on columns that allow some additional control over how to arrange content.
- Cross plattform (Linux, macOS, Windows).
- It's fast enough.
    * Benchmarks show that a pretty big table with complex constraints is build in `470μs` or `~0.5ms`.
    * The table seen at the top of the readme takes `~30μs`.
    * These numbers are from a overclocked `i7-8700K` with a max single-core performance of 4.9GHz.
    * To run the benchmarks yourselves, install criterion via `cargo install cargo-criterion` and run `cargo criterion` afterwards.

Comfy-table is written for the current `stable` Rust version.
Older Rust versions may work but aren't officially supported.

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

    println!("{table}");
}
```

Create a very basic table.\
This table will become as wide as your content. Nothing fancy happening here.

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

    println!("{table}");
}
```

Create a table with UTF8 styling, and apply a modifier that gives the table round corners.\
Additionally, the content will dynamically wrap to maintain a given table width.\
If the table width isn't explicitely set and the program runs in a terminal, the terminal size will be used.

On top of this, we set the default alignment for the right column to `Right` and the alignment of the left top cell to `Center`.

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

    println!("{table}");
}
```

This code generates the table that can be seen at the top of this document.

## Code Examples

A few examples can be found in the `example` folder.
To test an example, run `cargo run --example $name`. E.g.:

```bash
cargo run --example readme_table
```

If you're looking for more information, take a look at the [tests folder](https://github.com/Nukesor/comfy-table/tree/main/tests).  
There are tests for almost every feature including a visual view for each resulting table.

## Contribution Guidelines

Comfy-table is supposed to be minimalistic.
A fixed set of features that just work for "normal" use-cases:

- Normal tables (columns, rows, one cell per column/row).
- Dynamic arrangement of content to a given width.
- Some kind of manual intervention in the arrangement process.

If you come up with an idea or an improvement that fits into the current scope of the project, feel free to create an issue :)!

Some things however will most likely not be added to the project since they drastically increase the complexity of the library or cover very specific edge-cases.

Such features are:

- Nested tables
- Cells that span over multiple columns/rows
- CSV to table conversion and vice versa

## Unsafe

Comfy-table doesn't allow `unsafe` code in its code-base.
As it's a "simple" formatting library it also shouldn't be needed in the future.

However, Comfy-table uses two unsafe functions calls in its dependencies. \
Both calls can be disabled by explicitely calling [Table::force_no_tty](https://docs.rs/comfy-table/4.0.1/comfy_table/struct.Table.html#method.force_no_tty).

Furthermore, all terminal related functionality, including styling, can be disabled by excluding the `tty` feature flag.
Without this flag no `unsafe` code is used as far as I know.

1. `crossterm::tty::IsTty`. This function is necessary to detect whether we're currently on a tty or not.
    This is only called if no explicit width is provided via `Table::set_table_width`.
    ```rust,ignore
    /// On unix the `isatty()` function returns true if a file
    /// descriptor is a terminal.
    #[cfg(unix)]
    impl<S: AsRawFd> IsTty for S {
        fn is_tty(&self) -> bool {
            let fd = self.as_raw_fd();
            unsafe { libc::isatty(fd) == 1 }
        }
    }
    ```
2. `crossterm::terminal::size`. This function is necessary to detect the current terminal width if we're on a tty.
    This is only called if no explicit width is provided via `Table::set_table_width`.

    http://rosettacode.org/wiki/Terminal_control/Dimensions#Library:_BSD_libc
    This is another libc call which is used to communicate with `/dev/tty` via a file descriptor.
    ```rust,ignore
    ...
    if wrap_with_result(unsafe { ioctl(fd, TIOCGWINSZ.into(), &mut size) }).is_ok() {
        Ok((size.ws_col, size.ws_row))
    } else {
        tput_size().ok_or_else(|| std::io::Error::last_os_error().into())
    }
    ...
    ```
