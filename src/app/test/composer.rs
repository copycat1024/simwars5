use crate::{
    mvc::View,
    view::{Attribute, Compose, Node, NodeList, NodeRef, Render},
};
use soyo::{
    tui::{Color, Context, Letter, Quad},
    util::FlexVec,
};

pub struct TestComposer {
    bullet: NodeRef<Bullet>,
}

impl Default for TestComposer {
    fn default() -> Self {
        Self {
            bullet: NodeRef::default(),
        }
    }
}

impl Compose for TestComposer {
    fn register(&mut self, children: &mut NodeList) {
        let bullet = Bullet::new('-');
        self.bullet = children.register_renderer(bullet);
    }

    fn compose(&mut self, me: &Attribute, _: &mut NodeList) {
        self.bullet.compose(|child| {
            child.frame = me.frame.set_h(1);
        })
    }
}

pub struct Bullet {
    pub text: FlexVec<char>,
}

impl Bullet {
    pub fn new(fill: char) -> Self {
        Self {
            text: FlexVec::new(fill),
        }
    }
}

impl Render for Bullet {
    fn render(&self, quad: Quad, letter: &mut Letter) {
        *letter.bg = Color::BLUE;
        *letter.c = self.text[quad.x - 3];
    }
}
