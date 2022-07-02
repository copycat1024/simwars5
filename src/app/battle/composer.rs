use crate::{
    view::{Attribute, Compose, NodeList, NodeRef},
    widget::Grid,
};

pub struct BattleComposer {
    field: NodeRef<Grid>,
}

impl Default for BattleComposer {
    fn default() -> Self {
        Self {
            field: NodeRef::default(),
        }
    }
}

impl Compose for BattleComposer {
    fn register(&mut self, children: &mut NodeList) {
        let field = Grid::new(7, 3, 16, 8);
        self.field = children.register_renderer(field);
    }

    fn compose(&mut self, me: &Attribute, _: &mut NodeList) {
        if let Some((tw, th)) = self.field.view(|view| view.get_wh()) {
            self.field
                .compose(move |child| child.frame = me.frame.center(tw, th).rise_z());
        }
    }
}
