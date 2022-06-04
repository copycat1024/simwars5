use super::{LauncherEvent, LauncherModel, LauncherView};
use soyo::{
    mvc::Control,
    tui::{Event, Key},
};

pub const LAUNCHER_CONTROL: Control<LauncherModel, LauncherView> = Control::new(
    |event, _view, dispatch| {
        if let Event::Key { key } = event {
            if key == Key::ESC {
                dispatch.dispatch(LauncherEvent::Exit);
            } else if key == Key::ENTER {
                dispatch.dispatch(LauncherEvent::StartApp);
            }
        }
    },
    |_model, view| {
        view.write_top("Launcher");
    },
);
