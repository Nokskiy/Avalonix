use std::{
    fs::File,
    io::{BufReader, Error},
    sync::{Arc, Mutex, mpsc::channel},
    thread,
    time::Duration,
};

use rodio::{
    DeviceTrait, MixerDeviceSink, Player, Source,
    cpal::{self, DeviceDescription, Host, traits::HostTrait},
    mixer::Mixer,
    source::SineWave,
};

struct Playback {
    mixer: MixerDeviceSink,
    last_device_description: DeviceDescription,
    player: Player,
    last_playing_track_path: Option<String>,
    last_playing_time: Option<Duration>,
}

pub struct MediaPlayer {
    playback: Option<Playback>,
}

impl<'a> Playback {
    fn new() -> Self {
        let sink_handle =
            rodio::DeviceSinkBuilder::open_default_sink().expect("open default audio stream");

        // Load a sound from a file, using a path relative to Cargo.toml
        let file = BufReader::new(
            File::open("D:\\music\\Three Days Grace [restored]\\2006 - One-X\\03. Animal I Have Become.flac").unwrap(),
        );
        // Note that the playback stops when the player is dropped
        let player = rodio::play(&sink_handle.mixer(), file).unwrap();

        let host = Host::default();
        let dd = host.default_output_device().unwrap();
        thread::sleep(Duration::new(5, 0));
        let dd2 = host.default_output_device().unwrap();
        let a = dd.description().unwrap();
        if (dd.description().unwrap() != dd2.description().unwrap()) {
            println!("NOT EQ");
        } else if (dd.description().unwrap() == dd2.description().unwrap()) {
            println!("EQ");
        } else {
            println!("I DONT KNOW");
        }

        Self {
            mixer: sink_handle,
            last_device_description: dd.description().unwrap(),
            player,
            last_playing_track_path: None,
            last_playing_time: None,
        }
    }
}

impl MediaPlayer {
    pub fn new() -> Self {
        Self {
            playback: Some(Playback::new()),
        }
    }

    pub fn play(&mut self) {
        let playback = self.playback.as_ref().unwrap();
        let player = &playback.player;

        let source = SineWave::new(440.0)
            .take_duration(Duration::from_secs_f32(0.25))
            .amplify(0.20)
            .repeat_infinite();
        player.append(source);
    }

    pub fn stop(&mut self) {
        self.playback.as_ref().unwrap().player.stop();
    }

    pub fn update() {}
}

#[test]
fn test_play() {
    let mut mp: MediaPlayer = MediaPlayer::new();

    loop {}
}
