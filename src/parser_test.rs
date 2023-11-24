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
        r###"Gaza City. Hamas seized control of Gaza in 2007, after winning legislative elections the previous year."###,
        r###"Dr. Benjamin Neel was fired over his pro-Israel postings on social media. Now he is suing his employer, NYU Langone Health."###,
        r###"Much of Americans’ spending starts well before the day after Thanksgiving, particularly online."###,
        r###"Nordstrom’s visual team tackles decorating stores beginning on the Monday evening before Thanksgiving."###,
        r###"Navigating the Black Friday Travels Sales and Deals"###,
        r###"A family of migrants from China in Texas after surrendering to Border Patrol in April."###,
        r###"Riot police in Dublin on Thursday."###,
        r###"Nikki Haley still trails former President Donald J. Trump by a wide margin in polls, but she has gained ground on Gov. Ron DeSantis of Florida, who has held the No. 2 spot in national surveys all year."###,
        r###"Cliff Albright, co-founder and executive director of the Black Voters Matter Fund, said donors and party leaders were weighing heavier investments in swing states like Michigan, Pennsylvania and Wisconsin."###,
        r###"A shop in Buenos Aires, where prices are often written in chalk to allow for frequent increases."###,
        r###"Oscar Pistorius arriving at the Pretoria High Court in 2016. A parole board granted Mr. Pistorius’s petition for parole on the basis that he had served half of his 15-year sentence."###,
        r###"Boys from the Leones baseball team at the Hermes Barros Cabas baseball stadium in Bogotá."###,
        r###"George Santos and the Very Good Reason People Lie About Nonsense"###,
        r###"Patagonia’s Yvon Chouinard On the High Stakes of Low Quality"###,
        r###"Ayana Elizabeth Johnson."###,
        r###"Investigators said the car was a Bentley that sped toward the bridge, hit a median and went airborne."###,
        r###"Mayor Dan Carter at city hall in Oshawa, Ontario. Homeless and addicted to drugs from his teenage years until he was 31, he was fired from more jobs than he could remember."###,
        r###"Davida Wynn became infected with the coronavirus in May 2020 when she was a nurse on a Covid unit, and became so ill she was put into a medically induced coma for six weeks."###,
        r###"Monetochka performing in Zurich this month."###,
        r###"Decked out shelves at John Derian’s new holiday shop, which was set up inside a store normally used as a furniture showroom."###,
    ];

    let actual_descriptions: Vec<String> = results
        .iter()
        .filter(|photo| photo.description.is_some())
        .map(|photo| photo.description.clone().unwrap())
        .collect();
    assert_eq!(actual_descriptions, expected_descriptions);
}

#[test]
fn parser_ignores_tracking_images() {
    /*
    The NPR feed puts its images as html in CDATA blocks in <content:encoded>,
    and also adds a tracking pixel (npr-rss-pixel.png) which we want to avoid
    parsing as the item's image.

    This test confirms the image fetch is the human-visible one.
     */

    let feed = load_fixture("npr.xml".to_string());
    assert!(feed.is_some());

    let channel = Channel::from_str(&feed.unwrap());
    assert!(channel.is_ok());

    let results = get_photos(channel.unwrap());
    assert_eq!(results.len(), 14);

    let expected_images = vec![
        "https://media.npr.org/assets/img/2023/11/24/gettyimages-545171212_wide-f811535dc28ae966a4a8f7bdf67ddedb32d2ddd2.jpg?s=600",
        "https://media.npr.org/assets/img/2023/11/21/gettyimages-17150741001_wide-32145c61f6bc746a973fc0bd85163076e06484a7.jpg?s=600",
        "https://media.npr.org/assets/img/2023/11/23/tumbleweed1_wide-3076f91497489109e1a6f8ca4b9bd4ccb5083a0a.jpg?s=600",
        "https://media.npr.org/assets/img/2023/11/22/unclaimed_9_wide-b2e5b61c87eac643d8ebff8d0bbd6be40d7c742b.jpg?s=600",
        "https://media.npr.org/assets/img/2023/11/22/gettyimages-1238265839_wide-046ef920c5ef86ca5e9be4767015bb186e07ea67.jpg?s=600",
        "https://media.npr.org/assets/img/2023/11/22/ap12071219521_wide-159ba8364b31ce07a94e9c556e8dc50722639f7a.jpg?s=600",
        "https://media.npr.org/assets/img/2023/11/23/ap23145574875419_wide-95a558d101edeb0d72230148deff52c359ef42c1.jpg?s=600",
        "https://media.npr.org/assets/img/2023/11/24/gettyimages-1801062183_wide-d016bec56d6829171a3ffad36a73640b321acfb1.jpg?s=600",
        "https://media.npr.org/assets/img/2023/11/23/ap23327703189648_wide-e0581ac6cf655f8fa93c08f81aa105a106f9348d.jpg?s=600",
        "https://media.npr.org/assets/img/2023/11/23/nup_202950_01655_wide-bd18a34d96bb995675f483cd39b31a4456456704.jpg?s=600",
        "https://media.npr.org/assets/img/2023/11/23/ap23327577669073_wide-6a98ed6c5b23a0be8255f3a43471eba93f25aa68.jpg?s=600",
        "https://media.npr.org/assets/img/2023/11/23/ap23327365871490_wide-48f7c0812c320bab175f118649f2d4711ddc8d38.jpg?s=600",
        "https://media.npr.org/assets/img/2023/11/22/npr_jordan_ayman-oghanna034_wide-1bc4631d05db2736809a5404c451400fa6052f4f.jpg?s=600",
        "https://media.npr.org/assets/img/2023/11/22/gettyimages-1711772431_wide-83e91b4ebe323d0ddcdfba8966fa951181dc6856.jpg?s=600",
    ];

    let actual_images: Vec<String> = results
        .iter()
        .map(|photo| photo.image_url.clone())
        .collect();
    assert_eq!(actual_images, expected_images);
}
