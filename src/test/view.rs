use soyo::{
    mvc::View,
    tui::{Context, Quad},
    widget::{Label, Widget},
};

pub struct TestView {
    pos: Quad,
    top: Widget<Label>,
}

impl Default for TestView {
    fn default() -> Self {
        Self {
            pos: Quad::xywh(0, 0, 0, 0),
            top: Widget::new(Label::default()),
        }
    }
}

impl View for TestView {
    fn setup(&mut self) {
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

    fn resize(&mut self, w: i32, h: i32) {
        self.pos.w = w;
        self.pos.h = h;
    }

    fn render(&self, ctx: &mut Context) {
        self.top.render(ctx, self.pos);
    }
}

impl TestView {
    pub fn write_top(&mut self, text: &str) {
        let top = &mut self.top.widget;
        write!(top.text, "Size {} {} | {}", self.pos.w, self.pos.h, text);
    }
}
