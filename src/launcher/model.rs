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
}

pub enum LauncherMode {
    Test,
}

pub struct LauncherModel {
    pub app: bool,
    item: LauncherMode,
}

impl Default for LauncherModel {
    fn default() -> Self {
        Self {
            app: false,
            item: LauncherMode::Test,
        }
    }
}

impl Model for LauncherModel {
    type Event = LauncherEvent;

    fn reduce(&mut self, event: Self::Event, flow: &mut Flow) {
        match event {
            Self::Event::Exit => flow.stop = true,
            Self::Event::StartApp => self.app = true,
        }
    }
}

impl LauncherModel {
    pub fn start_app(&mut self, ctx: &mut Context) -> Result {
        self.app = false;

        writeln!(debug(), "App start");
        match self.item {
            LauncherMode::Test => {
                let mut app = App::<TestModel, TestView>::new(TEST_CONTROL);
                app.run(ctx)?;
            }
        }
        writeln!(debug(), "App end");

        Ok(())
    }
}
