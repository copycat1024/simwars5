use super::UbmpComposer;
use crate::{
    mvc::View,
    view::{Node, NodeRef},
};
use soyo::tui::Context;

pub struct UbmpView {
    root: Node,
    root_ref: NodeRef<UbmpComposer>,
}

impl Default for UbmpView {
    fn default() -> Self {
        let (root, root_ref) = Node::root(UbmpComposer::new());
        Self { root, root_ref }
    }
}

impl View for UbmpView {
    fn setup(&mut self) {}

    fn resize(&mut self, w: i32, h: i32) {
        self.root.resize(w, h);
        self.root.compose();
    }

    fn render(&self, ctx: &mut Context) {
        self.root.render(ctx);
    }
}

impl UbmpView {
    pub fn set_cell(&mut self, cell: u8) {
        self.root_ref.view(|view| view.set_cell(cell));
    }
}
