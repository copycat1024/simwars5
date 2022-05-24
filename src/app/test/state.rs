pub struct TestState {
    pub w: i32,
    pub h: i32,
}

impl TestState {
    pub fn new() -> Self {
        Self { w: 0, h: 0 }
    }
}
