use std::{
    fs::File,
    io::{BufReader, Error},
    sync::{Arc, Mutex, RwLock, mpsc::channel},
    thread,
    time::Duration,
};

use rodio::{
    Decoder, DeviceTrait, MixerDeviceSink, Player, Source,
    cpal::{self, DeviceDescription, Host, traits::HostTrait},
    mixer::Mixer,
    source::SineWave,
};

struct Playback {
    mixer: MixerDeviceSink,
    last_device_description: DeviceDescription,
    player: Player,
    last_playing_track_path: Option<String>,
}

pub struct MediaPlayer {
    playback: Option<Playback>,
}

impl<'a> Playback {
    fn new() -> Self {
        let sink_handle =
            rodio::DeviceSinkBuilder::open_default_sink().expect("open default audio stream");

        let mixer = sink_handle.mixer();
        let player = Player::connect_new(mixer);

        let host = Host::default();

        Self {
            mixer: sink_handle,
            last_device_description: host.default_output_device().unwrap().description().unwrap(),
            player,
            last_playing_track_path: None,
        }
    }

    fn change_device(&mut self) {
        let sink_handle =
            rodio::DeviceSinkBuilder::open_default_sink().expect("open default audio stream");

        let file_path = self.last_playing_track_path.as_ref().unwrap();

        self.player.stop();

        let player = Player::connect_new(&sink_handle.mixer());

        let host = Host::default();

        let file = BufReader::new(File::open(&file_path).unwrap());
        let source = Decoder::new(file).unwrap();

        player.append(source);

        self.mixer = sink_handle;
        self.last_device_description = host.default_output_device().unwrap().description().unwrap();
        self.player = player;
    }

    fn play(&mut self, file_path: String) {
        let file = BufReader::new(File::open(file_path).unwrap());
        let source = Decoder::new(file).unwrap();
        self.player.stop();
        self.player.append(source);
    }
}

impl MediaPlayer {
    pub fn new() -> Self {
        Self {
            playback: Some(Playback::new()),
        }
    }

    pub fn play(&mut self, file_path: String) {
        let playback = self.playback.as_mut().unwrap();

        playback.last_playing_track_path = Some(file_path.clone());
        playback.play(file_path);
    }

    pub fn stop(&mut self) {
        self.playback.as_ref().unwrap().player.stop();
    }

    pub fn get_pos(&mut self) -> Duration {
        self.playback.as_ref().unwrap().player.get_pos()
    }

    pub fn update(clone: Arc<Mutex<MediaPlayer>>) {
        thread::spawn(move || {
            let host = Host::default();
            loop {
                let device = host.default_output_device();
                match device {
                    Some(_) => {}
                    None => println!("can`t find default device"),
                }
                thread::sleep(Duration::new(1, 0));
                {
                    match device {
                        Some(device) => {
                            let mut guard = clone.try_lock().unwrap();

                            if guard.playback.as_ref().unwrap().last_device_description
                                != device.description().unwrap()
                            {
                                let playback: &mut Playback = guard.playback.as_mut().unwrap();
                                playback.change_device();
                            }
                        }
                        None => {}
                    }
                }
            }
        });
    }
}

#[test]
fn test_play() {
    let player = Arc::new(Mutex::new(MediaPlayer::new()));

    let clone1 = player.clone();
    let clone2 = player.clone();

    let file_path =
        "D:\\music\\Three Days Grace [restored]\\2006 - One-X\\03. Animal I Have Become.flac"
            .to_string();

    MediaPlayer::update(clone2);

    clone1.lock().as_mut().unwrap().play(file_path);

    loop {}
}
// it took too much nerves
