use app::App;
use input::Button;
use serde::Serialize;
use serde::Deserialize;
use serde::Serializer;

pub struct Axis {
    pos: Button,
    neg: Button,
}

const KEYBOARD: &'static str = "Keyboard";
const MOUSE: &'static str = "Mouse";
const CONTROLLER: &'static str = "Controller";

impl Axis {
    pub fn new(pos: Button, neg: Button) -> Axis {
        Axis {
            pos: pos,
            neg: neg,
        }
    }

    pub fn get_value(&self, app: &App) -> f64 {
        let pos_bool = app.is_button_down(self.pos);
        if pos_bool == app.is_button_down(self.neg) {
            return 0.0;
        }
        if pos_bool {
            return 1.0;
        }
        -1.0 // They aren't equal and positive is false so negative is true.
    }

    fn serialize_button<S: Serializer>(button: Button, serializer: &mut S, name: &'static str)
        -> Result<(), S::Error>
    {
        let mut state = serializer.serialize_map(Some(3))?;
        serializer.serialize_map_key(&mut state, name)?;
        match button {
            Button::Keyboard(key) => {
                let mut key_state = serializer.serialize_map(Some(2))?;
                serializer.serialize_map_key(&mut key_state, "Type")?;
                serializer.serialize_map_value(&mut key_state, KEYBOARD)?;
                serializer.serialize_map_key(&mut key_state, "Key")?;
                serializer.serialize_map_value(&mut key_state, key as u16)?;
                serializer.serialize_map_end(key_state)?;
            }
            Button::Mouse(button) => {
                let mut mouse_state = serializer.serialize_map(Some(2))?;
                serializer.serialize_map_key(&mut mouse_state, "Type")?;
                serializer.serialize_map_value(&mut mouse_state, MOUSE)?;
                serializer.serialize_map_key(&mut mouse_state, "Button")?;
                serializer.serialize_map_value(&mut mouse_state, button as u8)?;
                serializer.serialize_map_end(mouse_state)?;
            }
            Button::Controller(controller_button) => {
                let mut controller_state = serializer.serialize_map(Some(3))?;
                serializer.serialize_map_key(&mut controller_state, "Type")?;
                serializer.serialize_map_value(&mut controller_state, CONTROLLER)?;
                serializer.serialize_map_key(&mut controller_state, "Id")?;
                serializer.serialize_map_value(&mut controller_state, controller_button.id)?;
                serializer.serialize_map_key(&mut controller_state, "Button")?;
                serializer.serialize_map_value(&mut controller_state, controller_button.button)?;
                serializer.serialize_map_end(controller_state)?;

            }
        }
        serializer.serialize_map_end(state)?;
        Ok(())
    }
}

impl Serialize for Axis {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error> where S: Serializer {
        let mut state = serializer.serialize_map(Some(2))?;
        serializer.serialize_map_key(&mut state, "Axis")?;
        Axis::serialize_button(self.pos, serializer, "Pos")?;
        Axis::serialize_button(self.neg, serializer, "Neg")?;
        serializer.serialize_map_end(state)?;
        Ok(())
    }
}
