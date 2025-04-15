use chrono::{DateTime, Utc};
use std::time;

/// An espisode, contains the title, url, media url, and some media metadata.
#[derive(Debug)]
pub struct Episode {
    pub(crate) media_url: String,
    pub(crate) title: String,
    pub(crate) date: DateTime<Utc>,
    pub(crate) duration: Option<time::Duration>,
    pub(crate) resume_time: time::Duration,
    pub(crate) finished: bool,
}

impl Episode {
    /// Returns the url of the episodes attached media.
    pub fn media_url(&self) -> &str {
        &self.media_url
    }

    /// Returns the title of an episode.
    pub fn title(&self) -> &str {
        &self.title
    }

    /// Returns the ['DateTime'] of the episodes upload, in UTC.
    pub fn date(&self) -> &DateTime<Utc> {
        &self.date
    }

    /// Returns the total duration of the episode as a [`Duration`]. If this is none,
    /// that means the duration has not been worked out yet, and you need to
    /// get it.
    pub fn duration(&self) -> &Option<time::Duration> {
        &self.duration
    }

    /// Returns the time where the playback should be resumed.
    pub fn resume_time(&self) -> &time::Duration {
        &self.resume_time
    }

    /// Returns weather or not the episode has been finished.
    pub fn finished(&self) -> bool {
        self.finished
    }
}
