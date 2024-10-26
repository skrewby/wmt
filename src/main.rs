mod app;
mod hypr;
mod screen;

use anyhow::Result;
use app::App;

fn main() -> Result<()> {
    let mut terminal = ratatui::init();
    let mut app = App::new()?;
    let app_result = app.run(&mut terminal);
    ratatui::restore();
    app_result
}
