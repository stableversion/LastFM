
use std::fs;
use std::io::Write;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use structopt::StructOpt;
use std::process;

#[derive(StructOpt, Debug)]
#[structopt(name = "LastFM")]
struct Opt {
    #[structopt(short = "u", long = "user", help = "LastFM username")]
    user: String,
    #[structopt(short = "o", long = "output", help = "Filepath for output", default_value = "")]
    output: String,
    #[structopt(short = "t", long = "type", help = "CSV/Txt", default_value = "txt")]
    csv: String,
}


fn main() {
    let user = Opt::from_args().user;
    let path = Opt::from_args().output; 
    let file_input = Opt::from_args().csv; 
    
    if file_input.to_lowercase().eq("txt") {
        get_all_txt(&user, &path);
    } else if file_input.to_lowercase().eq("csv") {
        let tempget = get(&user, "1");
        save_to_csv(&user, &tempget, &path)
    } else {
        println!("Please use either txt or csv as file type");
        process::exit(1);
    }

}

// get LastFM json

 fn get(user: &str, page: &str) -> String{    
    let url = format!("http://ws.audioscrobbler.com/2.0/?method=user.getrecenttracks&user={}&api_key=b25b959554ed76058ac220b7b2e0a026&limit=300&page={}&format=json", user, page);
    let response = reqwest::blocking::get(&url).unwrap();
    if response.status().is_success() {
        let text = response.text().unwrap();
        return text;
    } else {
        println!("Username not found. Status: {:?}", response.status());
        process::exit(1);
    }
    
}

// Parses the json response and formats it to readable text

fn format_txt(text: &str) -> String {
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

fn get_page_number(text: &str) -> String {
    let json: Response = serde_json::from_str(&text).unwrap();
    let page_number = json.recenttracks.attr.totalPages;
    return page_number.to_string();
}

// Saves the string to txt file

fn save_to_file(tracks_str: &str, user: &str, total_file_name: &str) {           
    let file_name = format!("{}{}_tracks.txt", total_file_name, user);    
    let _file = fs::File::create(&file_name).unwrap();
    let mut file = fs::OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(&file_name)
        .unwrap();
    file.write_all(tracks_str.as_bytes()).unwrap();
}

// Combines all functions

fn get_all_txt(user: &str, filep: &str) {
    let page_number = get_page_number(&get(user, "1"));
    let mut page_number_str = String::new();
    let mut total_str = String::new();
    for page in 1..(page_number.parse::<i32>().unwrap()) + 1{
        let page_str = format!("{}", page);
        page_number_str.push_str(&page_str);
        println!("Scraping page {}", page_str);
        let get_str = format_txt(&get(user, &page_str));
        total_str.push_str(&get_str);
    };
    save_to_file(&total_str, user, filep);
    println!("Done");
}

fn save_to_csv(user: &str, text: &str, path: &str) {
    let json: Response = serde_json::from_str(&text).unwrap();
    let tracks: Vec<Track> = json.recenttracks.track;

    let mut wtr = csv::Writer::from_path(format!("{}{}_tracks.csv", path, user)).unwrap();
    wtr.write_record(&["Artist", "Track", "Date"]).unwrap();
    
    for track in tracks {
        let Artist = format!("{}", track.artist["#text"]);
        let Track = format!("{}", track.name);
        let Date = format!("{}", track.date["#text"]);
        wtr.write_record(&[Artist, Track, Date]);
    }
    wtr.flush();
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
