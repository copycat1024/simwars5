use super::{Tag, LAUNCHER_CONTROL};
use crate::{
    app::{battle::BATTLE_CONTROL, test::TEST_CONTROL, ubmp::UBMP_CONTROL},
    mvc::App,
};
use soyo::{
    log::{enable_log, flush_log},
    tui::{backend::Vt100, Context},
    util::Result,
};
use std::io::stdout;

pub struct AppItem {
    pub name: &'static str,
    runtime: fn(ctx: &mut Context) -> Result<usize>,
}

pub const APP_LIST: [AppItem; 4] = [
    AppItem {
        name: "Launcher",
        runtime: |ctx| App::new(LAUNCHER_CONTROL).run(ctx),
    },
    AppItem {
        name: "Test app",
        runtime: |ctx| App::new(TEST_CONTROL).run(ctx),
    },
    AppItem {
        name: "Unicode plane 0",
        runtime: |ctx| App::new(UBMP_CONTROL).run(ctx),
    },
    AppItem {
        name: "Battle",
        runtime: |ctx| App::new(BATTLE_CONTROL).run(ctx),
    },
];

pub fn launch() -> Result {
    // enable framework logger
    // enable_log(soyo::logger::Tag::Event);
    enable_log(soyo::log::Tag::Debug);

    // enable application logger
    enable_log(Tag::Launcher);

    // create context
    let vt100 = Vt100::new(stdout());
    let mut ctx = Context::new(vt100);
    let mut code: usize = 0;

    loop {
        code = if code < APP_LIST.len() {
            (APP_LIST[code].runtime)(&mut ctx)?
        } else {
            break;
        }
    }

    flush_log();

    Ok(())
}
