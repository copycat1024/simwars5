use super::{App, Tag, LAUNCHER_CONTROL};
use soyo::{
    log::{enable_log, flush_log},
    tui::backend::Vt100,
    util::Result,
};
use std::io::stdout;

pub type Context = soyo::tui::Context;

pub fn launch() -> Result {
    // enable framework logger
    // enable_log(soyo::logger::Tag::Event);
    enable_log(soyo::log::Tag::Debug);

    // enable application logger
    enable_log(Tag::Launcher);

    // create context
    let vt100 = Vt100::new(stdout());
    let ctx = Context::new(vt100);

    Launcher::new(ctx).run()?;

    flush_log();

    Ok(())
}

struct Launcher {
    ctx: Context,
}

impl Launcher {
    pub fn new(ctx: Context) -> Self {
        Self { ctx }
    }

    pub fn run(mut self) -> Result {
        let mut app = App::new(LAUNCHER_CONTROL);
        app.run(&mut self.ctx)
    }
}
