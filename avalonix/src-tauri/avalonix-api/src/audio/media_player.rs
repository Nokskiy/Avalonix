use rodio::{Decoder, Player};
use std::{fs::File, thread, time::Duration};

pub struct MediaPlayer {
    pub player: Player,
}

impl MediaPlayer {
    pub fn new() -> MediaPlayer {
        let (tx, rx) = std::sync::mpsc::channel();

        thread::spawn(move || {
            let handle =
                rodio::DeviceSinkBuilder::open_default_sink().expect("open default audio stream");

            let mixer = handle.mixer();
            let player = Player::connect_new(mixer);

            _ = tx.send(player);

            loop {}
        });

        MediaPlayer {
            player: rx.recv().unwrap(),
        }
    }

    pub fn play(&mut self, path: String) {
        let file = File::open(path).expect("Failed to open audio file");
        let source = Decoder::new(file).unwrap();

        self.stop();
        self.player.append(source);
    }

    pub fn stop(&mut self) {
        self.player.stop();
    }

    pub fn pause(&mut self) {
        self.player.pause();
    }

    pub fn continue_playing(&mut self) {
        self.player.play();
    }

    pub fn seek(&mut self, dur: Duration) {
        _ = self.player.try_seek(dur);
    }
}

#[test]
fn test_play() {
    let mut mp = MediaPlayer::new();
    mp.play("D:\\music\\Three Days Grace [restored]\\2006 - One-X\\02. Pain.flac".to_string());
    thread::sleep(Duration::new(1, 0));
    mp.pause();
    thread::sleep(Duration::new(1, 0));
    mp.continue_playing();
    thread::sleep(Duration::new(1, 0));
    mp.seek(Duration::new(120, 0));
    thread::sleep(Duration::new(1, 0));
}
