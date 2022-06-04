use super::main::Context;
use soyo::tui::{Color, Quad};

pub struct TopBar {
    rect: Quad,
    h: i32,
    content: Vec<char>,
}

impl TopBar {
    pub fn new() -> Self {
        Self {
            rect: Quad::xywh(0, 0, 0, 1),
            h: 0,
            content: Vec::new(),
        }
    }

    pub fn render(&self, ctx: &mut Context) {
        ctx.render(self.rect, 2, |quad, letter| {
            *letter.c = self.get_char(quad.x - self.align());
            *letter.bg = Color::RED;
        });
    }

    pub fn on_resize(&mut self, w: i32, h: i32) {
        self.rect.w = w;
        self.h = h;
        self.content = format!("w: {w}, h: {h}").chars().collect();
    }

    fn align(&self) -> i32 {
        let w1 = self.content.len() as i32;
        let w2 = self.rect.w as i32;
        (w2 - w1) / 2
    }

    fn get_char(&self, i: i32) -> char {
        self.content.get(i as usize).copied().unwrap_or(' ')
    }
}
