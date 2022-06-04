use super::TestState;
use crate::flux::common::{Label, Widget};
use soyo::tui::{Context, Quad};

pub struct TestView {
    pos: Quad,
    top: Widget<Label>,
}

impl TestView {
    pub fn new() -> Self {
        Self {
            pos: Quad::xywh(0, 0, 0, 0),
            top: Widget::new(Label::new()),
        }
    }

    pub fn render(&self, ctx: &mut Context) {
        self.top.render(ctx, self.pos);
    }

    pub fn on_resize(&mut self, w: i32, h: i32) {
        self.pos.w = w;
        self.pos.h = h;
    }

    pub fn on_start(&mut self) {
        self.top.composer.set(|quad, z| {
            (
                Quad {
                    x: 0,
                    y: 0,
                    w: quad.w,
                    h: 1,
                },
                z + 1,
            )
        });
    }

    pub fn update(&mut self, state: &mut TestState) {
        let top = &mut self.top.widget;
        write!(
            top.text,
            "abcdefghijklmno {} {} abcdefghijklmno abcdefghijklmno abcdefghijklmno",
            self.pos.w, self.pos.h
        );
    }
}
