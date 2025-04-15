use chrono::{DateTime, Utc};
use reqwest::get;
use rss::Channel;

use crate::{Episode, FeedError};

/// A podcast, contains the URL, name and a list of [`Episode`]s.
#[derive(Debug)]
pub struct Feed {
    pub(crate) url: String,
    pub(crate) name: String,
    pub(crate) episodes: Vec<Episode>,
    pub(crate) last_checked: DateTime<Utc>,
    pub(crate) last_upload: DateTime<Utc>,
}

impl Feed {
    pub(crate) async fn new(url: String) -> Result<Feed, FeedError> {
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

            let title = if let Some(episode_title) = item.title() {
                episode_title.to_string()
            } else {
                url.clone()
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
            url,
            name: channel.title,
            episodes,
            last_checked: Utc::now(),
            last_upload: Utc::now(),
        })
    }
}
