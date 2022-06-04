use super::TestState;
use crate::app::common::{Label, Widget};
use soyo::tui::{Context, Quad};

pub struct TestView {
    top: Widget<Label>,
}

impl TestView {
    pub fn new() -> Self {
        Self {
            top: Widget::new(Label::new()),
        }
    }

    pub fn render(&self, ctx: &mut Context, quad: Quad) {
        self.top.render(ctx, quad);
    }

    pub fn on_start(&mut self) {
        self.top.set_layout(|quad: Quad| Quad {
            x: 0,
            y: 0,
            w: quad.w,
            h: 1,
        });
    }

    pub fn update(&mut self, state: &TestState) {
        let top = &mut self.top.widget;
        write!(top.text, "{} {}", state.pos.w, state.pos.h);
    }
}
