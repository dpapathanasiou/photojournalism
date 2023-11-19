pub mod loader;
pub mod parser;
pub mod server;

use env_logger::Env;
use log::info;
use std::collections::HashMap;
use std::net::TcpListener;
use std::sync::{Arc, Mutex};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    let interval = std::env::var("PHOTOJOURNALISM_FETCH_INTERVAL")
        .expect("env var 'PHOTOJOURNALISM_FETCH_INTERVAL' not defined");
    let fetch_interval: u64 = match interval.parse() {
        Ok(i) => i,
        Err(_) => 3600, // default to one hour
    };

    let feeds = Arc::new(Mutex::new(HashMap::<String, Vec<parser::NewsPhoto>>::new()));

    info!("fetching rss feeds every {fetch_interval} seconds");
    let db = feeds.clone();
    tokio::spawn(async move {
        loader::background(db, fetch_interval).await;
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
    server::run(listener, feeds, next_size)?.await
}
