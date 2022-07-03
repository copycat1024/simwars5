use crate::launcher::APP_LIST;
use crate::mvc::{Flow, Model};

#[derive(Clone, Copy)]
pub enum LauncherEvent {
    Exit,
    StartApp,
    MenuNext,
    MenuPrev,
}

#[derive(Default)]
pub struct LauncherModel {
    id: usize,
}

impl Model for LauncherModel {
    type Event = LauncherEvent;

    fn reduce(&mut self, event: Self::Event, flow: &mut Flow) {
        match event {
            Self::Event::Exit => {
                flow.exit(usize::MAX);
            }
            Self::Event::StartApp => {
                flow.exit(self.id + 1);
            }
            Self::Event::MenuNext => {
                if self.id < APP_LIST.len() - 2 {
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
        APP_LIST[1..].iter().map(|app| app.name).collect()
    }

    pub fn item(&self) -> usize {
        self.id
    }
}
