use app;
use app::App;
use sdl2::mixer::Channel;

pub struct Dj {
    channel: Option<Channel>,
    id: u64,
    volume: f32,
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

    pub fn resume(&mut self) {
        if let Some(channel) = self.channel {
           channel.resume();
        }
    }

    pub fn pause(&mut self) {
        if let Some(channel) = self.channel {
           channel.pause();
        }
    }

    pub fn is_paused(&self) -> bool {
        if let Some(channel) = self.channel {
           channel.is_paused()
        } else {
           false
        }
    }

    pub fn set_volume(&mut self, volume: f32) {
        self.volume = volume;
        if let Some(channel) = self.channel {
            channel.set_volume((volume * 128.0) as i32);
        }
    }

    pub fn volume(&self) -> f32 {
        self.volume
    }

    pub fn play_sound(&mut self, app: &mut App, path: &str) {
        if self.channel.is_none() {
            self.channel = Some(app::get_channel(app));
        }
        self.channel.unwrap().play(app::fetch_sound(app, path), 0);
    }

    pub fn is_over(&self) -> bool {
        if let Some(channel) = self.channel {
            self.channel.is_playing()
        } else {
            true
        }
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
        channel: app::zero_channel(app),
        id: app::next_dj_id(app),
        volume: 1.0,
        drop: false,
        game_object_listener_id: 0,
        component_listener_id: 0,
    }
}
