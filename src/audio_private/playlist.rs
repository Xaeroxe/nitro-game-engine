use sdl2::mixer::Music;
use rand::thread_rng;
use std::path::PathBuf;
use rand::Rng;

/// Allows you to create and control playlists for music playback.
///
/// Playlists are optimized for music, if you want to play sound effects functions in nitro::Audio
/// should be of assistance to you.  Apps can have one active playlist at a time which lives in 
/// nitro::App.playlist.  To change the playlist stop App.playlist then assign a new playlist to
/// App.playlist and then start App.playlist.
pub struct Playlist {
    tracks: Vec<Music<'static>>,
    current_track: usize,

    /// If this is true playlist will be shuffled on start and on loop
    pub shuffle: bool,

    /// If this is true playlist will loop back to start when music is finished.
    pub loop_music: bool,
    started: bool,
}

impl Playlist {

    /// Creates a new playlist with no tracks, and shuffle and loop_music set to false.
    pub fn new() -> Playlist {
        Playlist {
            tracks: Vec::new(),
            current_track: 0,
            shuffle: false,
            loop_music: false,
            started: false,
        }
    }

    /// Start playing this playlist.
    ///
    /// Will not auto advance songs unless this playlist is attached to App::audio.playlist
    pub fn start(&mut self) {
        if self.shuffle {
            self.shuffle();
        }
        self.started = true;
        self.play_track(0).expect("No tracks present.");
    }

    /// Stops music playback
    pub fn stop(&mut self) {
        if self.started {
            self.started = false;
            Music::halt();
        }
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

    /// Advance to the next track in the playlist
    pub fn next_track(&mut self) {
        if self.current_track < self.tracks.len() - 1 {
            unborrow!(self.play_track(self.current_track + 1))
                .expect("Track index invalid.");
        } else if self.loop_music {
            if self.shuffle {
                self.shuffle();
            }
            self.play_track(0).expect("Track index invalid.");
        } else {
            Music::halt();
        }
    }

    /// Returns err if track is not a valid index into self.tracks
    pub fn play_track(&mut self, track: usize) -> Result<(), String> {
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

    /// Adds a new track to the playlist.  The first track added will be track 0 the second track
    /// will be track 1, so on so forth.
    pub fn add_track(&mut self, path: &str) -> Result<(), String> {
        let mut file_path = PathBuf::from("assets");
        file_path.push("music");
        file_path.push(path);
        self.tracks.push(Music::from_file(file_path)?);
        Ok(())
    }

    pub fn track_count(&self) -> usize {
        self.tracks.len()
    }
}

pub fn advance_if_needed(playlist: &mut Playlist) {
    if playlist.started {
        if !Music::is_playing() {
            playlist.next_track();
        }
    }
}
