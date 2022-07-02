use super::{TestComposer, TestModel};
use crate::mvc::{Control, Model};
use soyo::tui::{Event, Key};

pub const TEST_CONTROL: Control<TestModel, TestComposer> = Control::new(
    |event, _view, dispatch| {
        if let Event::Key { key } = event {
            if key == Key::ESC {
                dispatch.dispatch(<TestModel as Model>::Event::Exit)
            }
        }
    },
    |_model, _view| {},
);
