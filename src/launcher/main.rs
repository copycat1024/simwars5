use super::TopBar;
use crate::{
    app::{App, TestApp},
    tag::Tag,
};
use soyo::{
    logger::{activate_logger, flush_logger, log},
    tui::{backend::Vt100, Event, Key},
    util::Result,
};
use std::io::stdout;

pub type Context = soyo::tui::Context;

pub fn launch() -> Result {
    // enable framework logger
    // activate_logger(soyo::logger::Tag::Event);

    // enable application logger
    activate_logger(Tag::Launcher);

    // create context
    let vt100 = Vt100::new(stdout());
    let ctx = Context::new(vt100);

    Launcher::new(ctx).run()?;

    flush_logger();

    Ok(())
}

struct Launcher {
    ctx: Context,
    bar: TopBar,
}

impl Launcher {
    pub fn new(ctx: Context) -> Self {
        Self {
            ctx,
            bar: TopBar::new(),
        }
    }

    pub fn run(mut self) -> Result {
        let mut running = true;
        self.ctx.clear()?;

        loop {
            if let Some(e) = self.ctx.event()? {
                match e {
                    Event::Key { key } => self.on_key(key, &mut running)?,
                    Event::Resize { w, h } => self.on_resize(w, h),
                    _ => {}
                }

                self.render();
                self.ctx.draw()?;
            }

            if !running {
                return Ok(());
            }
        }
    }

    fn on_key(&mut self, key: Key, running: &mut bool) -> Result {
        if key == Key::ESC {
            *running = false;
        } else if key == Key::ENTER {
            self.start_app()?;
        }
        Ok(())
    }

    fn on_resize(&mut self, w: i32, h: i32) {
        self.bar.on_resize(w, h);
    }

    fn render(&mut self) {
        self.bar.render(&mut self.ctx);
    }

    fn start_app(&mut self) -> Result {
        writeln!(log(Tag::Launcher), "App start");
        let mut app = TestApp::new();
        app.run(&mut self.ctx)?;
        writeln!(log(Tag::Launcher), "App end");

        Ok(())
    }
}
