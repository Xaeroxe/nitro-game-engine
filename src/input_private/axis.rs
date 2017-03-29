use input::Button;
use input_private::input::Input;

/// An input axis, could be bound to W and S, A and D, up and down, or left and
/// right.
#[derive(Serialize, Deserialize, Clone)]
pub struct Axis {
    pos: Button,
    neg: Button,
}

impl Axis {
    pub fn new(pos: Button, neg: Button) -> Axis {
        Axis {
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
