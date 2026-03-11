use rodio::{Decoder, MixerDeviceSink, Player};
use std::{fs::File, time::Duration};

pub struct MediaPlayer {
    pub player: (MixerDeviceSink, Player),
}

impl MediaPlayer {
    pub fn new() -> MediaPlayer {
        println!("Media player created");
        let handle =
            rodio::DeviceSinkBuilder::open_default_sink().expect("open default audio stream");
        let player = Player::connect_new(handle.mixer());

        let sink_and_player = (handle, player);

        MediaPlayer {
            player: sink_and_player,
        }
    }

    pub fn play(&mut self, path: String) {
        println!("PLAYBACK STARTED");

        let file = File::open(path).expect("Failed to open audio file");
        let source = Decoder::new(file).unwrap();
        self.stop();
        self.player.1.append(source);
    }

    pub fn stop(&mut self) {
        self.player.1.stop();
    }

    pub fn pause(&mut self) {
        println!("PLAYBACK PAUSED");
        self.player.1.pause();
    }

    pub fn continue_playing(&mut self) {
        println!("PLAYBACK CONTINUED");
        self.player.1.play();
    }

    pub fn seek(&mut self, dur: Duration) {
        println!("PLAYBACK SEEKED");
        _ = self.player.1.try_seek(dur);
    }
}

#[test]
fn test_play() {
    let mut mp: MediaPlayer = MediaPlayer::new();
    mp.play("D:\\music\\Three Days Grace [restored]\\2006 - One-X\\02. Pain.flac".to_string());
}
