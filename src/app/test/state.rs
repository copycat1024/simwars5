use soyo::tui::Quad;

pub struct TestState {
    pub pos: Quad,
}

impl TestState {
    pub fn new() -> Self {
        Self {
            pos: Quad::xywh(0, 0, 0, 0),
        }
    }
}
