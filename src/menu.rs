use iced::widget::svg::Handle;
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
    Export,
    Save,
}

fn icon<A>(bytes: &[u8]) -> Element<A> {
    Svg::new(Handle::from_memory(bytes))
        .width(Length::Units(20))
        .height(Length::Units(20))
        .into()
}

impl State {
    pub fn view(&mut self) -> Element<Message> {
        Row::new()
            .push(
                Button::new(
                    &mut self.save,
                    icon(include_bytes!(
                        "fontawesome-free-5.15.3-desktop/svgs/solid/save.svg"
                    )),
                )
                .padding(8)
                .on_press(Message::Save),
            )
            .push(
                Button::new(
                    &mut self.open,
                    icon(include_bytes!(
                        "fontawesome-free-5.15.3-desktop/svgs/solid/folder-open.svg"
                    )),
                )
                .padding(8)
                .on_press(Message::Clear),
            )
            .push(
                Button::new(
                    &mut self.clear,
                    icon(include_bytes!(
                        "fontawesome-free-5.15.3-desktop/svgs/solid/recycle.svg"
                    )),
                )
                .padding(8)
                .on_press(Message::Clear),
            )
            .push(
                Button::new(
                    &mut self.export,
                    icon(include_bytes!(
                        "fontawesome-free-5.15.3-desktop/svgs/solid/file-export.svg"
                    )),
                )
                .padding(8)
                .on_press(Message::Export),
            )
            .push(
                Button::new(
                    &mut self.side_bar,
                    icon(include_bytes!(
                        "fontawesome-free-5.15.3-desktop/svgs/solid/list.svg"
                    )),
                )
                .padding(8)
                .on_press(Message::Sidebar),
            )
            .into()
    }
}
