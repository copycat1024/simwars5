use crate::app::common::Text;
use soyo::tui::{Color, Context, Rect};

pub struct Label {
    pub rect: Rect,
    pub text: Text,
}

impl Label {
    pub fn new() -> Self {
        Self {
            rect: Rect::xywh(0, 0, 20, 2),
            text: Text::new(),
        }
    }

    pub fn render(&self, ctx: &mut Context) {
        ctx.render(self.rect, 2, |x, _y, letter| {
            *letter.c = self.text[x - self.align()];
            *letter.bg = Color::RED;
        });
    }

    fn align(&self) -> i32 {
        let w1 = self.text.data.len() as i32;
        let w2 = self.rect.w as i32;
        (w2 - w1) / 2
    }
}
