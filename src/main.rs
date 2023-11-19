pub mod loader;
pub mod parser;
pub mod server;

use env_logger::Env;
use log::info;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::net::TcpListener;
use std::path::Path;
use std::sync::{Arc, Mutex};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    let feed_list = std::env::var("PHOTOJOURNALISM_FEED_LIST")
        .expect("env var 'PHOTOJOURNALISM_FEED_LIST' not defined");
    let feed_path = Path::new(&feed_list);

    let feeds = match File::open(&feed_path) {
        Ok(file) => {
            let file = BufReader::new(file);
            file.lines()
                .filter(|line| line.is_ok())
                .map(|line| line.unwrap())
                .filter(|text| !text.starts_with("#"))
                .collect::<Vec<_>>()
        }
        Err(_) => panic!("cannot find list of RSS feeds at '{feed_list}'"),
    };

    let interval = std::env::var("PHOTOJOURNALISM_FETCH_INTERVAL")
        .expect("env var 'PHOTOJOURNALISM_FETCH_INTERVAL' not defined");
    let fetch_interval: u64 = match interval.parse() {
        Ok(i) => i,
        Err(_) => 3600, // default to one hour
    };

    let feed_db = Arc::new(Mutex::new(HashMap::<String, Vec<parser::NewsPhoto>>::new()));

    info!("fetching rss feeds every {fetch_interval} seconds");
    let db = feed_db.clone();
    tokio::spawn(async move {
        loader::background(feeds, db, fetch_interval).await;
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

    info!("web service running on {address}");
    server::run(listener, feed_db, next_size)?.await
}
