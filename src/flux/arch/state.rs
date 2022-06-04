pub trait State: Default + 'static {
    type Event: 'static;

    fn reduce(&mut self, event: Self::Event);
}
