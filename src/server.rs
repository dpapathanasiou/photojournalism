use crate::loader::FeedDb;
use crate::shuffler::randomize;
use actix_files::Files;
use actix_web::http::header::ContentType;
use actix_web::middleware::Logger;
use actix_web::HttpResponse;
use actix_web::{dev::Server, web, App, HttpServer};
use serde::{Deserialize, Serialize};
use std::net::TcpListener;

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
        let total = photos.len();
        if start < total {
            let mut subset = Vec::new();
            let shuffle = randomize(1, total);
            for i in &shuffle[start..std::cmp::min(start + stop, total)] {
                match photos.get(*i) {
                    Some(photo) => subset.push(photo),
                    None => log::error!("missing photo at index {i}"),
                }
            }
            body = subset
                .iter()
                .map(|photo| photo.as_json())
                .filter(|p| p.is_ok())
                .map(|p| p.unwrap())
                .collect::<Vec<_>>()
                .join(",");
        }
    }

    HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(format!("[{body}]"))
}

#[derive(Debug, Serialize, Deserialize)]
struct Status {
    feeds: usize,
    photos: usize,
}

async fn health(state: web::Data<AppState>) -> HttpResponse {
    let mut feed_count: usize = 0;
    let mut photo_count: usize = 0;
    let feeds = &state.clone().feeds;
    if let Ok(db) = feeds.lock() {
        feed_count = db.keys().len();
        photo_count = db.values().flatten().collect::<Vec<_>>().len();
    }
    let status = Status {
        feeds: feed_count,
        photos: photo_count,
    };
    let result = match serde_json::to_string(&status) {
        Ok(s) => s,
        Err(_) => format!("{:#?}", status),
    };

    HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(result)
}

pub fn run(listener: TcpListener, db: FeedDb, next_size: usize) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState {
                feeds: db.clone(),
                next_size,
            }))
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .route("/health", web::get().to(health))
            .service(
                web::scope("/api")
                    .service(web::resource("/next/{offset}").route(web::get().to(get_next))),
            )
            .service(Files::new("/js", String::from("./static/js")).index_file("loader.js"))
            .service(Files::new("/", String::from("./static/")).index_file("index.html"))
    })
    .listen(listener)?
    .run();

    Ok(server)
}
