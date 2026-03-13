#![allow(missing_docs)]
use lofty::prelude::*;
use lofty::probe::Probe;
use std::path::Path;
use rkyv::{Archive, Deserialize, Serialize};

#[derive(Archive, Deserialize, Serialize, Debug, Clone)]
pub struct Metadata {
    pub title: Option<String>,
    pub artist: Option<String>,
    pub album: Option<String>,
    pub genre: Option<String>,
    pub year: Option<u16>,
    pub lyrics: Option<String>,
    pub bitrate: Option<u32>,
    pub duration_secs: u64, // u64 instead Duration
}

impl Metadata {
    pub fn from(track_path: &str) -> Result<Self, String> {
        let path = Path::new(&track_path);
        
        let tagged_file = Probe::open(path)
            .map_err(|e| format!("ERROR: Bad path: {}", e))?
            .read()
            .map_err(|e| format!("ERROR: Failed to read file: {}", e))?;

        let properties = tagged_file.properties();
        let tag = match tagged_file.primary_tag() {
            Some(primary_tag) => primary_tag,
            None => tagged_file.first_tag().ok_or("ERROR: No tags found!")?,
        };

        Ok(Metadata {  
            title: tag.title().map(String::from),
            artist: tag.artist().map(String::from), 
            album: tag.album().map(String::from), 
            genre: tag.genre().map(String::from), 
            year: tag.date().map(|d| d.year),
            lyrics: tag.get_string(ItemKey::Lyrics).map(String::from),
            bitrate: properties.overall_bitrate(),
            duration_secs: properties.duration().as_secs(),
        })
    }
}
