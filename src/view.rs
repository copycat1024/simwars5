struct Label {}

struct View {}

impl View {
    pub fn new() -> Self {
        let element = Box::new(element);

        Self { element, updater }
    }

    pub fn update(&mut self, state: &S) {
        use std::borrow::BorrowMut;
        let Self { element, updater } = self;
        updater(element.borrow_mut(), state);
    }
}
