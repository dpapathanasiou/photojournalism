pub mod parser;
use crate::parser::{get_photos, load_feed};

#[tokio::main]
async fn main() -> () {
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
        match load_feed(feed).await {
            Ok(channel) => println!("{feed}\n{:#?}", get_photos(channel)),
            Err(_) => println!("sorry"),
        }
    }
}
