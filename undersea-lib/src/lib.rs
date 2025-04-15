use chrono::{DateTime, Utc};

mod episode;
mod error;
mod feed;

pub use episode::Episode;
pub use error::FeedError;
pub use feed::Feed;

/// All of a users shows, the main point of interaction with the library
#[derive(Debug)]
pub struct Shows {
    feed: Vec<Feed>,
    last_change: DateTime<Utc>,
}

impl Shows {
    /// Create a new [`Shows`] struct, this should only be done if there were none to
    /// read from file and the user explicity asked for the creation of it
    pub fn new() -> Self {
        Self {
            feed: Vec::new(),
            last_change: Utc::now(),
        }
    }

    // Add a new show from a url to the list of feeds
    pub async fn add_show(&mut self, url: String) -> Result<(), FeedError> {
        self.feed.push(Feed::new(url).await?);
        Ok(())
    }
}
