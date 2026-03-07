#![allow(missing_docs)]

use lofty::prelude::*;
use lofty::probe::Probe;

use std::path::Path;
use std::time::Duration;

#[derive(Debug)]
pub struct Metadata {
    pub title: Option<String>,
    pub artist: Option<String>,
    pub album: Option<String>,
    pub genre: Option<String>,
    pub year: Option<u16>,
    pub duration: Duration,
}

impl Metadata {
    fn from(track_path: &str) -> Metadata {
        let path = Path::new(&track_path);

        let tagged_file = Probe::open(path)
            .expect("ERROR: Bad path provided!")
            .read()
            .expect("ERROR: Failed to read file!");

        let tag = match tagged_file.primary_tag() {
            Some(primary_tag) => primary_tag,
            None => tagged_file.first_tag().expect("ERROR: No tags found!"),
        };

        Metadata {
            title: Some(tag.title().unwrap_or_default().to_string()),
            artist: Some(tag.artist().unwrap_or_default().to_string()),
            album: Some(tag.album().unwrap_or_default().to_string()),
            genre: Some(tag.genre().unwrap_or_default().to_string()),
            year: Some(tag.date().unwrap_or_default().year),
            duration: tagged_file.properties().duration(),
        }
    }
}

#[test]
fn test_metadata_from() {
    let path = "D:\\music\\Linkin park\\Meteora\\03 Linkin Park - Somewhere I Belong.mp3";
    let metadata = Metadata::from(path);
    println!("{:#?}", metadata);
}
