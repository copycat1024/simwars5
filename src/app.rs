use soyo::{
    tui::{backend::Backend, Color, Context, Event, Rect},
    util::{LoggerServer, Result},
};

pub struct App<B: Backend> {
    context: Context<B>,
}

impl<B: Backend> App<B> {
    pub fn new(backend: B, logger: Option<&LoggerServer>) -> Self {
        Self {
            context: Context::compose(backend, logger),
        }
    }

    pub fn run(self) -> Result {
        let Self { mut context } = self;
        context.clear()?;

        loop {
            if let Some(e) = context.event()? {
                match e {
                    Event::Key { .. } => {
                        return Ok(());
                    }
                    _ => {}
                }
            }
            let mut rect = Rect::new();
            rect.xywh(0, 0, 16, 16);
            context.render(rect, 2, |x, y, letter| {
                let color = (x + 16 * y) as u8;
                *letter.c = ' ';
                *letter.bg = Color(color);
            });
            context.draw()?;
        }
    }
}
