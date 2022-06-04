use super::{TestModel, TestView};
use crate::mvc::arch::{Control, Model};
use soyo::tui::{Event, Key};

pub const TEST_CONTROL: Control<TestModel, TestView> = Control::new(
    |event, view, dispatch| {
        if let Event::Key { key } = event {
            if key == Key::ESC {
                dispatch.dispatch(<crate::mvc::test::model::TestModel as Model>::Event::Exit)
            }
        }
    },
    |model, view| {
        view.write_top("Welcome");
    },
);
