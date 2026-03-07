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
    pub lyrics: Option<String>,
    pub bitrate: Option<u32>,
    pub duration: Duration,
}

#[derive(Debug)]
pub struct HashableMetadata {
    pub title: Option<String>,
    pub artist: Option<String>,
    pub album: Option<String>,
}

impl Metadata {
    pub fn from(track_path: &str) -> Metadata {
        let path = Path::new(&track_path);

        let tagged_file = Probe::open(path)
            .expect("ERROR: Bad path provided!")
            .read()
            .expect("ERROR: Failed to read file!");

        let properties = tagged_file.properties();

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
            lyrics: Some(
                tag.get_string(ItemKey::Lyrics)
                    .unwrap_or_default()
                    .to_string(),
            ),
            bitrate: Some(properties.overall_bitrate().unwrap_or_default()),
            duration: properties.duration(),
        }
    }
}

impl HashableMetadata {
    pub fn from(track_path: &str) -> HashableMetadata {
        let path = Path::new(&track_path);

        let tagged_file = Probe::open(path)
            .expect("ERROR: Bad path provided!")
            .read()
            .expect("ERROR: Failed to read file!");

        let tag = match tagged_file.primary_tag() {
            Some(primary_tag) => primary_tag,
            None => tagged_file.first_tag().expect("ERROR: No tags found!"),
        };
        HashableMetadata {
            title: Some(tag.title().unwrap_or_default().to_string()),
            artist: Some(tag.artist().unwrap_or_default().to_string()),
            album: Some(tag.album().unwrap_or_default().to_string()),
        }
    }
}

#[test]
fn test_metadata_from() {
    let path = "D:\\music\\metallica\\And justice for all\\08 - To Live is to Die.flac";
    let metadata = Metadata::from(path);
    println!("{:#?}", metadata);
}
