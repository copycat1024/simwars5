use crate::app::{App, AppControl};
use soyo::tui::{backend::Backend, Color, Context, Key, Rect};

pub struct TestApp {}

impl TestApp {
    pub fn new() -> Self {
        Self {}
    }
}

impl App for TestApp {
    fn on_key(&mut self, ctrl: &mut AppControl, key: Key) {
        if key == Key::ESC {
            ctrl.running = false;
        }
    }

    fn render<B: Backend>(&mut self, ctx: &mut Context<B>) {
        let rect = Rect::xywh(0, 0, 16, 16);
        ctx.render(rect, 2, |x, y, letter| {
            let color = (x + 16 * y) as u8;
            *letter.c = ' ';
            *letter.bg = Color(color);
        });
    }
}
