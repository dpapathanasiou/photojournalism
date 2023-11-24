use super::*;
use rss::Channel;
use std::env::current_dir;
use std::fs::read_to_string;
use std::path::Path;
use std::str::FromStr;

fn load_fixture(filename: String) -> Option<String> {
    if let Ok(path) = current_dir() {
        let fixture_file = format!("tests/fixtures/{filename}");
        let file_path = Path::new(&fixture_file);
        let fixture_path = path.join(file_path);
        return match read_to_string(fixture_path) {
            Ok(data) => Some(data),
            Err(_) => None,
        };
    }
    None
}

#[test]
fn parser_can_find_images_in_cdata() {
    /*
    Aeon's RSS has images, but they are embedded as html img tags inside the
    CDATA block of descriptions (see the corresponding fixture file for an example).

    While this is non-standard, the parser is capable of finding them, which
    is what this test confirms.
     */

    let feed = load_fixture("aeon.xml".to_string());
    assert!(feed.is_some());

    let channel = Channel::from_str(&feed.unwrap());
    assert!(channel.is_ok());

    let results = get_photos(channel.unwrap());
    assert_eq!(results.len(), 20);
}

#[test]
fn parser_ignores_feeds_without_images() {
    /*
    The BBC feed does not contain any images (which is perfectly fine
    according to the RSS spec, it's just that we cannot list it among the
    feeds to collect here).
     */

    let feed = load_fixture("bbc.xml".to_string());
    assert!(feed.is_some());

    let channel = Channel::from_str(&feed.unwrap());
    assert!(channel.is_ok());

    let results = get_photos(channel.unwrap());
    assert_eq!(results.len(), 0);
}

#[test]
fn parser_finds_dublin_core_correctly() {
    /*
    Quanta Magazine's feed uses Dublin Core for photo credits, inside CDATA blocks,
    so this test confirms that the parser can get to underlying strings correctly.
     */

    let feed = load_fixture("quanta.xml".to_string());
    assert!(feed.is_some());

    let channel = Channel::from_str(&feed.unwrap());
    assert!(channel.is_ok());

    let results = get_photos(channel.unwrap());
    assert_eq!(results.len(), 5);

    let expected_credits = vec![
        "Patrick Honner",
        "Yasemin Saplakoglu",
        "Madison Goldberg",
        "Alex Stone",
        "Stephen Ornes",
    ];
    let actual_credits: Vec<String> = results
        .iter()
        .filter(|photo| photo.credit.is_some())
        .map(|photo| photo.credit.clone().unwrap())
        .collect();
    assert_eq!(actual_credits, expected_credits);
}

#[test]
fn parser_finds_image_content_when_given_both_thumbnails_and_content() {
    /*
    The Japan Times feed presents both a thumbnail (<media:thumbnail />) along with
    a corresponding full-size image (<media:content />).

    This test confirms the parser always picks the latter, when given both.
     */

    let feed = load_fixture("japantimes.xml".to_string());
    assert!(feed.is_some());

    let channel = Channel::from_str(&feed.unwrap());
    assert!(channel.is_ok());

    let results = get_photos(channel.unwrap());
    assert_eq!(results.len(), 30);

    let expected_images = vec![
        "https://www.japantimes.co.jp/japantimes/uploads/images/2023/11/24/265519.jpg",
        "https://www.japantimes.co.jp/japantimes/uploads/images/2023/11/24/265520.jpg",
        "https://www.japantimes.co.jp/japantimes/uploads/images/2023/11/24/265504.jpg",
        "https://www.japantimes.co.jp/japantimes/uploads/images/2023/11/24/265517.jpg",
        "https://www.japantimes.co.jp/japantimes/uploads/images/2023/11/24/265515.jpg",
        "https://www.japantimes.co.jp/japantimes/uploads/images/2023/11/24/264691.jpg",
        "https://www.japantimes.co.jp/japantimes/uploads/images/2023/11/24/265449.jpg",
        "https://www.japantimes.co.jp/japantimes/uploads/images/2023/11/24/265482.jpg",
        "https://www.japantimes.co.jp/japantimes/uploads/images/2023/11/24/265518.JPG",
        "https://www.japantimes.co.jp/japantimes/uploads/images/2023/11/24/265295.jpg",
        "https://www.japantimes.co.jp/japantimes/uploads/images/2023/11/24/265453.JPG",
        "https://www.japantimes.co.jp/japantimes/uploads/images/2023/11/24/265428.jpg",
        "https://www.japantimes.co.jp/japantimes/uploads/images/2023/11/24/264355.jpg",
        "https://www.japantimes.co.jp/japantimes/uploads/images/2023/11/24/264360.jpg",
        "https://www.japantimes.co.jp/japantimes/uploads/images/2023/11/24/265460.jpg",
        "https://www.japantimes.co.jp/japantimes/uploads/images/2023/11/24/265323.JPG",
        "https://www.japantimes.co.jp/japantimes/uploads/images/2023/11/24/265385.JPG",
        "https://www.japantimes.co.jp/japantimes/uploads/images/2023/11/24/265370.jpg",
        "https://www.japantimes.co.jp/japantimes/uploads/images/2023/11/24/265315.jpg",
        "https://www.japantimes.co.jp/japantimes/uploads/images/2023/11/24/265342.jpg",
        "https://www.japantimes.co.jp/japantimes/uploads/images/2023/11/13/262368.jpg",
        "https://www.japantimes.co.jp/japantimes/uploads/images/2023/11/24/265444.jpg",
        "https://www.japantimes.co.jp/japantimes/uploads/images/2023/11/24/265332.jpg",
        "https://www.japantimes.co.jp/japantimes/uploads/images/2023/11/24/265265.jpg",
        "https://www.japantimes.co.jp/japantimes/uploads/images/2023/11/24/265326.jpg",
        "https://www.japantimes.co.jp/japantimes/uploads/images/2023/11/24/265266.JPG",
        "https://www.japantimes.co.jp/japantimes/uploads/images/2023/11/24/265337.JPG",
        "https://www.japantimes.co.jp/japantimes/uploads/images/2023/11/24/265307.jpg",
        "https://www.japantimes.co.jp/japantimes/uploads/images/2023/11/24/265244.jpg",
        "https://www.japantimes.co.jp/japantimes/uploads/images/2023/11/24/265256.jpg",
    ];

    let actual_images: Vec<String> = results
        .iter()
        .map(|photo| photo.image_url.clone())
        .collect();
    assert_eq!(actual_images, expected_images);
}
