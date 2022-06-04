pub struct Flow {
    pub stop: bool,
    pub draw: bool,
}

impl Flow {
    pub fn new() -> Self {
        Self {
            stop: false,
            draw: false,
        }
    }
}
