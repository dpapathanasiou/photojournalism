use http_cache_reqwest::{CACacheManager, Cache, CacheMode, HttpCache, HttpCacheOptions};
use reqwest::header::USER_AGENT;
use reqwest::Client;
use reqwest_middleware::ClientBuilder;
use rss::Channel;
use scraper::{Html, Selector};
use std::error::Error;

#[derive(Debug, Clone)]
pub struct NewsPhoto {
    pub image_url: String,
    pub story_url: String,
    pub description: Option<String>,
    pub credit: Option<String>,
}

impl NewsPhoto {
    const EMPTY: String = String::new();
    pub fn new() -> Self {
        Self {
            image_url: Self::EMPTY,
            story_url: Self::EMPTY,
            description: None,
            credit: None,
        }
    }

    pub fn valid(&self) -> bool {
        self.image_url != Self::EMPTY && self.story_url != Self::EMPTY
    }

    pub fn link_text(&self) -> String {
        match (self.description.is_none(), self.credit.is_none()) {
            (true, true) => self.story_url.clone(),
            (true, false) => self.credit.clone().unwrap(),
            (false, true) => self.description.clone().unwrap(),
            (false, false) => format!(
                "{} ({})",
                self.description.clone().unwrap(),
                self.credit.clone().unwrap()
            ),
        }
    }
}

fn user_agent() -> String {
    format!(
        "{}/{} +http://github.com/dpapathanasiou/photojournalism",
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION")
    )
}

pub async fn load_feed(url: &str) -> std::result::Result<Channel, Box<dyn Error>> {
    let client = ClientBuilder::new(Client::new())
        .with(Cache(HttpCache {
            mode: CacheMode::Default,
            manager: CACacheManager::default(),
            options: HttpCacheOptions::default(),
        }))
        .build();
    let content = client
        .get(url)
        .header(USER_AGENT, user_agent())
        .send()
        .await?
        .bytes()
        .await?;

    let channel = Channel::read_from(&content[..])?;
    Ok(channel)
}

pub fn get_photos(c: Channel) -> Vec<NewsPhoto> {
    let mut results = Vec::new();
    for item in c.items() {
        let mut photo = NewsPhoto::new();

        /*
          Atom and RSS 2.0 differences aside, "valid" feeds are
          wildly inconsistent with where they put their image data.

          The strategy here is to look in the correct places last,
          defaulting data from other fields first, which can be
          overwritten by the expected definitions, if they exist.
        */

        if item.link().is_some() {
            photo.story_url = item.link().clone().unwrap().to_string()
        }

        if item.title().is_some() {
            photo.description = Some(item.title().clone().unwrap().to_string())
        }

        if let Some(desc) = item.description() {
            let fragment = Html::parse_fragment(desc);
            match Selector::parse(r#"img"#) {
                Ok(selector) => {
                    for elem in fragment.select(&selector) {
                        if elem.value().attr("src").is_some() {
                            photo.image_url = elem.value().attr("src").clone().unwrap().to_string()
                        }
                        if let Some(alt_text) = elem.value().attr("alt") {
                            if alt_text.len() > 0 {
                                photo.description = Some(alt_text.to_string())
                            }
                        }
                    }
                }
                Err(_) => photo.description = Some(desc.to_string()),
            }
        }

        if let Some(enc) = item.enclosure() {
            if enc.mime_type().starts_with("image/") {
                photo.image_url = enc.url().to_string()
            }
        }

        if let Some(src) = item.source() {
            if src.title().is_some() {
                photo.credit = Some(src.title().clone().unwrap().to_string())
            }
        }

        if let Some(dc) = item.dublin_core_ext() {
            photo.credit = Some(dc.creators().join(", "))
        }

        for (extension_key, extension_map) in item.extensions() {
            if extension_key == "atom" {
                if extension_map.contains_key("link") {
                    for medium in extension_map.get("link").unwrap() {
                        if medium.name() == "atom:link" {
                            for (key, val) in medium.attrs() {
                                if key == "href" {
                                    photo.story_url = val.to_string()
                                }
                            }
                        }
                    }
                }
            }

            if extension_key == "media" {
                if extension_map.contains_key("thumbnail") {
                    for medium in extension_map.get("thumbnail").unwrap() {
                        if medium.name() == "media:thumbnail" {
                            for (key, val) in medium.attrs() {
                                if key == "url" {
                                    photo.image_url = val.to_string()
                                }
                            }
                        }
                    }
                }

                if extension_map.contains_key("content") {
                    for medium in extension_map.get("content").unwrap() {
                        if medium.name() == "media:content" {
                            for (key, val) in medium.attrs() {
                                if key == "url" {
                                    photo.image_url = val.to_string()
                                }
                            }
                        }
                    }
                }

                if extension_map.contains_key("credit") {
                    for medium in extension_map.get("credit").unwrap() {
                        if medium.name() == "media:credit" && medium.value().is_some() {
                            photo.credit = Some(medium.value().unwrap().to_string())
                        }
                    }
                }

                if extension_map.contains_key("description") {
                    for medium in extension_map.get("description").unwrap() {
                        if medium.name() == "media:description" && medium.value().is_some() {
                            photo.description = Some(medium.value().unwrap().to_string())
                        }
                    }
                }
            }
        }

        if photo.valid() {
            results.push(photo)
        }
    }
    results
}
