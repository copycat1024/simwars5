use super::{UbmpComposer, UbmpEvent, UbmpModel};
use crate::mvc::Control;
use soyo::tui::{Event, Key};

pub const UBMP_CONTROL: Control<UbmpModel, UbmpComposer> = Control::new(
    || (UbmpModel::default(), UbmpComposer::default()),
    |event, _view, dispatch| {
        if let Event::Key { key } = event {
            if key == Key::ESC {
                dispatch.dispatch(UbmpEvent::Exit)
            } else if key == Key::LEFT {
                dispatch.dispatch(UbmpEvent::Prev);
            } else if key == Key::RIGHT {
                dispatch.dispatch(UbmpEvent::Next);
            }
        }
    },
    |model, view| {
        view.call_mut(|view| {
            view.set_cell(model.cell());
        })
    },
);
