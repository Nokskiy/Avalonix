use std::time::Duration;

pub struct Metadata {
    pub title: Option<String>,
    pub artist: Option<String>,
    pub album: Option<String>,
    pub genre: [String; 0],
    pub year: Option<u16>,
    pub duration: Duration,
}
