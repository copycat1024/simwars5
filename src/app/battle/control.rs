use super::{BattleComposer, BattleEvent, BattleModel};
use crate::mvc::Control;
use soyo::tui::{Event, Key};

pub const BATTLE_CONTROL: Control<BattleModel, BattleComposer> = Control::new(
    || (BattleModel::default(), BattleComposer::default()),
    |event, _view, dispatch| {
        if let Event::Key { key } = event {
            if key == Key::ESC {
                dispatch.dispatch(BattleEvent::Exit)
            }
        }
    },
    |_, _| {},
);
