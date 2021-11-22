
use std::fs;
use std::io::Write;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;


fn main() {
    let user = "test";
    let page = "1";
    get(&user, &page);
}

// make a request to lastFM api and return the response
// then format artist, name and date into a single txt file

fn get(user: &str, page: &str){
    let url = format!("http://ws.audioscrobbler.com/2.0/?method=user.getrecenttracks&user={}&api_key=b25b959554ed76058ac220b7b2e0a026&limit=300&page={}&format=json", user, page);
    let response = reqwest::blocking::get(&url).unwrap();
    let text = response.text().unwrap();
    let json: Response = serde_json::from_str(&text).unwrap();
    let tracks: Vec<Track> = json.recenttracks.track;
    let mut tracks_str = String::new();
    for track in tracks {
        let track_str = format!("{} - {} : {}\n", track.artist["#text"], track.name, track.date["#text"]);
        tracks_str.push_str(&track_str);
    }
    let file_name = format!("{}_tracks.txt", user);
    let _file = fs::File::create(&file_name).unwrap();
    let mut file = fs::OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(&file_name)
        .unwrap();
    file.write_all(tracks_str.as_bytes()).unwrap();
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

// structure for the recenttracks
#[derive(Deserialize)]
struct RecentTracks {
    track: Vec<Track>,
}
