use crate::test::{TestModel, TestView, TEST_CONTROL};
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

const APP_LEN: usize = 2;
const APP_LIST: [&str; APP_LEN] = ["Test app", "Unicode plane 0"];

pub struct LauncherModel {
    app: Option<usize>,
    id: usize,
}

impl Default for LauncherModel {
    fn default() -> Self {
        Self { app: None, id: 0 }
    }
}

impl Model for LauncherModel {
    type Event = LauncherEvent;

    fn reduce(&mut self, event: Self::Event, flow: &mut Flow) {
        match event {
            Self::Event::Exit => flow.stop = true,
            Self::Event::StartApp => self.app = Some(self.id),
            Self::Event::MenuNext => {
                if self.id < APP_LEN - 1 {
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
    pub fn app_list(&self) -> std::array::IntoIter<&str, APP_LEN> {
        APP_LIST.into_iter()
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
        match id {
            0 => App::<TestModel, TestView>::new(TEST_CONTROL).run(ctx),
            _ => Ok(()),
        }
    }
}
