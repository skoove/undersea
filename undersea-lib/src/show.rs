use chrono::{DateTime, Utc};
use reqwest::get;
use rss::Channel;

use crate::{Episode, FeedError};

/// A podcast, contains the URL, name and a list of [`Episode`]s.
#[derive(Debug)]
pub struct Show {
    pub(crate) url: String,
    pub(crate) name: String,
    pub(crate) episodes: Vec<Episode>,
    pub(crate) last_checked: DateTime<Utc>,
    pub(crate) last_upload: DateTime<Utc>,
}

impl Show {
    pub(crate) async fn new<S: Into<String> + reqwest::IntoUrl + Clone>(
        url: S,
    ) -> Result<Show, FeedError> {
        let response = get(url.clone()).await?.bytes().await?;
        let channel = Channel::read_from(&response[..]).unwrap();
        let mut episodes: Vec<Episode> = Vec::new();

        for item in channel.items() {
            let date = DateTime::parse_from_rfc2822(item.pub_date().unwrap()).unwrap();

            let media_url = if let Some(enclosure) = item.enclosure() {
                enclosure.url().to_string()
            } else {
                // skip ep if no attached file
                // TODO: handle this a bit better
                continue;
            };

            let url_copy = url.clone();
            let title = if let Some(episode_title) = item.title() {
                episode_title.to_string()
            } else {
                url_copy.into()
            };

            episodes.push(Episode {
                media_url,
                title,
                date: date.into(),
                // TODO: Implement this
                duration: None,
                resume_time: None,
                finished: false,
            });
        }

        Ok(Self {
            url: url.into(),
            name: channel.title,
            episodes,
            last_checked: Utc::now(),
            last_upload: Utc::now(),
        })
    }

    /// Returns the title of a show
    pub fn name(&self) -> &str {
        &self.name
    }
}
