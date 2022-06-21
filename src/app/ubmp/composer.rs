use crate::{
    view::{Attribute, Compose, Label, NodeList, NodeRef},
    widget::Utable,
};

pub struct UbmpComposer {
    top: NodeRef<Label>,
    table: NodeRef<Utable>,
}

impl UbmpComposer {
    pub fn new() -> Self {
        Self {
            top: NodeRef::default(),
            table: NodeRef::default(),
        }
    }
}

impl Compose for UbmpComposer {
    fn register(&mut self, children: &mut NodeList) {
        let top = Label::default();
        let utable = Utable::default();

        self.top = children.register_renderer(top);
        self.table = children.register_renderer(utable);
    }

    fn compose(&mut self, me: &Attribute, _: &mut NodeList) {
        self.top.compose(|child| {
            child.frame = me.frame.set_h(1).rise_z();
        });

        if let Some((tw, th)) = self.table.view(|view| view.get_wh()) {
            self.table
                .compose(move |child| child.frame = me.frame.center(tw, th).rise_z());
        }
    }
}

impl UbmpComposer {
    pub fn set_cell(&mut self, cell: u8) {
        self.top.view(|view| {
            write!(view.text, "Cell {}", cell);
        });
        self.table.view(|view| {
            view.set_cell(cell);
        });
    }
}
