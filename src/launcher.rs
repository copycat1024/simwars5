use crate::{
    app::{App, TestApp},
    tag::Tag,
};
use soyo::{
    logger::{Client, Server},
    tui::{backend::Vt100, Event, Key, Rect},
    util::Result,
};
use std::io::{stdout, Write};

type Backend = soyo::tui::backend::Vt100<std::io::Stdout>;
type Context = soyo::tui::Context<Backend>;

pub fn launch() -> Result {
    let log = get_logger();

    Launcher::new(&log).run()?;

    log.print_raw();

    Ok(())
}

struct Launcher {
    ctx: Context,
    log: Client,
}

impl Launcher {
    pub fn new(log: &Server) -> Self {
        Self {
            ctx: get_context(log),
            log: log.client(Tag::Launcher),
        }
    }

    pub fn run(mut self) -> Result {
        self.ctx.clear()?;

        loop {
            if let Some(e) = self.ctx.event()? {
                match e {
                    Event::Key { key } => {
                        if key == Key::ESC {
                            return Ok(());
                        } else {
                            self.on_key(key)?;
                        }
                    }
                    _ => {}
                }

                self.render();
                self.ctx.draw()?;
            }
        }
    }

    fn on_key(&mut self, key: Key) -> Result {
        if key == Key::ENTER {
            self.start_app()
        } else {
            Ok(())
        }
    }

    fn render(&mut self) {
        let rect = Rect::xywh(0, 0, 16, 16);
        self.ctx.render(rect, 2, |_x, _y, letter| {
            *letter.c = 'x';
        });
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
    log.enable(soyo::logger::Tag::Event);

    // enable application log
    log.enable(Tag::Launcher);

    log
}
