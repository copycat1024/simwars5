use crate::view::Render;
use soyo::{
    tui::{Letter, Quad},
    util::FlexVec,
};

pub struct Army {
    pub symbol: char,
    pub name: FlexVec<char>,
}

impl Army {
    pub fn new(symbol: char) -> Self {
        Self {
            symbol,
            name: FlexVec::new(' '),
        }
    }
}

impl Default for Army {
    fn default() -> Self {
        Self {
            symbol: ' ',
            name: FlexVec::new(' '),
        }
    }
}

impl Render for Army {
    fn render(&self, quad: Quad, letter: &mut Letter) {
        *letter.c = self.symbol;
    }
}
