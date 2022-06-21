use crate::mvc::{Flow, Model};

#[derive(Clone, Copy)]
pub enum TestEvent {
    Exit,
}

#[derive(Default)]
pub struct TestModel {}

impl Model for TestModel {
    type Event = TestEvent;

    fn reduce(&mut self, event: Self::Event, flow: &mut Flow) {
        match event {
            Self::Event::Exit => flow.stop = true,
        }
    }
}
