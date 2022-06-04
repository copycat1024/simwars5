use crate::app::common::{Renderable, Text};
use soyo::tui::{Color, Letter, Quad};

pub struct Label {
    pub text: Text,
}

impl Label {
    pub fn new() -> Self {
        Self { text: Text::new() }
    }

    fn align(&self, pos: Quad) -> i32 {
        let w1 = self.text.data.len() as i32;
        let w2 = pos.w;
        (w2 - w1) / 2
    }
}

impl Renderable for Label {
    fn render(&self, quad: Quad, letter: &mut Letter) {
        *letter.c = self.text[quad.x - self.align(quad)];
        *letter.bg = Color::RED;
    }
}
