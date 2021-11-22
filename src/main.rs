
use std::fs;
use std::io::Write;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;


fn main() {
    let user = "test";
    let page = "1";
    get(&user, &page);
}

// make a request to lastFM api and return the respons

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


//fn get(user: &str, page: &str) -> Result<(), Error> {
//    let url = format!("http://ws.audioscrobbler.com/2.0/?method=user.getrecenttracks&user={}&api_key=6f77c6b8b6d0e6f8d6e9f6f9f6f9f6f&page={}&limit=200&format=json", user, page);
//    let res = reqwest::blocking::get(&url).unwrap();
//
//    let json: serde_json::Value = serde_json::from_str(&res.text();
//
//    let tracks = json["recenttracks"]["track"];
//    for track in tracks.as_array().unwrap() {
//        let artist = track["artist"]["#text"].as_str().unwrap();
//        let name = track["name"].as_str().unwrap();
//        let album = track["album"]["#text"].as_str().unwrap();
//        let url = track["url"].as_str().unwrap();
//        let nowplaying = track["@attr"]["nowplaying"].as_str().unwrap();
//        if nowplaying == "true" {
//            println!("{} - {} - {} - {}", artist, name, album, url);
//        }
//    }
//    Ok(())
//}
