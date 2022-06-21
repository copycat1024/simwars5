use super::LauncherComposer;
use crate::{
    mvc::View,
    view::{Node, NodeRef},
};
use soyo::tui::Context;

pub struct LauncherView {
    root: Node,
    root_ref: NodeRef<LauncherComposer>,
}

impl Default for LauncherView {
    fn default() -> Self {
        let (root, root_ref) = Node::root(LauncherComposer::new());
        Self { root, root_ref }
    }
}

impl View for LauncherView {
    fn setup(&mut self) {}

    fn resize(&mut self, w: i32, h: i32) {
        self.root.resize(w, h);
        self.root.compose();
    }

    fn render(&self, ctx: &mut Context) {
        self.root.render(ctx);
    }
}

impl LauncherView {
    pub fn set_menu<'a, T>(&mut self, iter: T)
    where
        T: IntoIterator<Item = &'a str>,
    {
        self.root_ref.view(|view| view.set_menu(iter));
    }

    pub fn set_item(&mut self, item: usize) {
        self.root_ref.view(|view| view.set_item(item));
    }

    pub fn write_top(&mut self, text: &str) {
        self.root_ref.view(|view| view.write_top(text));
    }
}
