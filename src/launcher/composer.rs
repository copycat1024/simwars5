use crate::{
    view::{Attribute, Compose, Label, NodeList, NodeRef},
    widget::Menu,
};

pub struct LauncherComposer {
    top: NodeRef<Label>,
    menu: NodeRef<Menu>,
}

impl LauncherComposer {
    pub fn new() -> Self {
        Self {
            top: NodeRef::default(),
            menu: NodeRef::default(),
        }
    }
}

impl Compose for LauncherComposer {
    fn register(&mut self, children: &mut NodeList) {
        let top = Label::default();
        let menu = Menu::default();

        self.top = children.register_renderer(top);
        self.menu = children.register_renderer(menu);
    }

    fn compose(&mut self, me: &Attribute, _: &mut NodeList) {
        self.top.compose(|child| {
            child.frame = me.frame.set_h(1).rise_z();
        });

        self.menu.compose(|child| {
            let frame = me.frame;
            child.frame = frame
                .set_x(frame.w / 3)
                .set_y(5)
                .set_w(frame.w / 3)
                .set_h(frame.h - 11)
                .rise_z()
        });
    }
}

impl LauncherComposer {
    pub fn set_menu<'a, T>(&mut self, iter: T)
    where
        T: IntoIterator<Item = &'a str>,
    {
        self.menu.view(|view| view.set_list(iter));
    }

    pub fn set_item(&mut self, item: usize) {
        self.menu.view(|view| view.set_item(item));
    }

    pub fn write_top(&mut self, text: &str) {
        self.top.view(|view| {
            write!(view.text, "{}", text);
        });
    }
}
