use ears::{Sound, AudioController};
use std::collections::LinkedList;

pub struct Audio {
    sound_store: LinkedList<Sound>,
}

impl Audio {
    pub fn new() -> Audio {
        Audio{
            sound_store: LinkedList::new(),
        }
    }
    pub fn play_sound(&mut self, path: &str) {
        self.sound_store.push_front(Sound::new(("assets/sounds/".to_string() + path).as_str()).unwrap());
        self.sound_store.front_mut().unwrap().play();
    }
}
