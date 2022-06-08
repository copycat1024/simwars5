use soyo::{
    tui::{Color, Letter, Quad},
    widget::{FlexVec, Render},
};

pub struct Menu {
    item: i32,
    list: FlexVec<FlexVec<char>>,
}

impl Menu {
    fn align(&self, i: i32, pos: Quad) -> i32 {
        let w1 = self.list[i].len() as i32;
        let w2 = pos.w;
        (w2 - w1) / 2
    }

    pub fn set_list<'a, T>(&mut self, iter: T)
    where
        T: IntoIterator<Item = &'a str>,
    {
        self.list = FlexVec::from_iter(
            iter.into_iter().map(FlexVec::<char>::text),
            FlexVec::new(' '),
        );
    }

    pub fn set_item(&mut self, item: usize) {
        self.item = item as i32;
    }
}

impl Default for Menu {
    fn default() -> Self {
        Self {
            item: 0,
            list: FlexVec::new(FlexVec::new(' ')),
        }
    }
}

impl Render for Menu {
    fn render(&self, quad: Quad, letter: &mut Letter) {
        let text = &self.list[quad.y];

        *letter.c = text[quad.x - self.align(quad.y, quad)];
        *letter.bg = if quad.y == self.item {
            Color::BLUE
        } else {
            Color::RED
        };
    }
}
