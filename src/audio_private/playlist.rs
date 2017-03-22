use sdl2::mixer::Music;
use rand::thread_rng;
use std::path::PathBuf;
use rand::Rng;

pub enum ShuffleLoop {
    Neither,
    Loop,
    ShuffleLoop,
}

pub struct Playlist {
    tracks: Vec<Music<'static>>,
    current_track: usize,
    pub shuffle_loop: ShuffleLoop,
}

impl Playlist {
    pub fn start(&mut self) {
        match self.shuffle_loop {
            ShuffleLoop::Neither => {
                self.play_track(0).expect("No tracks present.");
            }
            ShuffleLoop::Loop => {
                self.play_track(0).expect("No tracks present.");
            }
            ShuffleLoop::ShuffleLoop => {
                unborrow!(self.play_track(thread_rng().gen_range(0, self.tracks.len())))
                    .expect("No tracks present.");
            }
        }
    }
    pub fn next_track(&mut self) {
        match self.shuffle_loop {
            ShuffleLoop::Neither => {
                if self.current_track < self.tracks.len() - 1 {
                    unborrow!(self.play_track(self.current_track + 1))
                        .expect("Track index invalid.");
                }
            }
            ShuffleLoop::Loop => {
                if self.current_track < self.tracks.len() - 1 {
                    unborrow!(self.play_track(self.current_track + 1))
                        .expect("Track index invalid.");
                } else {
                    self.play_track(0).expect("Track index invalid.");
                }
            }
            ShuffleLoop::ShuffleLoop => {
                unborrow!(self.play_track(thread_rng().gen_range(0, self.tracks.len())))
                    .expect("Track index invalid.");
            }
        }
    }

    /// Returns err if track is not a valid index into self.tracks
    pub fn play_track(&mut self, track: usize) -> Result<(), String> {
        if self.current_track < self.tracks.len() {
            Music::halt();
        }
        if track < self.tracks.len() {
            self.current_track = track;
            self.tracks[track].play(1)?;
            return Ok(());
        } else {
            return Err("Track index invalid.".to_string());
        }
    }

    pub fn current_track(&self) -> usize {
        self.current_track
    }

    pub fn add_track(&mut self, path: &str) -> Result<(), String> {
        let mut file_path = PathBuf::from("assets");
        file_path.push("music");
        file_path.push(path);
        self.tracks.push(Music::from_file(file_path)?);
        Ok(())
    }
}

pub fn new() -> Playlist {
    Playlist {
        tracks: Vec::new(),
        current_track: 0,
        shuffle_loop: ShuffleLoop::Neither,
    }
}

pub fn advance_if_needed(playlist: &mut Playlist) {
    if !Music::is_playing() {
        playlist.next_track();
    }
}
