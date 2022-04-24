mod app;

use app::App;
use soyo::{tui::backend::Vt100, util::Result};
use std::io::stdout;

fn main() -> Result {
    let backend = Vt100::new(stdout());
    let app = App::new(backend, None);

    app.run()?;

    Ok(())
}
