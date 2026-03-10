use rodio::Decoder;
use std::{fs::File, io::BufReader};

pub struct MediaPlayer {
    pub player: Option<rodio::Player>,
}

impl MediaPlayer {
    pub fn new() -> MediaPlayer {
        MediaPlayer { player: None }
    }

    pub fn play(&mut self, path: &str) {
        // It is necessary to make playback in a separate stream

        let mut handle =
            rodio::DeviceSinkBuilder::open_default_sink().expect("open default audio stream");
        handle.log_on_drop(false);
        self.player = Some(rodio::Player::connect_new(&handle.mixer()));

        let file = File::open(path).unwrap();
        let source = Decoder::new(BufReader::new(file)).unwrap();

        self.player.as_ref().unwrap().append(source);
        self.player.as_ref().unwrap().sleep_until_end();
        self.player.as_ref().unwrap().stop();
    }
}

#[test]
fn test_play() {
    let mut mp = MediaPlayer::new();
    mp.play("D:\\music\\Three Days Grace [restored]\\2006 - One-X\\02. Pain.flac");
    mp.play("D:\\music\\Three Days Grace [restored]\\2006 - One-X\\03. Animal I Have Become.flac");
}
