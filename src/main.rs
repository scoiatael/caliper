use iced::{
    executor, image, Align, Application, Clipboard, Column, Command, Container, Element, Length,
    Settings,
};
use nfd2::Response;

mod bezier;
mod canvas_over_image;
mod menu;

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

    AppState::run(Settings {
        antialiasing: true,
        flags: (Flags {
            file: Option::Some(path),
        }),
        ..Settings::default()
    })
}

struct AppState {
    bezier: bezier::State,
    curves: Vec<bezier::Curve>,
    img: image::Handle,
    menu: menu::State,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    AddCurve(bezier::Curve),
    FromMenu(menu::Message),
}

#[derive(Debug, Clone, Default)]
struct Flags {
    file: Option<std::path::PathBuf>,
}

impl Application for AppState {
    type Message = Message;
    type Executor = executor::Default;
    type Flags = Flags;

    fn new(flags: Self::Flags) -> (AppState, Command<Self::Message>) {
        let default = AppState {
            img: image::Handle::from_path(flags.file.expect("file missing?")),
            bezier: bezier::State::default(),
            curves: Vec::default(),
            menu: menu::State::default(),
        };
        (default, Command::none())
    }

    fn title(&self) -> String {
        String::from("Caliper - Iced")
    }

    fn update(&mut self, message: Message, _clipboard: &mut Clipboard) -> iced::Command<Message> {
        match message {
            Message::AddCurve(curve) => {
                self.curves.push(curve);
                self.bezier.request_redraw();
            }
            Message::FromMenu(menu::Message::Clear) => {
                self.bezier = bezier::State::default();
                self.curves.clear();
            }
        };
        Command::none()
    }

    fn view(&mut self) -> Element<Message> {
        let img: Element<bezier::Curve> = Container::new(image::Image::new(self.img.clone()))
            .width(Length::Fill)
            .height(Length::Fill)
            .into();

        let overlay: Element<bezier::Curve> = canvas_over_image::CanvasOverImage::new(
            Container::new(self.bezier.view(&self.curves))
                .width(Length::Fill)
                .height(Length::Fill)
                .into(),
            img,
        )
        .into();

        Column::new()
            .padding(20)
            .spacing(20)
            .align_items(Align::Center)
            .push(self.menu.view().map(Message::FromMenu))
            .push(overlay.map(Message::AddCurve))
            .into()
    }
}
