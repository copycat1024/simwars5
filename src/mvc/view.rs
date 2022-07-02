use crate::view::{Compose, Node, NodeRef};
use soyo::tui::Context;

pub struct View<T: 'static + Default> {
    root: Node,
    root_ref: NodeRef<T>,
}

impl<T: 'static + Default + Compose> Default for View<T> {
    fn default() -> Self {
        let (root, root_ref) = Node::root(T::default());
        Self { root, root_ref }
    }
}

impl<T: 'static + Default + Compose> View<T> {
    pub fn setup(&mut self) {}

    pub fn resize(&mut self, w: i32, h: i32) {
        self.root.resize(w, h);
        self.root.compose();
    }

    pub fn render(&self, ctx: &mut Context) {
        self.root.render(ctx);
    }

    pub fn node(&self) -> &NodeRef<T> {
        &self.root_ref
    }

    pub fn node_mut(&mut self) -> &mut NodeRef<T> {
        &mut self.root_ref
    }
}
