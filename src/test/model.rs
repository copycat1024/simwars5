use soyo::mvc::{Flow, Model};

#[derive(Clone, Copy)]
pub enum TestEvent {
    Exit,
}

pub struct TestModel {}

impl Default for TestModel {
    fn default() -> Self {
        Self {}
    }
}

impl Model for TestModel {
    type Event = TestEvent;

    fn reduce(&mut self, event: Self::Event, flow: &mut Flow) {
        match event {
            Self::Event::Exit => flow.stop = true,
        }
    }
}
