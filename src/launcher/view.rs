use crate::widget::Menu;
use soyo::{
    mvc::View,
    tui::Context,
    widget::{Label, Layer, Widget},
};

pub struct LauncherView {
    screen: Layer,
    top: Widget<Label>,
    menu: Widget<Menu>,
}

impl Default for LauncherView {
    fn default() -> Self {
        Self {
            screen: Layer::screen(0, 0),
            top: Widget::new(Label::default()),
            menu: Widget::new(Menu::default()),
        }
    }
}

impl View for LauncherView {
    fn setup(&mut self) {
        self.top.composer.set(|layer| layer.set_h(1).rise_z());
        self.menu.composer.set(|layer| {
            layer
                .set_x(layer.w / 3)
                .set_y(5)
                .set_w(layer.w / 3)
                .set_h(layer.h - 11)
                .rise_z()
        });
    }

    fn resize(&mut self, w: i32, h: i32) {
        self.screen = Layer::screen(w, h);
    }

    fn render(&self, ctx: &mut Context) {

        self.top.render(ctx, self.screen);
        self.menu.render(ctx, self.screen);
    }
}

impl LauncherView {
    pub fn set_menu<'a, T>(&mut self, iter: T)
    where
        T: IntoIterator<Item = &'a str>,
    {
        self.menu.widget.set_list(iter);
    }

    pub fn set_item(&mut self, item: usize) {
        self.menu.widget.set_item(item);
    }

    pub fn write_top(&mut self, text: &str) {
        let top = &mut self.top.widget;
        write!(top.text, "{}", text);
    }
}
