pub trait View: Default + 'static {
    fn setup(&mut self);
    fn resize(&mut self, w: i32, h: i32);
}
