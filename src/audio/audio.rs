use std::collections::HashMap;
use sdl2::mixer::Chunk;
use sdl2::mixer::Channel;
use sdl2::mixer::Sdl2MixerContext;
use sdl2::mixer;
use sdl2::AudioSubsystem;
use std::path::PathBuf;
use audio::Playlist;

/// Responsible for controlling audio playback in the game.
///
/// This structure makes 128 sound channels available to the user of this library.
/// When member methods request a channel id that id must be between 0 and 127.
pub struct Audio {
    sound_cache: HashMap<String, Chunk>,
    _audio: AudioSubsystem,
    _mixer: Sdl2MixerContext,
    pub playlist: Playlist,
}

impl Audio {
    /// Plays a sound at a volume between 1.0 and 0.0
    ///
    /// path is a filename relative to assets/sounds (assets\sounds on Windows)
    pub fn play_sound(&mut self, path: &str, volume: f32) -> Result<(), String> {
        if let Some(chunk) = self.sound_cache.get(path) {
            let channel = Channel::all().play(chunk, 0)?;
            channel.set_volume((volume * 128.0) as i32);
            return Ok(());
        }
        let mut file_path = PathBuf::from("assets");
        file_path.push("sounds");
        file_path.push(path);
        let chunk = Chunk::from_file(file_path)?;
        let channel = Channel::all().play(&chunk, 0)?;
        channel.set_volume((volume * 128.0) as i32);
        self.sound_cache.insert(path.to_owned(), chunk);
        Ok(())
    }

    /// Play a sound on a user sound channel.
    pub fn play_sound_on_channel(&mut self, channel_id: i32, path: &str) -> Result<(), String> {
        if channel_id >= 0 && channel_id < 128 {
            if let Some(chunk) = self.sound_cache.get(path) {
                mixer::channel(channel_id).play(chunk, 0)?;
                return Ok(());
            }
            let mut file_path = PathBuf::from("assets");
            file_path.push("sounds");
            file_path.push(path);
            let chunk = Chunk::from_file(file_path)?;
            Channel::all().play(&chunk, 0)?;
            self.sound_cache.insert(path.to_owned(), chunk);
            return Ok(());
        }
        Err("Channel out of range.".to_string())
    }

    /// Set the volume for a user sound channel. Volume is between 0.0 and 1.0
    pub fn set_channel_volume(&mut self, channel_id: i32, volume: f32) -> Result<(), String> {
        if channel_id >= 0 && channel_id < 128 {
            mixer::channel(channel_id).set_volume((volume * 128.0) as i32);
            return Ok(());
        }
        Err("Channel out of range.".to_string())
    }

    /// Get the volume for a user sound channel. Volume is between 0.0 and 1.0
    pub fn get_channel_volume(&self, channel_id: i32) -> Result<f32, String> {
        if channel_id >= 0 && channel_id < 128 {
            return Ok((mixer::channel(channel_id).get_volume() as f32) / 128.0);
        }
        Err("Channel out of range.".to_string())
    }

    /// Pause audio output on a user sound channel.
    pub fn pause_channel(&mut self, channel_id: i32) -> Result<(), String> {
        if channel_id >= 0 && channel_id < 128 {
            mixer::channel(channel_id).pause();
            return Ok(());
        }
        Err("Channel out of range.".to_string())
    }

    /// Resume paused audio output on a user sound channel.
    pub fn resume_channel(&mut self, channel_id: i32) -> Result<(), String> {
        if channel_id >= 0 && channel_id < 128 {
            mixer::channel(channel_id).resume();
            return Ok(());
        }
        Err("Channel out of range.".to_string())
    }

    /// Returns true if a channel is not playing anything. This will still return false if the
    /// channel has a sound to play but is paused.
    pub fn channel_idle(&mut self, channel_id: i32) -> Option<bool> {
        if channel_id >= 0 && channel_id < 128 {
            return Some(mixer::channel(channel_id).is_playing());
        }
        None
    }

    pub(crate) fn new(audio: AudioSubsystem, mixer: Sdl2MixerContext) -> Audio {
    Audio {
        playlist: Playlist::new(),
        sound_cache: HashMap::new(),
        _audio: audio,
        _mixer: mixer,
    }
}
}


