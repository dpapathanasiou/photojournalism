pub mod parser;
use crate::parser::{get_photos_from_feed, NewsPhoto};

use actix_web::http::header::ContentType;
use actix_web::HttpResponse;
use actix_web::{dev::Server, web, App, HttpServer};
use std::collections::HashMap;
use std::net::TcpListener;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tokio::time;

type FeedDb = Arc<Mutex<HashMap<String, Vec<NewsPhoto>>>>;

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
            println!("could not obtain lock") // TODO: log error
        }
    }
}

async fn background(db: FeedDb, interval: u64) {
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

pub struct AppState {
    pub feeds: FeedDb,
    pub next_size: usize,
}

async fn get_next(offset: web::Path<String>, state: web::Data<AppState>) -> HttpResponse {
    let start: usize = match offset.to_string().parse() {
        Ok(i) => i,
        Err(_) => 0,
    };
    let stop = &state.clone().next_size;

    let mut body = String::new();
    let feeds = &state.clone().feeds;
    if let Ok(db) = feeds.lock() {
        let photos = db.values().flatten().collect::<Vec<_>>();
        let subset = &photos[start..=(start + stop)];
        body = subset
            .iter()
            .map(|photo| photo.as_json())
            .filter(|p| p.is_ok())
            .map(|p| p.unwrap())
            .collect::<Vec<_>>()
            .join(",");
    }

    HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(format!("[{body}]"))
}

fn run(listener: TcpListener, db: FeedDb, next_size: usize) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState {
                feeds: db.clone(),
                next_size,
            }))
            .service(
                web::scope("/api")
                    .service(web::resource("/next/{offset}").route(web::get().to(get_next))),
            )
    })
    .listen(listener)?
    .run();

    Ok(server)
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let interval = std::env::var("PHOTOJOURNALISM_FETCH_INTERVAL")
        .expect("env var 'PHOTOJOURNALISM_FETCH_INTERVAL' not defined");
    let fetch_interval: u64 = match interval.parse() {
        Ok(i) => i,
        Err(_) => 3600, // default to one hour
    };

    let feeds = Arc::new(Mutex::new(HashMap::<String, Vec<NewsPhoto>>::new()));
    let db = feeds.clone();
    tokio::spawn(async move {
        background(db, fetch_interval).await;
    });

    let address = std::env::var("PHOTOJOURNALISM_SERVER")
        .expect("env var 'PHOTOJOURNALISM_SERVER' not defined");

    let listener = TcpListener::bind(&address)
        .expect("could not bind to address defined by env var 'PHOTOJOURNALISM_SERVER'");

    let page_size = std::env::var("PHOTOJOURNALISM_PAGE_SIZE")
        .expect("env var 'PHOTOJOURNALISM_PAGE_SIZE' not defined");
    let next_size = match page_size.parse() {
        Ok(i) => i,
        Err(_) => 8, // default
    };

    run(listener, feeds, next_size)?.await
}
