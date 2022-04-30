mod app;
mod launcher;
mod tag;

fn main() {
    launcher::launch().expect("Error")
}
