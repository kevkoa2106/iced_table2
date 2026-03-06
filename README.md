# iced_table2

A feature-rich table widget for [iced](https://github.com/iced-rs/iced) 0.14.

## Features

- **Synchronized scrolling** -- Header, body, and optional footer stay horizontally aligned as the user scrolls.
- **Interactive column resizing** -- Drag column dividers to resize. Visual feedback highlights the divider on hover.
- **Row selection** -- Click rows to select them with visual highlighting.
- **Header click / sorting** -- Click column headers to trigger a callback, useful for implementing column sorting.
- **Customizable styling** -- Implement the `Catalog` trait to control header, footer, row, and divider appearance. A default theme with alternating row colors is provided out of the box.

## Usage

Add the dependency to your `Cargo.toml`:

```toml
[dependencies]
iced_table2 = "0.14"
```

### Quick start

1. Define a column type that implements `iced_table2::table::Column`.
2. Create `widget::Id`s for the header, body, and (optionally) footer scrollables.
3. Call `iced_table2::table()` to build the widget.
4. Handle the `on_sync` message in your `update` function by calling `scrollable::scroll_to` on the header and footer ids to keep them in sync with the body.

```rust
use iced::widget;
use iced_table2::table;

// In your view function:
let header_id = widget::Id::unique();
let body_id = widget::Id::unique();

let table = table(header_id, body_id, &columns, &rows, Message::TableSynced)
    .cell_padding(8)
    .min_width(600.0);
```

To enable column resizing, chain `.on_column_resize(Message::ColumnDragged, Message::ColumnReleased)` and store the resize offsets in your column type.

To enable row selection, chain `.on_row_press(Message::RowPressed)` and call `.selected_row(index)` to highlight the selected row.

To react to header clicks (e.g. for sorting), chain `.on_header_press(Message::HeaderPressed)` and handle the column index in your `update` function.

## Examples

Run the examples from the repository root:

```sh
# Minimal table with 3 columns and 5 rows
cargo run --example basic

# Full-featured table with column resizing, footer, row selection, and sorting
cargo run --example advanced
```

## License

GPL-3.0
