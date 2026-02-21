use crate::parser::{NewsPhoto, get_photos_from_feed};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tokio::time;

pub type FeedDb = Arc<Mutex<HashMap<String, Vec<NewsPhoto>>>>;

async fn fetch(feeds: Vec<String>, db: FeedDb) {
    for feed in feeds {
        let photos = get_photos_from_feed(&feed).await;
        match db.lock() {
            Ok(mut hash) => {
                hash.insert(feed.to_string(), photos.clone());
            }
            _ => {
                log::error!("rss fetch: could not obtain FeedDb lock")
            }
        }
    }
}

pub async fn background(feeds: Vec<String>, db: FeedDb, interval: u64) {
    // load the FeedDb in the background, once at the given internal (in seconds)
    let mut interval = time::interval(Duration::from_secs(interval));

    loop {
        let db = db.clone();
        let feeds = feeds.clone();
        tokio::spawn(async move {
            fetch(feeds, db).await;
        });

        interval.tick().await;
    }
}
