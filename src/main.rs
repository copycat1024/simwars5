#![feature(once_cell)]

mod app;
mod launcher;

fn main() {
    launcher::launch().expect("Error")
}
