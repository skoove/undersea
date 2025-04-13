use chrono::{DateTime, Utc};
use reqwest::get;
use rss::Channel;
use std::time::{self, Duration};

#[derive(Debug)]
pub struct Feed {
    url: String,
    name: String,
    episodes: Vec<Episode>,
}

#[derive(Debug)]
pub struct Episode {
    url: String,
    media_url: String,
    title: String,
    date: DateTime<Utc>,
    duration: time::Duration,
    resume_time: time::Duration,
}

impl Feed {
    pub async fn new(url: String) -> reqwest::Result<Self> {
        let response = get(url.clone()).await?.bytes().await?;
        let channel = Channel::read_from(&response[..]).unwrap();
        let mut episodes: Vec<Episode> = Vec::new();

        for item in channel.items() {
            let instant_date = DateTime::parse_from_rfc2822(item.pub_date().unwrap()).unwrap();

            episodes.push(Episode {
                url: url.clone(),
                media_url: item.enclosure().unwrap().url().to_string(),
                title: item.title().unwrap().to_string(),
                date: instant_date.into(),
                // TODO: Implement this
                duration: Duration::from_secs(0),
                resume_time: Duration::from_secs(0),
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

    use crate::Feed;

    #[test]
    fn test() {
        let rt = Runtime::new().unwrap();

        let feed = rt
            .block_on(Feed::new(
                "https://feed.podbean.com/justrollwithit/feed.xml".to_string(),
            ))
            .unwrap();

        println!("{:#?}", feed);
    }
}
