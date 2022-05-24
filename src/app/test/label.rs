use soyo::tui::{Color, Context, Rect};

pub struct Label {
    pub rect: Rect,
    pub text: Vec<char>,
}

impl Label {
    pub fn new() -> Self {
        Self {
            rect: Rect::xywh(0, 0, 0, 1),
            text: Vec::new(),
        }
    }

    pub fn render(&self, ctx: &mut Context) {
        ctx.render(self.rect, 2, |x, _y, letter| {
            *letter.c = self.get_char(x - self.align());
            *letter.bg = Color::RED;
        });
    }

    pub fn on_resize(&mut self, w: i32, h: i32) {
        self.rect.w = w;
        self.h = h;
        self.content = format!("w: {w}, h: {h}").chars().collect();
    }

    fn align(&self) -> i32 {
        let w1 = self.text.len() as i32;
        let w2 = self.rect.w as i32;
        (w2 - w1) / 2
    }

    fn get_char(&self, i: i32) -> char {
        self.content.get(i as usize).map(|c| *c).unwrap_or(' ')
    }
}

struct Text {
    pub data: Vec<char>,
}

impl From<&str> for Text {}
