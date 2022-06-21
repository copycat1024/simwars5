use crate::{
    mvc::View,
    view::{Attribute, Compose, Node, NodeList, NodeRef, Render},
};
use soyo::{
    tui::{Color, Context, Letter, Quad},
    util::FlexVec,
};

pub struct TestView {
    root: Node,
}

impl Default for TestView {
    fn default() -> Self {
        let view = TestComposer::new();
        Self {
            root: Node::root(view).0,
        }
    }
}

impl View for TestView {
    fn setup(&mut self) {}

    fn resize(&mut self, w: i32, h: i32) {
        self.root.resize(w, h);
        self.root.compose();
    }

    fn render(&self, ctx: &mut Context) {
        self.root.render(ctx);
    }
}

struct TestComposer {
    bullet: NodeRef<Bullet>,
}

impl TestComposer {
    fn new() -> Self {
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
