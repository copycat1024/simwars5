use crate::flux::{arch::State, AppControl};
use soyo::tui::{Key, Quad};
use std::time::Duration;

pub enum TestEvent {
    Exit,
}

pub struct TestState {}

impl Default for TestState {
    fn default() -> Self {
        Self {}
    }
}

impl State for TestState {
    type Event = TestEvent;

    fn reduce(&mut self, event: Self::Event) {}
}
