use super::{Label, TestState};
use soyo::tui::{Color, Context, Key, Rect};

pub struct TestView {
    top: Label,
}

impl TestView {
    pub fn new() -> Self {
        Self { top: Label::new() }
    }

    pub fn render(&self, ctx: &mut Context) {
        self.top.render(ctx);
    }

    pub fn update(&mut self, state: &TestState) {
        write!(self.top.text, "{} {}", state.w, state.h);
    }
}
