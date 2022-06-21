use crate::mvc::{Flow, Model};

#[derive(Clone, Copy)]
pub enum UbmpEvent {
    Exit,
    Prev,
    Next,
}

pub struct UbmpModel {
    cell: u8,
}

impl Default for UbmpModel {
    fn default() -> Self {
        Self { cell: 0x26 }
    }
}

impl UbmpModel {
    pub fn cell(&self) -> u8 {
        self.cell
    }
}

impl Model for UbmpModel {
    type Event = UbmpEvent;

    fn reduce(&mut self, event: Self::Event, flow: &mut Flow) {
        match event {
            Self::Event::Exit => flow.stop = true,
            Self::Event::Prev => {
                if self.cell > 0 {
                    self.cell -= 1;
                    flow.clear = true;
                    flow.draw = true;
                }
            }
            Self::Event::Next => {
                if self.cell < 255 {
                    self.cell += 1;
                    flow.clear = true;
                    flow.draw = true;
                }
            }
        }
    }
}
