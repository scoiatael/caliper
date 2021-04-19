//! This example showcases an interactive `Canvas` for drawing Bézier curves.
use iced::{
    button, executor, image, Align, Application, Button, Clipboard, Column, Command, Container,
    Element, Length, Settings, Text,
};
use nfd2::Response;

mod bezier;

#[derive(Debug, Clone)]
struct OpenDialogError;

impl std::fmt::Display for OpenDialogError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "got no files to open")
    }
}

pub fn main() -> iced::Result {
    let path = match nfd2::open_file_dialog(Option::Some("png"), None).expect("oh no") {
        Response::Okay(file_path) => Result::Ok(file_path),
        Response::OkayMultiple(mut files) => {
            println!("Got {:?}, using first one.", files);
            match files.pop() {
                Option::None => Result::Err(OpenDialogError),
                Option::Some(file) => Result::Ok(file),
            }
        }
        Response::Cancel => Result::Err(OpenDialogError),
    }
    .expect("failed open dialog");

    Example::run(Settings {
        antialiasing: true,
        flags: (Flags {
            file: Option::Some(path),
        }),
        ..Settings::default()
    })
}

struct Example {
    bezier: bezier::State,
    curves: Vec<bezier::Curve>,
    button_state: button::State,
    img: image::Handle,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    AddCurve(bezier::Curve),
    Clear,
}

#[derive(Debug, Clone, Default)]
struct Flags {
    file: Option<std::path::PathBuf>,
}

impl Application for Example {
    type Message = Message;
    type Executor = executor::Default;
    type Flags = Flags;

    fn new(flags: Self::Flags) -> (Example, Command<Self::Message>) {
        let example = Example {
            img: image::Handle::from_path(flags.file.expect("file missing?")),
            bezier: bezier::State::default(),
            curves: Vec::default(),
            button_state: button::State::default(),
        };
        (example, Command::none())
    }

    fn title(&self) -> String {
        String::from("Bezier tool - Iced")
    }

    fn update(&mut self, message: Message, _clipboard: &mut Clipboard) -> iced::Command<Message> {
        match message {
            Message::AddCurve(curve) => {
                self.curves.push(curve);
                self.bezier.request_redraw();
            }
            Message::Clear => {
                self.bezier = bezier::State::default();
                self.curves.clear();
            }
        };
        Command::none()
    }

    fn view(&mut self) -> Element<Message> {
        Column::new()
            .padding(20)
            .spacing(20)
            .align_items(Align::Center)
            .push(
                Text::new("Bezier tool example")
                    .width(Length::Shrink)
                    .size(50),
            )
            .push(
                Container::new(image::Image::new(self.img.clone()))
                    .width(Length::Fill)
                    .height(Length::Fill),
            )
            .push(self.bezier.view(&self.curves).map(Message::AddCurve))
            .push(
                Button::new(&mut self.button_state, Text::new("Clear"))
                    .padding(8)
                    .on_press(Message::Clear),
            )
            .into()
    }
}
