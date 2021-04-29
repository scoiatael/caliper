use iced::{
    executor, image, Align, Application, Clipboard, Column, Command, Container, Element, Length,
    Row, Settings, Text,
};

mod bezier;
mod canvas_over_image;
mod dialog;
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
    bezier: bezier::State,
    curves: Vec<bezier::Curve>,
    img: image::Handle,
    menu: menu::State,
    sidebar: bool,
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
            sidebar: false,
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
            Message::FromMenu(menu::Message::Sidebar) => {
                self.sidebar = !self.sidebar;
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
