use iced::{
    image, Align, Application, Clipboard, Column, Command, Container, Element, Length, Row,
    Settings, Text,
};
use iced_futures;
use std::path::PathBuf;

mod bezier;
mod canvas_over_image;
mod dialog;
mod disk;
mod export;
mod menu;

pub fn main() -> iced::Result {
    let path = dialog::open_file().expect("failed open dialog");

    AppState::run(Settings {
        antialiasing: true,
        flags: (Flags {
            file: Option::Some(path),
        }),
        ..Settings::default()
    })
}

struct AppState {
    path: PathBuf,
    bezier: bezier::State,
    curves: bezier::Curves,
    img: image::Handle,
    menu: menu::State,
    sidebar: bool,
}

impl From<disk::Serialized> for AppState {
    fn from(s: disk::Serialized) -> Self {
        AppState {
            path: s.path.clone(),
            curves: s.curves(),
            img: image::Handle::from_path(s.path),
            bezier: bezier::State::default(),
            menu: menu::State::default(),
            sidebar: false,
        }
    }
}

impl Into<disk::Serialized> for &mut AppState {
    fn into(self) -> disk::Serialized {
        disk::Serialized::new(self.path.clone(), &self.curves)
    }
}

impl From<PathBuf> for AppState {
    fn from(path: PathBuf) -> Self {
        AppState {
            path: path.clone(),
            img: image::Handle::from_path(path),
            bezier: bezier::State::default(),
            curves: bezier::Curves::default(),
            menu: menu::State::default(),
            sidebar: false,
        }
    }
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
    type Executor = iced_futures::executor::Tokio;
    type Flags = Flags;

    fn new(flags: Self::Flags) -> (AppState, Command<Self::Message>) {
        let path = flags.file.expect("file missing?");
        match path.extension() {
            Some(str) => {
                if str == "clp" {
                    let serialized = disk::load(&path).expect("failed to load file");
                    (serialized.into(), Command::none())
                } else {
                    (path.into(), Command::none())
                }
            }
            _ => (path.into(), Command::none()),
        }
    }

    fn title(&self) -> String {
        String::from("Caliper - Iced")
    }

    fn update(&mut self, message: Message, _clipboard: &mut Clipboard) -> iced::Command<Message> {
        match message {
            Message::AddCurve(curve) => {
                self.curves.push(curve);
                self.bezier.request_redraw();
                Command::none()
            }
            Message::FromMenu(menu::Message::Clear) => {
                self.bezier = bezier::State::default();
                self.curves.clear();
                Command::none()
            }
            Message::FromMenu(menu::Message::Sidebar) => {
                self.sidebar = !self.sidebar;
                Command::none()
            }
            Message::FromMenu(menu::Message::Export) => {
                let mut dst = self.path.clone();
                dst.set_extension("clp.svg");
                let _ = export::run(&self.curves, &dst);
                Command::none()
            }
            Message::FromMenu(menu::Message::Save) => {
                let mut dst = self.path.clone();
                dst.set_extension("clp");
                // TODO: Let user know save failed x]
                let _ = disk::save(self.into(), &dst);
                Command::none()
            }
        }
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

        let overlay_with_sidebar: Element<Message> = if self.sidebar {
            let mut curves: Column<Message> = Column::new();
            for (idx, curve) in self.curves.iter().enumerate() {
                curves = curves.push(Text::new(format!("{} - {:?}", idx, curve)));
            }
            Row::new()
                .push(curves)
                .push(overlay.map(Message::AddCurve))
                .into()
        } else {
            overlay.map(Message::AddCurve)
        };

        Column::new()
            .padding(20)
            .spacing(20)
            .align_items(Align::Center)
            .push(self.menu.view().map(Message::FromMenu))
            .push(overlay_with_sidebar)
            .into()
    }
}
