use super::{TestModel, TestView};
use soyo::{
    mvc::{Control, Model},
    tui::{Event, Key},
};

pub const TEST_CONTROL: Control<TestModel, TestView> = Control::new(
    |event, _view, dispatch| {
        if let Event::Key { key } = event {
            if key == Key::ESC {
                dispatch.dispatch(<crate::test::model::TestModel as Model>::Event::Exit)
            }
        }
    },
    |_model, view| {
        view.write_top("Welcome");
    },
);
