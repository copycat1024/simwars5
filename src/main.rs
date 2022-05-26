#![feature(once_cell)]

mod app;
mod launcher;
mod tag;

fn main() {
    launcher::launch().expect("Error")
}
