use super::BattleComposer;
use crate::{mvc::View, view::Node};
use soyo::tui::Context;

pub struct BattleView {
    root: Node,
}

impl Default for BattleView {
    fn default() -> Self {
        let (root, _) = Node::root(BattleComposer::new());
        Self { root }
    }
}

impl View for BattleView {
    fn setup(&mut self) {}

    fn resize(&mut self, w: i32, h: i32) {
        self.root.resize(w, h);
        self.root.compose();
    }

    fn render(&self, ctx: &mut Context) {
        self.root.render(ctx);

        // let cell = self.field.widget.get_cell(0, 0, 1);
        // for army in self.army.iter() {
        //     army.render(ctx, cell);
        // }
    }
}

impl BattleView {
    pub fn set_cell(&mut self, _: u8) {}
}
