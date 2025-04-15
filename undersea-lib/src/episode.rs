use chrono::{DateTime, Utc};
use std::time;

/// An espisode, contains the title, url, media url, and some media metadata.
#[derive(Debug)]
pub struct Episode {
    pub(crate) media_url: String,
    pub(crate) title: String,
    pub(crate) date: DateTime<Utc>,
    pub(crate) duration: Option<time::Duration>,
    pub(crate) resume_time: Option<time::Duration>,
    pub(crate) finished: bool,
}
