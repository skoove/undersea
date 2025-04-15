use chrono::{DateTime, Utc};
use reqwest::IntoUrl;

mod episode;
mod error;
mod show;

pub use episode::Episode;
pub use error::FeedError;
pub use show::Show;

/// All of a users shows, the main point of interaction with the library
#[derive(Debug)]
pub struct Shows {
    pub(crate) shows: Vec<Show>,
    pub(crate) last_change: DateTime<Utc>,
}

impl Shows {
    /// Create a new [`Shows`] struct, this should only be done if there were none to
    /// read from file and the user explicity asked for the creation of it
    pub fn new() -> Self {
        Self {
            shows: Vec::new(),
            last_change: Utc::now(),
        }
    }

    // Add a new show from a url to the list of feeds
    pub async fn add_show<S: Into<String> + Clone + IntoUrl>(
        &mut self,
        url: S,
    ) -> Result<(), FeedError> {
        self.shows.push(Show::new(url).await?);
        Ok(())
    }

    /// Returns the name of all added shows
    pub fn names(&self) -> Vec<&str> {
        let mut titles = Vec::new();

        for show in self.shows.iter() {
            titles.push(show.name());
        }

        titles
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TESTING_URLS: [&str; 5] = [
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

    #[tokio::test]
    async fn add_single_from_url() {
        let mut shows = Shows::new();

        shows.add_show(TESTING_URLS[0]).await.unwrap();
        assert_eq!(shows.shows.len(), 1);

        shows.add_show(TESTING_URLS[1]).await.unwrap();
        assert_eq!(shows.shows.len(), 2);

        shows.add_show(TESTING_URLS[2]).await.unwrap();
        assert_eq!(shows.shows.len(), 3)
    }
}
