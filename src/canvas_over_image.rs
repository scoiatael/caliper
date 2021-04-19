use iced_graphics::{Backend, Defaults, Primitive, Renderer};
use iced_native::{
    event::{self, Event},
    layout, mouse, Clipboard, Element, Hasher, Layout, Length, Point, Rectangle, Widget,
};

pub struct CanvasOverImage<'a, Message: 'a, B: Backend> {
    canvas: Element<'a, Message, Renderer<B>>,
    image: Element<'a, Message, Renderer<B>>,
}

impl<'a, Message, B: Backend> CanvasOverImage<'a, Message, B>
where
    B: Backend,
{
    pub fn new(
        canvas: Element<'a, Message, Renderer<B>>,
        image: Element<'a, Message, Renderer<B>>,
    ) -> CanvasOverImage<'a, Message, B> {
        CanvasOverImage {
            canvas: canvas,
            image: image,
        }
    }
}

impl<'a, Message, B> Widget<Message, Renderer<B>> for CanvasOverImage<'a, Message, B>
where
    B: Backend,
{
    fn width(&self) -> Length {
        self.canvas.width()
    }

    fn height(&self) -> Length {
        self.canvas.height()
    }

    fn layout(&self, renderer: &Renderer<B>, limits: &layout::Limits) -> layout::Node {
        let canvas = self.canvas.layout(renderer, limits);
        // let image = self.image.layout(renderer, limits);
        canvas
    }

    fn hash_layout(&self, state: &mut Hasher) {
        self.canvas.hash_layout(state);
    }

    fn on_event(
        &mut self,
        event: Event,
        layout: Layout<'_>,
        cursor_position: Point,
        renderer: &Renderer<B>,
        clipboard: &mut dyn Clipboard,
        messages: &mut Vec<Message>,
    ) -> event::Status {
        self.canvas.on_event(
            event,
            layout,
            cursor_position,
            renderer,
            clipboard,
            messages,
        )
    }

    fn draw(
        &self,
        renderer: &mut Renderer<B>,
        defaults: &Defaults,
        layout: Layout<'_>,
        cursor_position: Point,
        viewport: &Rectangle,
    ) -> (Primitive, mouse::Interaction) {
        use iced_native::Renderer;

        let base = self
            .image
            .draw(renderer, defaults, layout, cursor_position, viewport);
        let overlay = self
            .canvas
            .draw(renderer, defaults, layout, cursor_position, viewport);

        renderer.overlay(base, overlay, *viewport)
    }
}

impl<'a, Message: 'a, B: 'a> Into<Element<'a, Message, Renderer<B>>>
    for CanvasOverImage<'a, Message, B>
where
    B: Backend,
{
    fn into(self) -> Element<'a, Message, Renderer<B>> {
        Element::new(self)
    }
}
