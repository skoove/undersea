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

impl Default for Shows {
    /// Create a new [`Shows`] struct, this should only be done if there were none to
    /// read from file and the user explicity asked for the creation of it
    fn default() -> Self {
        Self {
            shows: Vec::new(),
            last_change: Utc::now(),
        }
    }
}

impl Shows {
    /// Add a new show from a url to the list of feeds
    /// # Errors
    /// Fails if the show cannot be added, most likely because of network issues
    pub async fn add<S>(&mut self, url: S) -> Result<(), FeedError>
    where
        S: IntoUrl + Clone + Into<String>,
    {
        self.shows.push(Show::new(url).await?);
        self.last_change = Utc::now();
        Ok(())
    }

    /// Add multiple shows from urls
    /// # Errors
    /// Will fail if any of the shows were unable to be added, this will not
    /// continue adding shows if one fails, but shows before the one that
    /// failed will stay added.
    pub async fn add_multiple<I>(&mut self, urls: I) -> Result<(), FeedError>
    where
        I: IntoIterator,
        I::Item: IntoUrl + Clone + Into<String>,
    {
        for url in urls {
            self.add(url).await?;
        }
        self.last_change = Utc::now();
        Ok(())
    }

    /// Returns the name of all added shows
    #[must_use]
    pub fn names(&self) -> Vec<&str> {
        let mut names = Vec::new();

        for show in &self.shows {
            names.push(show.name());
        }

        names
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
        let mut shows = Shows::default();

        shows.add(TESTING_URLS[3]).await.expect("to add shows");
        assert_eq!(shows.shows.len(), 1);
    }

    #[tokio::test]
    async fn add_multiple() {
        let mut shows = Shows::default();

        shows
            .add_multiple(TESTING_URLS)
            .await
            .expect("to add shows");

        assert_eq!(shows.shows.len(), TESTING_URLS.len());
    }
}
