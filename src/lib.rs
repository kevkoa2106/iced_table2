//! A table widget for [iced](https://github.com/iced-rs/iced) 0.14.
//!
//! This crate provides a [`Table`] widget that displays rows of data in columns
//! with synchronized header, body, and optional footer scrolling. Columns can be
//! interactively resized by dragging dividers, rows can be clicked and visually
//! highlighted via single-row selection, and the table's appearance is fully
//! customizable via the [`Catalog`] trait.
//!
//! # Getting started
//!
//! 1. Define a column type that implements the [`Column`](table::Column) trait.
//! 2. Create [`widget::Id`](iced_core::widget::Id)s for the header, body, and
//!    (optionally) footer scrollables.
//! 3. Call [`table()`] to build the widget.
//! 4. In your `update` function, handle the `on_sync` message by scrolling
//!    the header and footer scrollables to the received offset so they stay
//!    aligned with the body.
//!
//! ```ignore
//! use iced::widget;
//! use iced_table2::table;
//!
//! let header_id = widget::Id::unique();
//! let body_id = widget::Id::unique();
//!
//! let table = table(header_id, body_id, &columns, &rows, Message::TableSynced)
//!     .cell_padding(8)
//!     .min_width(600.0);
//! ```
#![deny(missing_debug_implementations, missing_docs)]
pub use style::Catalog;
pub use table::{table, Table};

mod divider;
mod style;

pub mod table {
    //! Types for building and configuring a [`Table`] widget.
    use iced_core::widget;
    use iced_core::{Element, Length, Padding};
    use iced_widget::{column, container, mouse_area, row, scrollable, Space};

    use super::divider::Divider;
    use super::style;

