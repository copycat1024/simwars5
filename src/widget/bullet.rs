use crate::view::Render;
use soyo::{
    tui::{Letter, Quad},
    util::FlexVec,
};

pub struct Bullet {
    bullet: char,
    pub text: FlexVec<char>,
}

impl Bullet {
    pub fn new(bullet: char) -> Self {
        Self {
            bullet,
            text: FlexVec::new(' '),
        }
    }
}

impl Default for Bullet {
    fn default() -> Self {
        Self {
            bullet: ' ',
            text: FlexVec::new(' '),
        }
    }
}

impl Render for Bullet {
    fn render(&self, quad: Quad, letter: &mut Letter) {
        if quad.x == 0 {
            *letter.c = self.bullet;
        } else if quad.x < 3 {
            *letter.c = '\0';
        } else {
            *letter.c = self.text[quad.x - 3]
        }
    }
}
