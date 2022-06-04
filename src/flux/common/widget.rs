use super::composer::Composer;
use soyo::tui::{Context, Letter, Quad};

pub trait Renderable {
    fn render(&self, quad: Quad, letter: &mut Letter);
}

pub struct Widget<T: Renderable> {
    pub widget: T,
    pub composer: Composer,
}

impl<T: Renderable> Widget<T> {
    pub fn new(widget: T) -> Self {
        Self {
            widget,
            composer: Composer::new(),
        }
    }

    pub fn render(&self, ctx: &mut Context, quad: Quad) {
        let (quad, z) = self.composer.compose(quad, 0);
        ctx.render(quad, z, |q, l| self.widget.render(q, l));
    }
}
