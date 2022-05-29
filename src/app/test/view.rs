use super::TestState;
use crate::app::common::Label;
use soyo::tui::Context;

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
        use soyo::log::debug;
        writeln!(debug(), "{} {}", state.w, state.h);
        write!(self.top.text, "{} {}", state.w, state.h);
    }
}
