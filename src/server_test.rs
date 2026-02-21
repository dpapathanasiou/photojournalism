use super::*;
use crate::parser::NewsPhoto;
use actix_web::{App, body::to_bytes, test, web};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

fn set_app_state() -> AppState {
    let mut parsed_feeds = HashMap::<String, Vec<NewsPhoto>>::new();

    parsed_feeds.insert(
        "https://rss.nytimes.com/services/xml/rss/nyt/HomePage.xml".to_string(),
        vec![
            NewsPhoto{
                image_url: "https://static01.nyt.com/images/2023/11/23/multimedia/23finland-border-kmbp/23finland-border-kmbp-mediumSquareAt3X.jpg".to_string(), 
                story_url: "https://www.nytimes.com/2023/11/23/world/europe/finland-russia-border-migrants.html".to_string(), 
                description: Some("Finnish border guards escorting migrants at the international crossing with Russia near Salla, Finland, on Thursday.".to_string()),
                credit: Some("Jussi Nukari/Lehtikuva, via Associated Press".to_string())
            },
            NewsPhoto {
                image_url: "https://static01.nyt.com/images/2023/11/23/multimedia/23themorning-lead-promo/23themorning-lead-bmhq-mediumSquareAt3X.jpg".to_string(),
                story_url: "https://www.nytimes.com/2023/11/23/briefing/thanksgiving-pep-talk.html".to_string(),
                description: Some("A Thanksgiving Pep Talk".to_string()),
                credit: Some("Johnny Miller for The New York Times".to_string())
            },
        ],
    );
    parsed_feeds.insert(
        "https://www.france24.com/en/rss".to_string(),
        vec![
            NewsPhoto {
                image_url: "https://s.france24.com/media/display/98336912-8a11-11ee-9a7e-005056bf30b7/w:1024/p:16x9/ENBT%20BIL%20SILICON%20VALLEY%20PUSH%20PICTURE.jpg".to_string(),
                story_url: "https://www.france24.com/en/tv-shows/revisited/20231124-bouncing-back-silicon-valley-bets-on-ai-to-regain-past-glory".to_string(),
                description: Some("Bouncing back: Silicon Valley bets on AI to regain past glory".to_string()),
                credit: Some("Pierrick LEURENT".to_string())
            },
        ],
    );

    let feed_db = Arc::new(Mutex::new(parsed_feeds));
    AppState {
        feeds: feed_db,
        next_size: 3,
    }
}

#[actix_web::test]
async fn endpoints_test() {
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(set_app_state()))
            .service(web::resource("/health").route(web::get().to(health)))
            .service(web::resource("/api/next/{offset}").route(web::get().to(get_next))),
    )
    .await;

    let health_request = test::TestRequest::get().uri("/health").to_request();
    let health_response = test::call_service(&app, health_request).await;
    assert!(health_response.status().is_success());
    let health_response_body = to_bytes(health_response.into_body()).await.unwrap();
    assert_eq!(
        health_response_body.to_owned(),
        r###"{"feeds":2,"photos":3}"###
    );

    // retrieve the three photos in shared memory
    let fetch_request = test::TestRequest::get().uri("/api/next/0").to_request();
    let fetch_response = test::call_service(&app, fetch_request).await;
    assert!(fetch_response.status().is_success());
    let fetch_response_body = to_bytes(fetch_response.into_body()).await.unwrap();
    assert_eq!(fetch_response_body.len(), 1126);

    // attempt to fetch beyond what is available: should result in an empty list
    let excess_fetch_request = test::TestRequest::get().uri("/api/next/4").to_request();
    let excess_fetch_response = test::call_service(&app, excess_fetch_request).await;
    assert!(excess_fetch_response.status().is_success());
    let excess_fetch_response_body = to_bytes(excess_fetch_response.into_body()).await.unwrap();
    assert_eq!(excess_fetch_response_body.len(), 2)
}
