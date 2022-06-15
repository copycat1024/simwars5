use soyo::mvc::{Flow, Model};

#[derive(Clone, Copy)]
pub enum BattleEvent {
    Exit,
}

pub struct BattleModel {
    cell: u8,
}

impl Default for BattleModel {
    fn default() -> Self {
        Self { cell: 0x26 }
    }
}

impl BattleModel {
    pub fn cell(&self) -> u8 {
        self.cell
    }
}

impl Model for BattleModel {
    type Event = BattleEvent;

    fn reduce(&mut self, event: Self::Event, flow: &mut Flow) {
        match event {
            Self::Event::Exit => flow.stop = true,
        }
    }
}
