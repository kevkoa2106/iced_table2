use iced::widget;
use iced_core::Element;
use iced_table2::table::{self, Column};

#[derive(Debug, Clone)]
#[allow(dead_code)]
enum Message {
    Sync(iced::widget::scrollable::AbsoluteOffset),
    ColumnDrag(usize, f32),
    ColumnRelease,
    RowPress(usize),
}

struct TestRow {
    value: String,
}

struct TestColumn {
    width: f32,
    resize_offset: Option<f32>,
}

impl<'a> Column<'a, Message, iced::Theme, iced::Renderer> for TestColumn {
    type Row = TestRow;

    fn header(&'a self, _col_index: usize) -> Element<'a, Message, iced::Theme, iced::Renderer> {
        iced::widget::text("Header").into()
    }

    fn cell(
        &'a self,
        _col_index: usize,
        _row_index: usize,
        row: &'a TestRow,
    ) -> Element<'a, Message, iced::Theme, iced::Renderer> {
        iced::widget::text(&row.value).into()
    }

    fn footer(
        &'a self,
        _col_index: usize,
        _rows: &'a [TestRow],
    ) -> Option<Element<'a, Message, iced::Theme, iced::Renderer>> {
        Some(iced::widget::text("Footer").into())
    }

    fn width(&self) -> f32 {
        self.width
    }

    fn resize_offset(&self) -> Option<f32> {
        self.resize_offset
    }
}

#[test]
fn table_basic_construction() {
    let columns = vec![
        TestColumn { width: 100.0, resize_offset: None },
        TestColumn { width: 200.0, resize_offset: None },
    ];
    let rows = vec![
        TestRow { value: "A".into() },
        TestRow { value: "B".into() },
    ];

    let header_id = widget::Id::unique();
    let body_id = widget::Id::unique();

    let _element: Element<'_, Message, iced::Theme, iced::Renderer> =
        table::table(header_id, body_id, &columns, &rows, Message::Sync).into();
}

#[test]
fn table_with_all_options() {
    let columns = vec![
        TestColumn { width: 150.0, resize_offset: Some(10.0) },
    ];
    let rows = vec![
        TestRow { value: "X".into() },
    ];

    let header_id = widget::Id::unique();
    let body_id = widget::Id::unique();
    let footer_id = widget::Id::unique();

    let _element: Element<'_, Message, iced::Theme, iced::Renderer> =
        table::table(header_id, body_id, &columns, &rows, Message::Sync)
            .footer(footer_id)
            .on_column_resize(Message::ColumnDrag, Message::ColumnRelease)
            .on_row_press(Message::RowPress)
            .selected_row(0)
            .min_width(800.0)
            .min_column_width(50.0)
            .divider_width(3.0)
            .cell_padding(10)
            .into();
}

#[test]
fn table_with_empty_rows() {
    let columns = vec![
        TestColumn { width: 100.0, resize_offset: None },
    ];
    let rows: Vec<TestRow> = vec![];

    let header_id = widget::Id::unique();
    let body_id = widget::Id::unique();

    let _element: Element<'_, Message, iced::Theme, iced::Renderer> =
        table::table(header_id, body_id, &columns, &rows, Message::Sync).into();
}

#[test]
fn table_with_empty_columns() {
    let columns: Vec<TestColumn> = vec![];
    let rows = vec![
        TestRow { value: "A".into() },
    ];

    let header_id = widget::Id::unique();
    let body_id = widget::Id::unique();

    let _element: Element<'_, Message, iced::Theme, iced::Renderer> =
        table::table(header_id, body_id, &columns, &rows, Message::Sync).into();
}

#[test]
fn column_trait_default_footer_is_none() {
    struct NoFooterColumn;

    impl<'a> Column<'a, Message, iced::Theme, iced::Renderer> for NoFooterColumn {
        type Row = TestRow;

        fn header(&'a self, _: usize) -> Element<'a, Message, iced::Theme, iced::Renderer> {
            iced::widget::text("H").into()
        }

        fn cell(
            &'a self,
            _: usize,
            _: usize,
            _: &'a TestRow,
        ) -> Element<'a, Message, iced::Theme, iced::Renderer> {
            iced::widget::text("C").into()
        }

        fn width(&self) -> f32 { 100.0 }
        fn resize_offset(&self) -> Option<f32> { None }
    }

    let col = NoFooterColumn;
    let rows: Vec<TestRow> = vec![];
    assert!(col.footer(0, &rows).is_none());
}
