use crate::parser::{get_photos_from_feed, NewsPhoto};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tokio::time;

pub type FeedDb = Arc<Mutex<HashMap<String, Vec<NewsPhoto>>>>;

async fn fetch(db: FeedDb) {
    let feeds = vec![
        "https://rss.nytimes.com/services/xml/rss/nyt/HomePage.xml",
        "https://www.politico.com/rss/politicopicks.xml",
        "https://www.wired.com/feed/rss",
        "https://www.france24.com/en/rss",
        "https://www.japantimes.co.jp/feed/",
        "https://aeon.co/feed.rss",
        "https://api.quantamagazine.org/feed/",
    ];

    for feed in feeds {
        let photos = get_photos_from_feed(feed).await;
        if let Ok(mut hash) = db.lock() {
            hash.insert(feed.to_string(), photos.clone());
        } else {
            log::error!("rss fetch: could not obtain FeedDb lock")
        }
    }
}

pub async fn background(db: FeedDb, interval: u64) {
    // load the FeedDb in the background, once at the given internal (in seconds)
    let mut interval = time::interval(Duration::from_secs(interval));

    loop {
        let db = db.clone();
        tokio::spawn(async move {
            fetch(db).await;
        });

        interval.tick().await;
    }
}
