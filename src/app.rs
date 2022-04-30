use soyo::{
    logger::Client,
    tui::{backend::Backend, Color, Context, Event, Rect},
    util::Result,
};
use std::io::Write;

pub struct TestApp {}

impl TestApp {
    pub fn new() -> Self {
        Self {}
    }

    pub fn run<B: Backend>(self, ctx: &mut Context<B>, log: &mut Client) -> Result {
        writeln!(log, "Clear");
        ctx.clear()?;

        loop {
            if let Some(e) = ctx.event()? {
                match e {
                    Event::Key { .. } => {
                        return Ok(());
                    }
                    _ => {}
                }
            }
            let mut rect = Rect::new();
            rect.xywh(0, 0, 16, 16);
            ctx.render(rect, 2, |x, y, letter| {
                let color = (x + 16 * y) as u8;
                *letter.c = ' ';
                *letter.bg = Color(color);
            });
            ctx.draw()?;
        }
    }
}
