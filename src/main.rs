
use std::fs;
use std::io::Write;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "LastFM")]
struct Opt {
    #[structopt(short = "u", long = "user", help = "LastFM username")]
    user: String,
}


fn main() {
    let user = Opt::from_args().user;
    get_all(&user);

}

// LastFM API request

fn get(user: &str, page: &str) -> String {
    let url = format!("http://ws.audioscrobbler.com/2.0/?method=user.getrecenttracks&user={}&api_key=b25b959554ed76058ac220b7b2e0a026&limit=300&page={}&format=json", user, page);
    let response = reqwest::blocking::get(&url).unwrap();
    let text = response.text().unwrap();
    return text;
}

// Parses the json response and formates it to readable text

fn format_txt(text: String) -> String {
    let json: Response = serde_json::from_str(&text).unwrap();
    let tracks: Vec<Track> = json.recenttracks.track;
    let mut tracks_str = String::new();
    for track in tracks {
        let track_str = format!("{} - {} : {}\n", track.artist["#text"], track.name, track.date["#text"]);
        tracks_str.push_str(&track_str);
    }
    return tracks_str;
}

// Gets page number from json response

fn get_page_number(text: String) -> String {
    let json: Response = serde_json::from_str(&text).unwrap();
    let page_number = json.recenttracks.attr.totalPages;
    return page_number.to_string();
}

// Saves the string to txt file

fn save_to_file(tracks_str: String, user: &str) {           
    let file_name = format!("{}_tracks.txt", user);
    let _file = fs::File::create(&file_name).unwrap();
    let mut file = fs::OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(&file_name)
        .unwrap();
    file.write_all(tracks_str.as_bytes()).unwrap();
}

// Combines all functions

fn get_all(user: &str) {
    let page_number = get_page_number(get(user, "1"));
    let mut page_number_str = String::new();
    let mut total_str = String::new();
    for page in 1..(page_number.parse::<i32>().unwrap()) + 1{
        let page_str = format!("{}", page);
        page_number_str.push_str(&page_str);
        println!("Scraping page {}", page_str);
        let get_str = format_txt(get(user, &page_str));
        total_str.push_str(&get_str);
    };
    save_to_file(total_str, user);
    println!("Done");
}

// structure for a track
#[derive(Deserialize)]
struct Track {
    artist: HashMap<String, String>,
    name: String,
    date: HashMap<String, String>,
}

// structure for the response
#[derive(Deserialize)]
struct Response {
    recenttracks: RecentTracks,

}
// structure for @attr
#[derive(Deserialize)]
struct Attr {
    totalPages: String,
}

// structure for recenttracks
#[derive(Deserialize)]
struct RecentTracks {
    track: Vec<Track>,
    #[serde(rename = "@attr")]
    attr: Attr,
}