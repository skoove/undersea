use chrono::{DateTime, Utc};
use core::time;
use reqwest::{IntoUrl, get};
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
    pub(crate) async fn new<S>(url: S) -> Result<Show, FeedError>
    where
        S: IntoUrl + Clone + Into<String>,
    {
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
                resume_time: time::Duration::from_secs(0),
                finished: false,
            });
        }

        episodes.sort_by_key(|ep| ep.date);

        Ok(Self {
            url: url.into(),
            name: channel.title,
            episodes,
            last_checked: Utc::now(),
            last_upload: Utc::now(),
        })
    }

    /// Time of the last time the feed was checked for new episodes and other changes.
    #[must_use]
    pub fn last_check(&self) -> &DateTime<Utc> {
        &self.last_checked
    }

    /// Returns the date of the last new episode uploaded to a shows feed. This does not
    /// request the new date, to do that you need to update the feed manually.
    #[must_use]
    pub fn last_upload(&self) -> &DateTime<Utc> {
        &self.last_upload
    }

    /// Return the url of a show
    #[must_use]
    pub fn url(&self) -> &str {
        &self.url
    }

    /// Returns the title of a show
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns referances to all episodes added
    #[must_use]
    pub fn episodes(&self) -> Vec<&Episode> {
        let mut episode_refs = Vec::new();
        for episode in &self.episodes {
            episode_refs.push(episode);
        }
        episode_refs
    }

    /// Returns the title of all the episodes of a show
    #[must_use]
    pub fn episode_titles(&self) -> Vec<String> {
        let mut titles = Vec::new();

        for episode in self.episodes() {
            titles.push(episode.title().to_string());
        }

        titles
    }
}
