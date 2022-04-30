const CUSTOM: isize = soyo::logger::Tag::Custom as isize;

pub enum Tag {
    Launcher = CUSTOM,
}

impl From<Tag> for u8 {
    fn from(tag: Tag) -> u8 {
        tag as u8
    }
}
