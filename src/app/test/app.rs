use super::{TestState, TestView};
use crate::app::{App, AppControl};
use soyo::tui::{Context, Key};
use std::time::Duration;

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
    }

    fn on_update(&mut self, _: &mut AppControl, _: Duration) {
        let Self { view, state } = self;
        view.update(state);
    }

    fn render(&self, ctx: &mut Context) {
        self.view.render(ctx);
    }
}
