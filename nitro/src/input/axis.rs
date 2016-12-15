use app::App;
use input::Button;

pub struct Axis {
    pos : Button,
    neg : Button,
    name : String,
}

impl Axis {
    pub fn new(pos : Button, neg : Button, name : String) -> Axis {
        Axis{pos : pos, neg : neg, name : name}
    }

    pub fn get_value(&self, app : &App) -> f64 {
        let pos_bool = app.is_button_pressed(self.pos);
        if pos_bool == app.is_button_pressed(self.neg) {
            return 0.0;
        }
        if pos_bool {
            return 1.0;
        }
        -1.0 // They aren't equal and positive is false so negative is true.
    }

    pub fn get_name(&self) -> &str {
        self.name.as_str()
    }
}
