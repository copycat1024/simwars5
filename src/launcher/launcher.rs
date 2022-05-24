use super::TopBar;
use crate::{
    app::{App, TestApp},
    tag::Tag,
};
use soyo::{
    logger::{Client, Server},
    tui::{backend::Vt100, Event, Key},
    util::Result,
};
use std::io::{stdout, Write};

pub type Context = soyo::tui::Context;

pub fn launch() -> Result {
    let log = get_logger();

    Launcher::new(&log).run()?;

    log.print_raw();

    Ok(())
}

struct Launcher {
    ctx: Context,
    log: Client,
    bar: TopBar,
}

impl Launcher {
    pub fn new(log: &Server) -> Self {
        Self {
            ctx: get_context(log),
            log: log.client(Tag::Launcher),
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
        writeln!(self.log, "App start")?;
        let mut app = TestApp::new();
        app.run(&mut self.ctx, &mut self.log)?;
        writeln!(self.log, "App end")?;

        Ok(())
    }
}

fn get_context(log: &Server) -> Context {
    let vt100 = Vt100::new(stdout());
    Context::compose(vt100, Some(log))
}

fn get_logger() -> Server {
    let mut log = Server::default();

    // enable framework log
    // log.enable(soyo::logger::Tag::Event);

    // enable application log
    log.enable(Tag::Launcher);

    log
}
