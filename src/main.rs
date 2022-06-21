#![feature(coerce_unsized)]
#![feature(unsize)]
#![feature(const_mut_refs)]

mod app;
mod launcher;
mod mvc;
mod usym;
mod view;
mod widget;

use std::panic::{set_hook, PanicInfo};

fn main() {
    set_hook(Box::new(panic_hook));
    launcher::launch().expect("Error");
}

fn panic_hook(info: &PanicInfo<'_>) {
    use backtrace::Backtrace;
    use crossterm::{
        cursor::Show,
        style::ResetColor,
        terminal::{disable_raw_mode, EnableLineWrap, LeaveAlternateScreen},
        ExecutableCommand,
    };

    disable_raw_mode().unwrap();
    std::io::stdout()
        .execute(LeaveAlternateScreen)
        .unwrap()
        .execute(ResetColor)
        .unwrap()
        .execute(EnableLineWrap)
        .unwrap()
        .execute(Show)
        .unwrap();

    if let Some(s) = info.payload().downcast_ref::<&str>() {
        println!("Panic: {s:?}");
    } else {
        println!("Panic unknown");
    }
    let bt = Backtrace::new();
    println!("{:?}", bt);
}
