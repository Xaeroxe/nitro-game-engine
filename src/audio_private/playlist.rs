use sdl2::mixer::Music;
use rand::thread_rng;
use std::path::PathBuf;
use rand::Rng;

pub struct Playlist {
    tracks: Vec<Music<'static>>,
    current_track: usize,
    pub shuffle: bool,
    pub loop_music: bool,
}

impl Playlist {
    pub fn new() -> Playlist {
        Playlist {
            tracks: Vec::new(),
            current_track: 0,
            shuffle: false,
            loop_music: false,
        }
    }

    pub fn start(&mut self) {
        if self.shuffle {
            self.shuffle();
        }
        self.play_track(0).expect("No tracks present.");
    }

    fn shuffle(&mut self) {
        if self.tracks.len() > 1 {
            let mut new_tracks = Vec::new();

            // Make sure the new first track is never the previous last track.  Keeps from hearing
            // the same song twice in a row.
            new_tracks.push(unborrow!(self.tracks.remove(thread_rng().gen_range(0, self.tracks.len()-1))));
            while self.tracks.len() > 0 {
                new_tracks.push(unborrow!(self.tracks.remove(thread_rng().gen_range(0, self.tracks.len()))));
            }
            self.tracks = new_tracks;
        }
    }

    pub fn next_track(&mut self) {
        if self.current_track < self.tracks.len() {
            Music::halt();
        }
        if self.current_track < self.tracks.len() - 1 {
            unborrow!(self.play_track(self.current_track + 1))
                .expect("Track index invalid.");
        } else if self.loop_music {
            if self.shuffle {
                self.shuffle();
            }
            self.play_track(0).expect("Track index invalid.");
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

pub fn advance_if_needed(playlist: &mut Playlist) {
    if !Music::is_playing() {
        playlist.next_track();
    }
}
