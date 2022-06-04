use soyo::tui::{Context, Letter, Quad};

pub trait LayoutFn: Fn(Quad) -> Quad + 'static {}
impl<F> LayoutFn for F where F: Fn(Quad) -> Quad + 'static {}

pub trait Renderable {
    fn render(&self, quad: Quad, letter: &mut Letter);
}

pub struct Widget<T: Renderable> {
    pub z: i32,
    pub widget: T,
    layout: Box<dyn LayoutFn>,
}

impl<T: Renderable> Widget<T> {
    pub fn new(widget: T) -> Self {
        Self {
            z: 1,
            widget,
            layout: Box::new(default_layout),
        }
    }

    pub fn render(&self, ctx: &mut Context, quad: Quad) {
        let pos = (self.layout)(quad);
        ctx.render(pos, self.z, |q, l| self.widget.render(q, l));
    }

    pub fn set_layout<F: LayoutFn>(&mut self, func: F) {
        self.layout = Box::new(func);
    }
}

fn default_layout(quad: Quad) -> Quad {
    quad
}
