use crate::test::TEST_CONTROL;
use soyo::{
    log::debug,
    mvc::{App, Flow, Model},
    tui::Context,
    util::Result,
};

#[derive(Clone, Copy)]
pub enum LauncherEvent {
    Exit,
    StartApp,
    MenuNext,
    MenuPrev,
}

struct AppItem {
    name: &'static str,
    runtime: fn(ctx: &mut Context) -> Result,
}

const APP_LIST: [AppItem; 2] = [
    AppItem {
        name: "Test app",
        runtime: |ctx| App::new(TEST_CONTROL).run(ctx),
    },
    AppItem {
        name: "Unicode plane 0",
        runtime: |_| Ok(()),
    },
];

#[derive(Default)]
pub struct LauncherModel {
    app: Option<usize>,
    id: usize,
}

impl Model for LauncherModel {
    type Event = LauncherEvent;

    fn reduce(&mut self, event: Self::Event, flow: &mut Flow) {
        match event {
            Self::Event::Exit => flow.stop = true,
            Self::Event::StartApp => self.app = Some(self.id),
            Self::Event::MenuNext => {
                if self.id < APP_LIST.len() - 1 {
                    self.id += 1;
                    flow.draw = true;
                }
            }
            Self::Event::MenuPrev => {
                if self.id > 0 {
                    self.id -= 1;
                    flow.draw = true;
                }
            }
        }
    }
}

impl LauncherModel {
    pub fn app_list(&self) -> Vec<&str> {
        APP_LIST.into_iter().map(|app| app.name).collect()
    }

    pub fn item(&self) -> usize {
        self.id
    }
}

impl LauncherModel {
    pub fn start_app(&mut self, ctx: &mut Context, flow: &mut Flow) -> Result {
        if let Some(id) = self.app.take() {
            writeln!(debug(), "App start");
            self.run_app(ctx, id)?;
            writeln!(debug(), "App end");
            flow.draw = true;
        }

        Ok(())
    }

    fn run_app(&mut self, ctx: &mut Context, id: usize) -> Result {
        if id < APP_LIST.len() {
            (APP_LIST[id].runtime)(ctx)
        } else {
            unreachable!()
        }
    }
}
