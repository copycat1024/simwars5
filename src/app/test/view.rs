use crate::widget::Menu;
use soyo::{
    mvc::View,
    tui::Context,
    widget::{Label, Layer, Widget},
};

pub struct TestView {
    screen: Layer,
    top: Widget<Label>,
    menu: Widget<Menu>,
}

impl Default for TestView {
    fn default() -> Self {
        Self {
            screen: Layer::screen(0, 0),
            top: Widget::new(Label::default()),
            menu: Widget::new(Menu::default()),
        }
    }
}

impl View for TestView {
    fn setup(&mut self) {
        self.top.composer.set(|layer| layer.set_h(1).rise_z());
        self.menu
            .composer
            .set(|layer| layer.center(layer.w / 3, layer.h - 20).rise_z());
        self.menu.widget.set_list(["a", "b"]);
    }

    fn resize(&mut self, w: i32, h: i32) {
        self.screen = Layer::screen(w, h);
    }

    fn render(&self, ctx: &mut Context) {
        self.top.render(ctx, self.screen);
        self.menu.render(ctx, self.screen);
    }
}

impl TestView {
    pub fn write_top(&mut self, text: &str) {
        let top = &mut self.top.widget;
        write!(
            top.text,
            "Size {} {} | {}",
            self.screen.w, self.screen.h, text
        );
    }
}
