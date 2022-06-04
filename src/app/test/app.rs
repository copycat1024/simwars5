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
    fn on_start(&mut self, _ctrl: &mut AppControl) {
        self.view.on_start();
    }

    fn on_key(&mut self, ctrl: &mut AppControl, key: Key) {
        if key == Key::ESC {
            ctrl.stop = true;
        }
    }

    fn on_resize(&mut self, _: &mut AppControl, w: i32, h: i32) {
        self.state.pos.w = w;
        self.state.pos.h = h;
    }

    fn update(&mut self, _: &mut AppControl) {
        let Self { view, state } = self;
        view.update(state);
    }

    fn render(&self, ctx: &mut Context) {
        let Self { view, state } = self;
        view.render(ctx, state.pos);
    }
}
