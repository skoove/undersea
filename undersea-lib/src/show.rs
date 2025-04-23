use chrono::{DateTime, Utc};
use reqwest::{IntoUrl, get};
use rss::Channel;
use std::time;

use crate::{Episode, FeedError};

/// A podcast, contains the URL, name and a list of [`Episode`]s.
#[derive(Debug)]
pub struct Show {
    pub(crate) url: String,
    pub(crate) name: String,
    pub(crate) episodes: Vec<Episode>,
    pub(crate) image: Option<rss::Image>,
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

            let description = item.content().map(|i| i.to_string());

            episodes.push(Episode {
                media_url,
                title,
                description,
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
            image: channel.image,
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

    /// Returns the image as [`rss::Image`], you will need to figure out your
    /// own way of downloading and displaying the image
    #[must_use]
    pub fn image(&self) -> Option<&rss::Image> {
        self.image.as_ref()
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

    /// Returns the episode by its index in the list.
    ///
    /// # Errors
    /// Returns [`None`] if the episode was not there.
    #[must_use]
    pub fn episode_by_index(&self, index: usize) -> Option<&Episode> {
        self.episodes.get(index)
    }
}
