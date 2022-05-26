use soyo::tui::{Color, Context, Rect};
use std::fmt::Arguments;

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

pub struct Text {
    pub data: Vec<char>,
}

const FILL_CHAR: char = ' ';

impl Text {
    fn new() -> Self {
        Self { data: Vec::new() }
    }

    pub fn write_fmt(&mut self, fmt: Arguments<'_>) {
        self.data = format!("{}", fmt).chars().collect();
    }
}

impl std::ops::Index<i32> for Text {
    type Output = char;

    fn index(&self, i: i32) -> &Self::Output {
        self.data.get(i as usize).unwrap_or(&FILL_CHAR)
    }
}
