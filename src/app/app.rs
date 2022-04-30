use soyo::{
    logger::Client,
    tui::{backend::Backend, Context, Event, Key},
    util::Result,
};
use std::time::Duration;

pub struct AppControl<'a> {
    pub log: &'a mut Client,
    pub running: bool,
}

impl<'a> AppControl<'a> {
    fn new(log: &'a mut Client) -> Self {
        Self { log, running: true }
    }
}

pub trait App: Sized {
    fn on_start(&mut self, _ctrl: &mut AppControl) {}
    fn on_end(&mut self, _ctrl: &mut AppControl) {}
    fn on_resize(&mut self, _ctrl: &mut AppControl, _w: i32, _h: i32) {}
    fn on_key(&mut self, _ctrl: &mut AppControl, _key: Key) {}
    fn on_update(&mut self, _ctrl: &mut AppControl, _delta: Duration) {}

    fn render<B: Backend>(&mut self, ctx: &mut Context<B>);

    fn run<B: Backend>(&mut self, ctx: &mut Context<B>, log: &mut Client) -> Result {
        // init app
        let mut ctrl_obj = AppControl::new(log);
        let ctrl = &mut ctrl_obj;
        let (w, h) = ctx.size();

        self.on_start(ctrl);
        self.on_resize(ctrl, w, h);
        ctx.clear()?;

        // main loop
        loop {
            if let Some(e) = ctx.event()? {
                match e {
                    Event::Key { key } => self.on_key(ctrl, key),
                    Event::Resize { w, h } => self.on_resize(ctrl, w, h),
                    Event::Update { delta } => self.on_update(ctrl, delta),
                }

                self.render(ctx);
                ctx.draw()?;
            }

            if !ctrl.running {
                break;
            }
        }

        // clean up app
        self.on_end(ctrl);
        ctx.clear()
    }
}
