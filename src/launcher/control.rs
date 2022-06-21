use super::{LauncherEvent, LauncherModel, LauncherView};
use crate::mvc::Control;
use soyo::tui::{Event, Key};

pub const LAUNCHER_CONTROL: Control<LauncherModel, LauncherView> = Control::new(
    |event, _view, dispatch| {
        if let Event::Key { key } = event {
            if key == Key::ESC {
                dispatch.dispatch(LauncherEvent::Exit);
            } else if key == Key::ENTER {
                dispatch.dispatch(LauncherEvent::StartApp);
            } else if key == Key::UP {
                dispatch.dispatch(LauncherEvent::MenuPrev);
            } else if key == Key::DOWN {
                dispatch.dispatch(LauncherEvent::MenuNext);
            }
        }
    },
    |model, view| {
        view.set_menu(model.app_list());
        view.set_item(model.item());

        view.write_top("Launcher");
    },
);
