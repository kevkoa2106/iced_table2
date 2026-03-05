use iced::widget::{self, scrollable, text};
use iced::{Element, Task, Theme};
use iced_table2::table::{self, table};

fn main() -> iced::Result {
    iced::application(App::default, App::update, App::view)
        .title("Advanced Table")
        .run()
}

struct App {
    columns: Vec<MyColumn>,
    rows: Vec<Product>,
    header_id: widget::Id,
    body_id: widget::Id,
    footer_id: widget::Id,
    selected_row: Option<usize>,
}

#[derive(Debug, Clone)]
enum Message {
    SyncHeader(scrollable::AbsoluteOffset),
    ColumnDragged(usize, f32),
    ColumnReleased,
    RowPressed(usize),
}

struct Product {
    name: &'static str,
    category: &'static str,
    price: f64,
    quantity: u32,
}

struct MyColumn {
    title: &'static str,
    width: f32,
    resize_offset: Option<f32>,
}

impl<'a> table::Column<'a, Message, Theme, iced::Renderer> for MyColumn {
    type Row = Product;

    fn header(&'a self, _col_index: usize) -> Element<'a, Message> {
        text(self.title).into()
    }

    fn cell(
        &'a self,
        col_index: usize,
        _row_index: usize,
        row: &'a Product,
    ) -> Element<'a, Message> {
        let content: String = match col_index {
            0 => row.name.to_string(),
            1 => row.category.to_string(),
            2 => format!("${:.2}", row.price),
            3 => row.quantity.to_string(),
            _ => String::new(),
        };
        text(content).into()
    }

    fn footer(
        &'a self,
        col_index: usize,
        rows: &'a [Product],
    ) -> Option<Element<'a, Message>> {
        let content = match col_index {
            0 => format!("{} items", rows.len()),
            2 => {
                let total: f64 = rows.iter().map(|r| r.price).sum();
                format!("Avg: ${:.2}", total / rows.len() as f64)
            }
            3 => {
                let total: u32 = rows.iter().map(|r| r.quantity).sum();
                format!("Total: {}", total)
            }
            _ => return None,
        };
        Some(text(content).into())
    }

    fn width(&self) -> f32 {
        self.width
    }

    fn resize_offset(&self) -> Option<f32> {
        self.resize_offset
    }
}

impl Default for App {
    fn default() -> Self {
        Self {
            columns: vec![
                MyColumn { title: "Product", width: 200.0, resize_offset: None },
                MyColumn { title: "Category", width: 150.0, resize_offset: None },
                MyColumn { title: "Price", width: 120.0, resize_offset: None },
                MyColumn { title: "Quantity", width: 100.0, resize_offset: None },
            ],
            rows: vec![
                Product { name: "Laptop", category: "Electronics", price: 999.99, quantity: 15 },
                Product { name: "Mouse", category: "Electronics", price: 29.99, quantity: 150 },
                Product { name: "Keyboard", category: "Electronics", price: 79.99, quantity: 85 },
                Product { name: "Desk", category: "Furniture", price: 349.99, quantity: 20 },
                Product { name: "Chair", category: "Furniture", price: 249.99, quantity: 30 },
                Product { name: "Monitor", category: "Electronics", price: 449.99, quantity: 40 },
                Product { name: "Headphones", category: "Audio", price: 149.99, quantity: 60 },
                Product { name: "Webcam", category: "Electronics", price: 69.99, quantity: 75 },
                Product { name: "Lamp", category: "Furniture", price: 39.99, quantity: 100 },
                Product { name: "Speaker", category: "Audio", price: 89.99, quantity: 55 },
            ],
            header_id: widget::Id::unique(),
            body_id: widget::Id::unique(),
            footer_id: widget::Id::unique(),
            selected_row: None,
        }
    }
}

impl App {
    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::SyncHeader(offset) => {
                return Task::batch([
                    widget::operation::scroll_to(self.header_id.clone(), offset),
                    widget::operation::scroll_to(self.footer_id.clone(), offset),
                ]);
            }
            Message::ColumnDragged(index, offset) => {
                if let Some(col) = self.columns.get_mut(index) {
                    col.resize_offset = Some(offset);
                }
            }
            Message::ColumnReleased => {
                for col in &mut self.columns {
                    if let Some(offset) = col.resize_offset.take() {
                        col.width = (col.width + offset).max(4.0);
                    }
                }
            }
            Message::RowPressed(index) => {
                self.selected_row = if self.selected_row == Some(index) {
                    None
                } else {
                    Some(index)
                };
            }
        }
        Task::none()
    }

    fn view(&self) -> Element<'_, Message> {
        let mut tbl = table(
            self.header_id.clone(),
            self.body_id.clone(),
            &self.columns,
            &self.rows,
            Message::SyncHeader,
        )
        .footer(self.footer_id.clone())
        .on_column_resize(Message::ColumnDragged, Message::ColumnReleased)
        .on_row_press(Message::RowPressed)
        .cell_padding(8)
        .min_width(600.0);

        if let Some(index) = self.selected_row {
            tbl = tbl.selected_row(index);
        }

        tbl.into()
    }
}
