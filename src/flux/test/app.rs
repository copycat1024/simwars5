use std::collections::VecDeque;

use super::{TestState, TestView};
use crate::flux::{arch::State, App, AppControl};
use soyo::tui::{Context, Key};

pub struct TestApp<S>
where
    S: State,
{
    state_event: VecDeque<S::Event>,
    state: S,
    view: TestView,
}

impl<S> TestApp<S>
where
    S: State,
{
    pub fn new() -> Self {
        Self {
            state_event: VecDeque::new(),
            state: S::default(),
            view: TestView::new(),
        }
    }
}

impl App for TestApp<TestState> {
    fn on_start(&mut self, _ctrl: &mut AppControl) {
        self.view.on_start();
    }

    fn on_key(&mut self, ctrl: &mut AppControl, key: Key) {
        if key == Key::ESC {
            ctrl.stop = true;
        }
    }

    fn on_resize(&mut self, ctrl: &mut AppControl, w: i32, h: i32) {
        self.view.on_resize(w, h);
    }

    fn update(&mut self, _: &mut AppControl) {
        let Self { view, state, .. } = self;
        view.update(state);
    }

    fn render(&self, ctx: &mut Context) {
        let Self { view, state, .. } = self;
        view.render(ctx);
    }
}
