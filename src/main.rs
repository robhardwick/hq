mod app;
mod widgets;

use anyhow::Result;

use app::App;

fn main() -> Result<()> {
    let mut terminal = ratatui::init();
    let app_result = App::new()?.run(&mut terminal);
    ratatui::restore();
    app_result
}
