use super::{TestModel, TestView};
use crate::mvc::{Control, Model};
use soyo::tui::{Event, Key};

pub const TEST_CONTROL: Control<TestModel, TestView> = Control::new(
    |event, _view, dispatch| {
        if let Event::Key { key } = event {
            if key == Key::ESC {
                dispatch.dispatch(<TestModel as Model>::Event::Exit)
            }
        }
    },
    |_model, _view| {},
);
