use rodio::Sink;
use app;
use app::App;

pub struct Dj {
    sink: Sink,
}

impl Dj {
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
        self.sink.append(app::fetch_sound(app, path));
    }

    pub fn detach(self) {
        self.sink.detach();
    }
}

pub fn new_dj(sink: Sink) -> Dj {
    Dj {
        sink: sink,
    }
}
