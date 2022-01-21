# LastFM
Simple LastFM profile scraper, written in rust (WIP)

## Why
- Why not?
- No JS dependency
- Can run locally
- Automation

## Data Format
- Artist
- Name
- Date

e.x Yakui The Maid - Calamity : 20 Nov 2021, 03:19

## Building 

```
git clone https://github.com/stableversion/LastFM
```
```
cd LastFM
```
```
cargo build --release
```
```
cd target/release
```
```
./lasfm_scrape
```
or windows (from command line)
```
lasfm_scrape.exe
```


## Usage

### Linux

```
./lastfm_scrape --user test
```

```
./lastfm_scrape --user test --output /home/test/Documents/ --type csv
```

### Windows

Haven't checked, should be the same, just use "lastfm_scrape.exe" instead of "./"


## Todo
- ~~Handle multiple pages~~
- Output to other formats like CSV, etc. (No multi-page support, yet)
- ~~Specify output directory~~
- ~~Command line options~~
- ~~Error Handling~~ 
