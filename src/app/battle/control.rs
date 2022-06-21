use super::{BattleEvent, BattleModel, BattleView};
use crate::mvc::Control;
use soyo::tui::{Event, Key};

pub const BATTLE_CONTROL: Control<BattleModel, BattleView> = Control::new(
    |event, _view, dispatch| {
        if let Event::Key { key } = event {
            if key == Key::ESC {
                dispatch.dispatch(BattleEvent::Exit)
            }
        }
    },
    |model, view| {
        view.set_cell(model.cell());
    },
);
