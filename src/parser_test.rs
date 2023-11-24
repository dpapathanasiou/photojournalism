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

#[test]
fn parser_finds_image_description_when_available() {
    /*
    The NY Times feed is a good example of well-executed content, which follows
    the RSS spec correctly.

    In particular, they add a description specific to the image (<media:description>),
    which is separate and distinct from the item description (<description>).

    This test confirms the parser always picks the former, when given both.
     */

    let feed = load_fixture("nytimes.xml".to_string());
    assert!(feed.is_some());

    let channel = Channel::from_str(&feed.unwrap());
    assert!(channel.is_ok());

    let results = get_photos(channel.unwrap());
    assert_eq!(results.len(), 20);

    let expected_descriptions = vec![
        "Gaza City. Hamas seized control of Gaza in 2007, after winning legislative elections the previous year.", 
        "Dr. Benjamin Neel was fired over his pro-Israel postings on social media. Now he is suing his employer, NYU Langone Health.", 
        "Much of Americans’ spending starts well before the day after Thanksgiving, particularly online.", 
        "Nordstrom’s visual team tackles decorating stores beginning on the Monday evening before Thanksgiving.", 
        "Navigating the Black Friday Travels Sales and Deals", 
        "A family of migrants from China in Texas after surrendering to Border Patrol in April.", 
        "Riot police in Dublin on Thursday.", 
        "Nikki Haley still trails former President Donald J. Trump by a wide margin in polls, but she has gained ground on Gov. Ron DeSantis of Florida, who has held the No. 2 spot in national surveys all year.", 
        "Cliff Albright, co-founder and executive director of the Black Voters Matter Fund, said donors and party leaders were weighing heavier investments in swing states like Michigan, Pennsylvania and Wisconsin.", 
        "A shop in Buenos Aires, where prices are often written in chalk to allow for frequent increases.", 
        "Oscar Pistorius arriving at the Pretoria High Court in 2016. A parole board granted Mr. Pistorius’s petition for parole on the basis that he had served half of his 15-year sentence.", 
        "Boys from the Leones baseball team at the Hermes Barros Cabas baseball stadium in Bogotá.", 
        "George Santos and the Very Good Reason People Lie About Nonsense", 
        "Patagonia’s Yvon Chouinard On the High Stakes of Low Quality", 
        "Ayana Elizabeth Johnson.", 
        "Investigators said the car was a Bentley that sped toward the bridge, hit a median and went airborne.", 
        "Mayor Dan Carter at city hall in Oshawa, Ontario. Homeless and addicted to drugs from his teenage years until he was 31, he was fired from more jobs than he could remember.", 
        "Davida Wynn became infected with the coronavirus in May 2020 when she was a nurse on a Covid unit, and became so ill she was put into a medically induced coma for six weeks.", 
        "Monetochka performing in Zurich this month.", 
        "Decked out shelves at John Derian’s new holiday shop, which was set up inside a store normally used as a furniture showroom.",
    ];

    let actual_descriptions: Vec<String> = results
        .iter()
        .filter(|photo| photo.description.is_some())
        .map(|photo| photo.description.clone().unwrap())
        .collect();
    assert_eq!(actual_descriptions, expected_descriptions);
}
