#![feature(const_mut_refs)]

mod mvc;
mod launcher;

fn main() {
    launcher::launch().expect("Error")
}