    /// Creates a new [`Table`] with the given columns and row data.
    ///
    /// `header` and `body` are [`widget::Id`]s for the header and body
    /// scrollables. The body is the only scrollable that the user interacts
    /// with directly; the header (and optional footer) are kept in sync
    /// through `on_sync`.
    ///
    /// When the body is scrolled, the table emits the `on_sync` message with
    /// the current horizontal offset. You **must** handle this message in your
    /// `update` function by scrolling the header and footer scrollables to the
    /// received offset so they stay aligned with the body.
    pub fn table<'a, Column, Row, Message, Theme>(
        header: widget::Id,
        body: widget::Id,
        columns: &'a [Column],
        rows: &'a [Row],
        on_sync: fn(scrollable::AbsoluteOffset) -> Message,
    ) -> Table<'a, Column, Row, Message, Theme>
    where
        Theme: style::Catalog + container::Catalog,
    {
        Table {
            header,
            body,
            footer: None,
            columns,
            rows,
            on_sync,
            on_column_drag: None,
            on_column_release: None,
            on_row_press: None,
            selected_row: None,
            min_width: 0.0,
            min_column_width: 4.0,
            divider_width: 2.0,
            cell_padding: 4.into(),
            style: Default::default(),
            scrollbar: scrollable::Scrollbar::default(),
        }
    }

    /// Describes how a single column renders its header, cells, and optional footer.
    ///
    /// Implement this trait on your column type to tell the [`Table`] what to
    /// display. Each method receives the column index so a single type can
    /// represent multiple columns (e.g. an enum or a vec of column definitions).
    pub trait Column<'a, Message, Theme, Renderer> {
        /// The row data type that this column knows how to render.
        type Row;

        /// Returns the header [`Element`] for this column at `col_index`.
        fn header(&'a self, col_index: usize) -> Element<'a, Message, Theme, Renderer>;

        /// Returns the cell [`Element`] for the given `col_index`, `row_index`,
        /// and `row` data.
        fn cell(
            &'a self,
            col_index: usize,
            row_index: usize,
            row: &'a Self::Row,
        ) -> Element<'a, Message, Theme, Renderer>;

        /// Returns the footer [`Element`] for this column, if any.
        ///
        /// The full slice of rows is provided so the footer can compute
        /// aggregates (sums, counts, etc.). Returns `None` by default.
        fn footer(
            &'a self,
            _col_index: usize,
            _rows: &'a [Self::Row],
        ) -> Option<Element<'a, Message, Theme, Renderer>> {
            None
        }

        /// Returns the current width of this column in logical pixels.
        fn width(&self) -> f32;

        /// Returns the offset of an in-progress resize, if any.
        ///
        /// While the user is dragging a column divider, the table calls this
        /// to determine the visual width adjustment. Return `None` when the
        /// column is not being resized.
        fn resize_offset(&self) -> Option<f32>;
    }

    /// A table widget that displays rows of data organized into columns.
    #[allow(missing_debug_implementations)]
    pub struct Table<'a, Column, Row, Message, Theme>
    where
        Theme: style::Catalog + container::Catalog,
    {
        header: widget::Id,
        body: widget::Id,
        footer: Option<widget::Id>,
        columns: &'a [Column],
        rows: &'a [Row],
        on_sync: fn(scrollable::AbsoluteOffset) -> Message,
        on_column_drag: Option<fn(usize, f32) -> Message>,
        on_column_release: Option<Message>,
        /// Callback emitted when a body row is clicked, receiving the row index.
        on_row_press: Option<fn(usize) -> Message>,
        /// Index of the currently selected row, if any.
        selected_row: Option<usize>,
        min_width: f32,
        min_column_width: f32,
        divider_width: f32,
        cell_padding: Padding,
        style: <Theme as style::Catalog>::Style,
        scrollbar: scrollable::Scrollbar,
    }

    impl<'a, Column, Row, Message, Theme> Table<'a, Column, Row, Message, Theme>
    where
        Theme: style::Catalog + container::Catalog,
    {
        /// Enables interactive column resizing.
        ///
        /// `on_drag` is called continuously while the user drags a column
        /// divider, receiving the column index and the pixel offset from the
        /// drag origin. Store this offset and return it from
        /// [`Column::resize_offset`] so the table can preview the new width.
        ///
        /// `on_release` is emitted once when the drag ends. Use it to apply
        /// the final offset to the column's stored width and clear the
        /// resize offset.
        pub fn on_column_resize(
            self,
            on_drag: fn(usize, f32) -> Message,
            on_release: Message,
        ) -> Self {
            Self {
                on_column_drag: Some(on_drag),
                on_column_release: Some(on_release),
                ..self
            }
        }

        /// Sets a callback that is emitted when a body row is clicked.
        ///
        /// The callback receives the index of the clicked row.
        pub fn on_row_press(self, on_press: fn(usize) -> Message) -> Self {
            Self {
                on_row_press: Some(on_press),
                ..self
            }
        }

        /// Marks the row at `index` as selected.
        ///
        /// The selected row is rendered with the
        /// [`selected_row`](crate::Catalog::selected_row) style instead of the
        /// normal [`row`](crate::Catalog::row) style.
        pub fn selected_row(self, index: usize) -> Self {
            Self {
                selected_row: Some(index),
                ..self
            }
        }

        /// Enables the footer row using the given scrollable id.
        ///
        /// When set, each column's [`Column::footer`] method is called to
        /// produce footer cells. The footer scrolls in sync with the header
        /// and body.
        pub fn footer(self, footer: widget::Id) -> Self {
            Self {
                footer: Some(footer),
                ..self
            }
        }

        /// Sets the minimum total width of the table in logical pixels.
        ///
        /// Useful with [`responsive`](iced_widget::responsive) to ensure the
        /// table always fills the width of its parent container.
        pub fn min_width(self, min_width: f32) -> Self {
            Self { min_width, ..self }
        }

        /// Sets the minimum width a column can be resized to, in logical pixels.
        ///
        /// Defaults to `4.0`.
        pub fn min_column_width(self, min_column_width: f32) -> Self {
            Self {
                min_column_width,
                ..self
            }
        }

        /// Sets the width of the column dividers in logical pixels.
        ///
        /// Defaults to `2.0`.
        pub fn divider_width(self, divider_width: f32) -> Self {
            Self {
                divider_width,
                ..self
            }
        }

        /// Sets the [`Padding`] applied inside each cell of the table.
        ///
        /// Defaults to `4.0` on all sides.
        pub fn cell_padding(self, cell_padding: impl Into<Padding>) -> Self {
            Self {
                cell_padding: cell_padding.into(),
                ..self
            }
        }

        /// Sets the style of this [`Table`].
        ///
        /// See [`Catalog`](crate::Catalog) for how to define custom styles.
        pub fn style(self, style: impl Into<<Theme as style::Catalog>::Style>) -> Self {
            Self {
                style: style.into(),
                ..self
            }
        }

        /// Sets the [`Scrollbar`](iced_widget::scrollable::Scrollbar) appearance
        /// for the table body's horizontal and vertical scrollbars.
        pub fn scrollbar(self, scrollbar: scrollable::Scrollbar) -> Self {
            Self { scrollbar, ..self }
        }
    }

    impl<'a, Column, Row, Message, Theme, Renderer> From<Table<'a, Column, Row, Message, Theme>>
        for Element<'a, Message, Theme, Renderer>
    where
        Renderer: iced_core::Renderer + iced_core::text::Renderer + 'a,
        Theme: style::Catalog + container::Catalog + scrollable::Catalog + 'a,
        Column: self::Column<'a, Message, Theme, Renderer, Row = Row>,
        Message: 'a + Clone,
    {
        fn from(table: Table<'a, Column, Row, Message, Theme>) -> Self {
            let Table {
                header,
                body,
                footer,
                columns,
                rows,
                on_sync,
                on_column_drag,
                on_column_release,
                on_row_press,
                selected_row,
                min_width,
                min_column_width,
                divider_width,
                cell_padding,
                style,
                scrollbar,
            } = table;

            let header = scrollable(style::wrapper::header(
                row(columns
                    .iter()
                    .enumerate()
                    .map(|(index, column)| {
                        header_container(
                            index,
                            column,
                            on_column_drag,
                            on_column_release.clone(),
                            min_column_width,
                            divider_width,
                            cell_padding,
                            style.clone(),
                        )
                    })
                    .chain(dummy_container(columns, min_width, min_column_width))),
                style.clone(),
            ))
            .id(header)
            .direction(scrollable::Direction::Both {
                vertical: scrollable::Scrollbar::new()
                    .width(0)
                    .margin(0)
                    .scroller_width(0),
                horizontal: scrollable::Scrollbar::new()
                    .width(0)
                    .margin(0)
                    .scroller_width(0),
            });

            let body = scrollable(column(rows.iter().enumerate().map(|(row_index, _row)| {
                let is_selected = selected_row == Some(row_index);

                let row_content = row(columns
                    .iter()
                    .enumerate()
                    .map(|(col_index, column)| {
                        body_container(
                            col_index,
                            row_index,
                            column,
                            _row,
                            min_column_width,
                            divider_width,
                            cell_padding,
                        )
                    })
                    .chain(dummy_container(columns, min_width, min_column_width)));

                let styled: Element<'_, Message, Theme, Renderer> = if is_selected {
                    style::wrapper::selected_row(row_content, style.clone(), row_index)
                } else {
                    style::wrapper::row(row_content, style.clone(), row_index)
                };

                if let Some(on_press) = on_row_press {
                    mouse_area(styled).on_press((on_press)(row_index)).into()
                } else {
                    styled.into()
                }
            })))
            .id(body)
            .on_scroll(move |viewport| {
                let offset = viewport.absolute_offset();

                (on_sync)(scrollable::AbsoluteOffset { y: 0.0, ..offset })
            })
            .direction(scrollable::Direction::Both {
                horizontal: scrollbar,
                vertical: scrollbar,
            })
            .height(Length::Fill);

            let footer = footer.map(|footer| {
                scrollable(style::wrapper::footer(
                    row(columns
                        .iter()
                        .enumerate()
                        .map(|(index, column)| {
                            footer_container(
                                index,
                                column,
                                rows,
                                on_column_drag,
                                on_column_release.clone(),
                                min_column_width,
                                divider_width,
                                cell_padding,
                                style.clone(),
                            )
                        })
                        .chain(dummy_container(columns, min_width, min_column_width))),
                    style,
                ))
                .id(footer)
                .direction(scrollable::Direction::Both {
                    vertical: scrollable::Scrollbar::new()
                        .width(0)
                        .margin(0)
                        .scroller_width(0),
                    horizontal: scrollable::Scrollbar::new()
                        .width(0)
                        .margin(0)
                        .scroller_width(0),
                })
            });

            let mut column = column![header, body];

            if let Some(footer) = footer {
                column = column.push(footer);
            }

            column.height(Length::Fill).into()
        }
    }

    fn header_container<'a, Column, Row, Message, Theme, Renderer>(
        index: usize,
        column: &'a Column,
        on_drag: Option<fn(usize, f32) -> Message>,
        on_release: Option<Message>,
        min_column_width: f32,
        divider_width: f32,
        cell_padding: Padding,
        style: <Theme as style::Catalog>::Style,
    ) -> Element<'a, Message, Theme, Renderer>
    where
        Renderer: iced_core::Renderer + 'a,
        Theme: style::Catalog + container::Catalog + 'a,
        Column: self::Column<'a, Message, Theme, Renderer, Row = Row>,
        Message: 'a + Clone,
    {
        let content = container(column.header(index))
            .width(Length::Fill)
            .padding(cell_padding)
            .into();

        with_divider(
            index,
            column,
            content,
            on_drag,
            on_release,
            min_column_width,
            divider_width,
            style,
        )
    }

    fn body_container<'a, Column, Row, Message, Theme, Renderer>(
        col_index: usize,
        row_index: usize,
        column: &'a Column,
        row: &'a Row,
        min_column_width: f32,
        divider_width: f32,
        cell_padding: Padding,
    ) -> Element<'a, Message, Theme, Renderer>
    where
        Renderer: iced_core::Renderer + 'a,
        Theme: style::Catalog + container::Catalog + 'a,
        Column: self::Column<'a, Message, Theme, Renderer, Row = Row>,
        Message: 'a + Clone,
    {
        let width = column.width() + column.resize_offset().unwrap_or_default();

        let content = container(column.cell(col_index, row_index, row))
            .width(Length::Fill)
            .padding(cell_padding);

        let spacing = Space::new().width(divider_width).height(Length::Shrink);

        row![content, spacing]
            .width(width.max(min_column_width))
            .into()
    }

    fn footer_container<'a, Column, Row, Message, Theme, Renderer>(
        index: usize,
        column: &'a Column,
        rows: &'a [Row],
        on_drag: Option<fn(usize, f32) -> Message>,
        on_release: Option<Message>,
        min_column_width: f32,
        divider_width: f32,
        cell_padding: Padding,
        style: <Theme as style::Catalog>::Style,
    ) -> Element<'a, Message, Theme, Renderer>
    where
        Renderer: iced_core::Renderer + 'a,
        Theme: style::Catalog + container::Catalog + 'a,
        Column: self::Column<'a, Message, Theme, Renderer, Row = Row>,
        Message: 'a + Clone,
    {
        let content = if let Some(footer) = column.footer(index, rows) {
            container(footer)
                .width(Length::Fill)
                .padding(cell_padding)
                .into()
        } else {
            Element::from(Space::new().width(Length::Fill))
        };

        with_divider(
            index,
            column,
            content,
            on_drag,
            on_release,
            min_column_width,
            divider_width,
            style,
        )
    }

    fn with_divider<'a, Column, Row, Message, Theme, Renderer>(
        index: usize,
        column: &'a Column,
        content: Element<'a, Message, Theme, Renderer>,
        on_drag: Option<fn(usize, f32) -> Message>,
        on_release: Option<Message>,
        min_column_width: f32,
        divider_width: f32,
        style: <Theme as style::Catalog>::Style,
    ) -> Element<'a, Message, Theme, Renderer>
    where
        Renderer: iced_core::Renderer + 'a,
        Theme: style::Catalog + container::Catalog + 'a,
        Column: self::Column<'a, Message, Theme, Renderer, Row = Row>,
        Message: 'a + Clone,
    {
        let width =
            (column.width() + column.resize_offset().unwrap_or_default()).max(min_column_width);

        if let Some((on_drag, on_release)) = on_drag.zip(on_release) {
            let old_width = column.width();

            container(Divider::new(
                content,
                divider_width,
                move |offset| {
                    let new_width = (old_width + offset).max(min_column_width);

                    (on_drag)(index, new_width - old_width)
                },
                on_release,
                style,
            ))
            .width(width)
            .into()
        } else {
            row![content, Space::new().width(divider_width).height(Length::Shrink)]
                .width(width)
                .into()
        }
    }

    // Used to enforce "min_width"
    fn dummy_container<'a, Column, Row, Message, Theme, Renderer>(
        columns: &'a [Column],
        min_width: f32,
        min_column_width: f32,
    ) -> Option<Element<'a, Message, Theme, Renderer>>
    where
        Renderer: iced_core::Renderer + 'a,
        Theme: style::Catalog + container::Catalog + 'a,
        Column: self::Column<'a, Message, Theme, Renderer, Row = Row>,
        Message: 'a + Clone,
    {
        let total_width: f32 = columns
            .iter()
            .map(|column| {
                (column.width() + column.resize_offset().unwrap_or_default()).max(min_column_width)
            })
            .sum();

        let remaining = min_width - total_width;

        (remaining > 0.0).then(|| container(Space::new().width(remaining)).into())
    }
}
