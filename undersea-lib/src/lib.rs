use chrono::{DateTime, Utc};
use reqwest::{Client, get, header::CONTENT_LENGTH};
use rss::Channel;
use std::{
    io::Cursor,
    time::{self, Duration},
};

/// A podcast, contains the URL, name and a list of [`Episode`]s.
#[derive(Debug)]
pub struct Feed {
    url: String,
    name: String,
    episodes: Vec<Episode>,
}

/// An espisode, contains the title, url, media url, and some media metadata.
#[derive(Debug)]
pub struct Episode {
    media_url: String,
    title: String,
    date: DateTime<Utc>,
    duration: Option<time::Duration>,
    resume_time: Option<time::Duration>,
    finished: bool,
}

impl Feed {
    pub async fn new(url: String) -> reqwest::Result<Self> {
        let response = get(url.clone()).await?.bytes().await?;
        let channel = Channel::read_from(&response[..]).unwrap();
        let mut episodes: Vec<Episode> = Vec::new();

        for item in channel.items() {
            let date = DateTime::parse_from_rfc2822(item.pub_date().unwrap()).unwrap();

            let media_url = if let Some(enclosure) = item.enclosure() {
                enclosure.url().to_string()
            } else {
                // if there is no enclosure on the item, we just skip it, its *probably* not a podcast episode
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
        })
    }
}

#[cfg(test)]
mod tests {
    use tokio::runtime::Runtime;

    use crate::*;

    #[test]
    fn test() {
        let rt = Runtime::new().unwrap();

        let feed = rt
            .block_on(Feed::new(
                "https://feed.podbean.com/justrollwithit/feed.xml".to_string(),
            ))
            .unwrap();

        for (i, ep) in feed.episodes.iter().enumerate() {
            println!("{}: {}", i, ep.title);
            println!("   │{}", ep.date);
            println!("   │{}", ep.media_url);
        }
    }
}
