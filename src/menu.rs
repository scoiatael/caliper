use iced::{button, Button, Element, Length, Row, Svg};

#[derive(Default)]
pub struct State {
    save: button::State,
    open: button::State,
    clear: button::State,
    export: button::State,
    side_bar: button::State,
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    Clear,
    Sidebar,
}

fn icon<A>(name: &str) -> Element<A> {
    Svg::from_path(format!(
        "{}/resources/fontawesome-free-5.15.3-desktop/svgs/{}.svg",
        env!("CARGO_MANIFEST_DIR"),
        name
    ))
    .width(Length::Units(20))
    .height(Length::Units(20))
    .into()
}

impl State {
    pub fn view(&mut self) -> Element<Message> {
        Row::new()
            .push(
                Button::new(&mut self.save, icon("solid/save"))
                    .padding(8)
                    .on_press(Message::Clear),
            )
            .push(
                Button::new(&mut self.open, icon("solid/folder-open"))
                    .padding(8)
                    .on_press(Message::Clear),
            )
            .push(
                Button::new(&mut self.clear, icon("solid/recycle"))
                    .padding(8)
                    .on_press(Message::Clear),
            )
            .push(
                Button::new(&mut self.export, icon("solid/file-export"))
                    .padding(8)
                    .on_press(Message::Clear),
            )
            .push(
                Button::new(&mut self.side_bar, icon("solid/list"))
                    .padding(8)
                    .on_press(Message::Sidebar),
            )
            .into()
    }
}
