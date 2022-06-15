mod app;
mod launcher;
mod usym;
mod widget;

fn main() {
    launcher::launch().expect("Error")
}
