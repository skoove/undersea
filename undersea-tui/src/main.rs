use anyhow::Result;

mod app;

use crate::app::App;

pub const TESTING_URLS: [&str; 5] = [
    // just roll with it
    "https://feed.podbean.com/justrollwithit/feed.xml",
    // the magnus archives
    "https://feeds.acast.com/public/shows/b6085bcd-3542-4a43-b6a8-021e3fd251b8",
    // scp: find us alive
    "https://feeds.redcircle.com/d4548587-da15-4a7e-8b83-3c78b44ec7e5",
    // lost terminal
    "https://www.spreaker.com/show/4488937/episodes/feed",
    // red vally (this one can be annoying as the mp3 links redirect!)
    "https://feeds.megaphone.fm/redvalley",
];

#[tokio::main]
async fn main() -> Result<()> {
    let mut terminal = ratatui::init();
    let mut app = App::new().await;
    let app_result = app.run(&mut terminal);
    ratatui::restore();
    app_result
}
