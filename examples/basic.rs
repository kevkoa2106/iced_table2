use iced::widget::{self, scrollable, text};
use iced::{Element, Task, Theme};
use iced_table2::table::{self, table};

fn main() -> iced::Result {
    iced::application(App::default, App::update, App::view)
        .title("Basic Table")
        .run()
}

struct App {
    columns: Vec<MyColumn>,
    rows: Vec<Person>,
    header_id: widget::Id,
    body_id: widget::Id,
}

#[derive(Debug, Clone)]
enum Message {
    SyncHeader(scrollable::AbsoluteOffset),
}

struct Person {
    name: &'static str,
    age: u32,
    email: &'static str,
}

struct MyColumn {
    title: &'static str,
    width: f32,
}

impl<'a> table::Column<'a, Message, Theme, iced::Renderer> for MyColumn {
    type Row = Person;

    fn header(&'a self, _col_index: usize) -> Element<'a, Message> {
        text(self.title).into()
    }

    fn cell(
        &'a self,
        col_index: usize,
        _row_index: usize,
        row: &'a Person,
    ) -> Element<'a, Message> {
        let content: String = match col_index {
            0 => row.name.to_string(),
            1 => row.age.to_string(),
            2 => row.email.to_string(),
            _ => String::new(),
        };
        text(content).into()
    }

    fn width(&self) -> f32 {
        self.width
    }

    fn resize_offset(&self) -> Option<f32> {
        None
    }
}

impl Default for App {
    fn default() -> Self {
        Self {
            columns: vec![
                MyColumn { title: "Name", width: 200.0 },
                MyColumn { title: "Age", width: 100.0 },
                MyColumn { title: "Email", width: 300.0 },
            ],
            rows: vec![
                Person { name: "Alice", age: 30, email: "alice@example.com" },
                Person { name: "Bob", age: 25, email: "bob@example.com" },
                Person { name: "Charlie", age: 35, email: "charlie@example.com" },
                Person { name: "Diana", age: 28, email: "diana@example.com" },
                Person { name: "Eve", age: 42, email: "eve@example.com" },
            ],
            header_id: widget::Id::unique(),
            body_id: widget::Id::unique(),
        }
    }
}

impl App {
    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::SyncHeader(offset) => {
                widget::operation::scroll_to(self.header_id.clone(), offset)
            }
        }
    }

    fn view(&self) -> Element<'_, Message> {
        table(
            self.header_id.clone(),
            self.body_id.clone(),
            &self.columns,
            &self.rows,
            Message::SyncHeader,
        )
        .cell_padding(8)
        .min_width(600.0)
        .into()
    }
}
