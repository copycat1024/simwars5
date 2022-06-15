use crate::widget::{Bullet, Field};
use soyo::{
    mvc::View,
    tui::Context,
    widget::{Label, Layer, Widget},
};

pub struct BattleView {
    screen: Layer,
    top: Widget<Label>,
    field: Widget<Field>,
    bullet: Widget<Bullet>,
}

impl Default for BattleView {
    fn default() -> Self {
        Self {
            screen: Layer::screen(0, 0),
            top: Widget::new(Label::default()),
            field: Widget::new(Field::new(7, 3, 16, 8)),
            bullet: Widget::new(Bullet::new('\u{26e8}')),
            // bullet: Widget::new(Bullet::new('\u{2694}')),
        }
    }
}

impl View for BattleView {
    fn setup(&mut self) {
        self.top.composer.set(|layer| layer.set_h(1).rise_z());

        let (tw, th) = self.field.widget.get_wh();
        self.field
            .composer
            .set(move |layer| layer.center(tw, th).rise_z());
        self.bullet
            .composer
            .set(move |layer| layer.set_x(2).set_y(2).set_w(10).set_h(1).rise_z());
    }

    fn resize(&mut self, w: i32, h: i32) {
        self.screen = Layer::screen(w, h);
    }

    fn render(&self, ctx: &mut Context) {
        self.top.render(ctx, self.screen);
        self.field.render(ctx, self.screen);
        self.bullet.render(ctx, self.screen);
    }
}

impl BattleView {
    pub fn set_cell(&mut self, cell: u8) {
        let top = &mut self.top.widget;
        let bullet = &mut self.bullet.widget;

        write!(top.text, "Cell {}", cell);
        write!(bullet.text, "100");
    }
}
