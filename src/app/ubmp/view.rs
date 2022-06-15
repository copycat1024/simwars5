use crate::widget::Utable;
use soyo::{
    mvc::View,
    tui::Context,
    widget::{Label, Layer, Widget},
};

pub struct UbmpView {
    screen: Layer,
    top: Widget<Label>,
    table: Widget<Utable>,
}

impl Default for UbmpView {
    fn default() -> Self {
        Self {
            screen: Layer::screen(0, 0),
            top: Widget::new(Label::default()),
            table: Widget::new(Utable::default()),
        }
    }
}

impl View for UbmpView {
    fn setup(&mut self) {
        self.top.composer.set(|layer| layer.set_h(1).rise_z());

        let (tw, th) = self.table.widget.get_wh();
        self.table
            .composer
            .set(move |layer| layer.center(tw, th).rise_z());
    }

    fn resize(&mut self, w: i32, h: i32) {
        self.screen = Layer::screen(w, h);
    }

    fn render(&self, ctx: &mut Context) {
        self.top.render(ctx, self.screen);
        self.table.render(ctx, self.screen);
    }
}

impl UbmpView {
    pub fn set_cell(&mut self, cell: u8) {
        let top = &mut self.top.widget;
        let table = &mut self.table.widget;

        write!(top.text, "Cell {}", cell);
        table.set_cell(cell);
    }
}
