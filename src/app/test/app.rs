use super::TestState;
use crate::app::{App, AppControl};
use soyo::tui::{Color, Context, Key, Rect};

pub struct TestApp {
    state: TestState,
}

impl TestApp {
    pub fn new() -> Self {
        Self {
            state: TestState::new(),
        }
    }
}

impl App for TestApp {
    fn on_key(&mut self, ctrl: &mut AppControl, key: Key) {
        if key == Key::ESC {
            ctrl.stop = true;
        }
    }

    fn on_resize(&mut self, ctrl: &mut AppControl, w: i32, h: i32) {
        self.state.w = w;
        self.state.h = h;
        ctrl.draw = true;
    }

    fn render(&mut self, ctx: &mut Context) {
        let rect = Rect::xywh(0, 0, 16, 16);
        // ctx.render(rect, 2, |x, y, letter| {
        //     let color = (x + 16 * y) as u8;
        //     *letter.c = ' ';
        //     *letter.bg = Color(color);
        // });
    }
}
