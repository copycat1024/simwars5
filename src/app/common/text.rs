use std::fmt::Arguments;

pub struct Text {
    pub data: Vec<char>,
}

const FILL_CHAR: char = ' ';

impl Text {
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }

    pub fn write_fmt(&mut self, fmt: Arguments<'_>) {
        self.data = format!("{}", fmt).chars().collect();
    }
}

impl std::ops::Index<i32> for Text {
    type Output = char;

    fn index(&self, i: i32) -> &Self::Output {
        self.data.get(i as usize).unwrap_or(&FILL_CHAR)
    }
}
