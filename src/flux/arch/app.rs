use crate::flux::AppControl;
use soyo::{
    tui::{Context, Event, Key},
    util::Result,
};
use std::time::Duration;

// pub trait App: Sized {
//     fn on_start(&mut self, _ctrl: &mut AppControl) {}
//     fn on_end(&mut self, _ctrl: &mut AppControl) {}
//     fn on_resize(&mut self, _ctrl: &mut AppControl, _w: i32, _h: i32) {}
//     fn on_key(&mut self, _ctrl: &mut AppControl, _key: Key) {}
//     fn on_tick(&mut self, _ctrl: &mut AppControl, _delta: Duration) {}

//     fn update(&mut self, _ctrl: &mut AppControl) {}
//     fn render(&self, ctx: &mut Context);

//     fn run(&mut self, ctx: &mut Context) -> Result {
//         // init app
//         let mut ctrl = AppControl::new();
//         let (w, h) = ctx.size();

//         self.on_start(&mut ctrl);
//         ctrl.draw = true;
//         self.on_resize(&mut ctrl, w, h);
//         ctx.clear()?;

//         // main loop
//         loop {
//             if let Some(e) = ctx.event()? {
//                 match e {
//                     Event::Key { key } => self.on_key(&mut ctrl, key),
//                     Event::Resize { w, h } => {
//                         ctrl.draw = true;
//                         ctx.clear()?;
//                         self.on_resize(&mut ctrl, w, h)
//                     }
//                     Event::Update { delta } => self.on_tick(&mut ctrl, delta),
//                 }

//                 self.update(&mut ctrl);

//                 if ctrl.draw {
//                     self.render(ctx);
//                     ctx.draw()?;
//                     ctrl.draw = false;
//                 }
//             }

//             if ctrl.stop {
//                 break;
//             }
//         }

//         // clean up app
//         self.on_end(&mut ctrl);
//         ctx.clear()
//     }
// }
