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

    let aeon = load_fixture("aeon.xml".to_string());
    assert!(aeon.is_some());

    let channel = Channel::from_str(&aeon.unwrap());
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

    let bbc = load_fixture("bbc.xml".to_string());
    assert!(bbc.is_some());

    let channel = Channel::from_str(&bbc.unwrap());
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

    let quanta = load_fixture("quanta.xml".to_string());
    assert!(quanta.is_some());

    let channel = Channel::from_str(&quanta.unwrap());
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
