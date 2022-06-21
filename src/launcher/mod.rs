mod app;
mod composer;
mod control;
mod main;
mod model;
mod tag;
mod view;

use app::App;
use composer::LauncherComposer;
use control::LAUNCHER_CONTROL;
use model::{LauncherEvent, LauncherModel};
use view::LauncherView;

pub use main::launch;
pub use tag::Tag;
