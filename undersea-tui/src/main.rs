use anyhow::Result;

mod app;

use crate::app::App;

#[tokio::main]
async fn main() -> Result<()> {
    println!("ijsdfuiohsduif");
    let mut terminal = ratatui::init();
    let app_result = App::new().run(&mut terminal);
    ratatui::restore();
    app_result
}
