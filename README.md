# Comfy-table

[![GitHub Actions Workflow](https://github.com/Nukesor/comfy-table/workflows/Tests/badge.svg)](https://github.com/Nukesor/comfy-table/actions)
[![docs](https://docs.rs/comfy-table/badge.svg)](https://docs.rs/comfy-table/)
[![Crates.io](https://img.shields.io/crates/v/comfy-table.svg)](https://crates.io/crates/comfy-table)
[![codecov](https://codecov.io/gh/nukesor/comfy-table/branch/master/graph/badge.svg)](https://codecov.io/gh/nukesor/comfy-table)
[![dependency status](https://deps.rs/repo/github/nukesor/comfy-table/status.svg)](https://deps.rs/repo/github/nukesor/comfy-table)
[![license](http://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/nukesor/comfy-table/blob/master/LICENSE)
[![Patreon](https://github.com/Nukesor/images/blob/master/patreon-donate-blue.svg)](https://www.patreon.com/nukesor)
[![Paypal](https://github.com/Nukesor/images/blob/master/paypal-donate-blue.svg)](https://www.paypal.me/arnebeer/)

![comfy-table](https://raw.githubusercontent.com/Nukesor/images/master/comfy_table.gif)

Comfy-table tries to provide utility for building beautiful tables, while being easy to use.

**Warning:** This library is quite young and very actively developed. There still may be some bugs, event though I'm continuously writing tests to cover all features.
If you find anything, please create an issue. I'll then try to fix them as soon as possible.


Features:

- Automatic arrangement content to a specific width.
- Styling options for terminals.
- Cross plattform (Linux, macOS, Windows)
- Every part of the table can be customized


**Basic usage:**
```rust
use comfy_table::prelude::*;

fn main() {
    let mut table = Table::new();
    table
        .set_header(&vec!["Header1", "Header2", "Header3"])
        .add_row(&vec![
            "This is a text",
            "This is another text",
            "This is the third text",
        ])
        .add_row(&vec![
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


**Advanced usage:**
```rust
use comfy_table::prelude::*;
use comfy_table::style::presets::UTF8_FULL;

fn main() {
    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL)
        .set_content_arrangement(ContentArrangement::Dynamic)
        .set_table_width(40)
        .set_header(&vec!["Header1", "Header2", "Header3"])
        .add_row(&vec![
            "This is a text",
            "This is another text",
            "This is the third text",
        ])
        .add_row(&vec![
            "This is another text",
            "Now\nadd some\nmulti line stuff",
            "This is awesome",
        ]);

    println!("{}", table);
}
```
Create a table with UTF8 styling, that automatically wraps content to maintain a given table width.
If the table width isn't explicitely set, the terminal size will be used, this is executed in a terminal.

```text,ignore
┌────────────┬────────────┬────────────┐
│ Header1    ┆ Header2    ┆ Header3    │
╞════════════╪════════════╪════════════╡
│ This is a  ┆ This is    ┆ This is    │
│ text       ┆ another    ┆ the third  │
│            ┆ text       ┆ text       │
├╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌┤
│ This is    ┆ Now        ┆ This is    │
│ another    ┆ add some   ┆ awesome    │
│ text       ┆ multi line ┆            │
│            ┆ stuff      ┆            │
└────────────┴────────────┴────────────┘
```


## More Examples

If you're looking for examples, take a look at the [tests folder](https://github.com/Nukesor/comfy-table/tree/master/tests).\
There is a test for almost every feature including a visual view for each resulting table.


## Feedback

This is my first Rust library! If you have some suggestions on how to improve this library please create an issue. I'm always open to constructive criticism and eager to learn how to do this properly!
