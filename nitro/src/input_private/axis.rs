use input::Button;
use input_private::input::Input;

#[derive(Serialize, Deserialize)]
pub struct Axis {
    name: String,
    pos: Button,
    neg: Button,
}

impl Axis {
    pub fn new(name: &str, pos: Button, neg: Button) -> Axis {
        Axis {
            name: String::from(name),
            pos: pos,
            neg: neg,
        }
    }

    pub fn get_value(&self, input: &Input) -> f32 {
        let pos_bool = input.is_button_down(self.pos);
        if pos_bool == input.is_button_down(self.neg) {
            return 0.0;
        }
        if pos_bool {
            return 1.0;
        }
        -1.0 // They aren't equal and positive is false so negative is true.
    }
}
