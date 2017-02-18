use rodio::Sink;
use app;
use app::App;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::sync::Arc;
use rodio::Source;
use rodio::Sample;

pub struct Dj {
    sink: Sink,
    id: u64,
    drop: bool,
    game_object_listener_id: u64,
    component_listener_id: i32,
}

impl Dj {
    pub fn id(&self) -> u64 {
        self.id
    }

    pub fn drop(&mut self) {
        self.drop = true;
    }

    pub fn set_idle_listener(&mut self, game_object_id: u64, component_id: i32) {
        self.game_object_listener_id = game_object_id;
        self.component_listener_id = component_id;
    }

    pub fn play(&mut self) {
        self.sink.play();
    }

    pub fn pause(&mut self) {
        self.sink.pause();
    }

    pub fn is_paused(&self) -> bool {
        self.sink.is_paused()
    }

    pub fn set_volume(&mut self, volume: f32) {
        self.sink.set_volume(volume);
    }

    pub fn volume(&self) -> f32 {
        self.sink.volume()
    }

    pub fn queue(&mut self, app: &mut App, path: &str) {
        let sound_over = Arc::new(AtomicBool::new(false));
        self.sink.append(app::fetch_sound(app, path));
    }

    pub fn is_over(&self) -> bool {
        //TODO: Fix this.
        true
    }
}

pub fn was_dropped(dj: &Dj) -> bool {
    dj.drop
}

pub fn get_listener(dj: &Dj) -> (u64, i32) {
    (dj.game_object_listener_id, dj.component_listener_id)
}

pub fn new_dj(sink: Sink, app: &mut App) -> Dj {
    Dj {
        sink: sink,
        id: app::next_dj_id(app),
        drop: false,
        game_object_listener_id: 0,
        component_listener_id: 0,
    }
}
