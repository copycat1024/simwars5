use super::Frame;

pub struct Attribute {
    pub frame: Frame,
}

impl Attribute {
    pub fn new() -> Self {
        Self {
            frame: Frame::screen(0, 0),
        }
    }

    pub fn resize(&mut self, w: i32, h: i32) {
        self.frame = self.frame.set_w(w).set_h(h);
    }
}
