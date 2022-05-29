use super::{TestState, TestView};
use crate::app::{App, AppControl};
use soyo::tui::{Context, Key};

pub struct TestApp {
    state: TestState,
    view: TestView,
}

impl TestApp {
    pub fn new() -> Self {
        Self {
            state: TestState::new(),
            view: TestView::new(),
        }
    }
}

impl App for TestApp {
    fn on_key(&mut self, ctrl: &mut AppControl, key: Key) {
        if key == Key::ESC {
            ctrl.stop = true;
        }
    }

    fn on_resize(&mut self, _: &mut AppControl, w: i32, h: i32) {
        self.state.w = w;
        self.state.h = h;
        use soyo::log::debug;
        writeln!(debug(), "{} {}", self.state.w, self.state.h);
    }

    fn update(&mut self, _: &mut AppControl) {
        let Self { view, state } = self;
        view.update(state);
    }

    fn render(&self, ctx: &mut Context) {
        self.view.render(ctx);
    }
}
