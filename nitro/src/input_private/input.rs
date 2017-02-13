use input::Button;
use input::Axis;
use std::io::Result as IOResult;
use std::io::Error;
use std::io::Write;
use std::io::Read;
use std::fs::File;
use std::clone::Clone;
use std::collections::HashMap;
use bincode::SizeLimit;
use bincode::serde::{serialize, deserialize, DeserializeError};
use piston_window;

pub struct Input {
    buttons_pressed: Vec<Button>,
    previous_buttons_pressed: Vec<Button>, // buttons_pressed from last frame.
    axes: HashMap<i32, Axis>,
    actions: HashMap<i32, Button>,
}

#[derive(Debug)]
pub enum ReadBincodeFileError {
    IOError(Error),
    BincodeError(DeserializeError),
}

impl From<Error> for ReadBincodeFileError {
    fn from(err: Error) -> ReadBincodeFileError {
        ReadBincodeFileError::IOError(err)
    }
}

impl From<DeserializeError> for ReadBincodeFileError {
    fn from(err: DeserializeError) -> ReadBincodeFileError {
        ReadBincodeFileError::BincodeError(err)
    }
}

impl Input {
    pub fn new() -> Input {
        Input {
            buttons_pressed: vec![],
            previous_buttons_pressed: vec![],
            axes: HashMap::new(),
            actions: HashMap::new(),
        }
    }

    pub fn load_bindings(&mut self, path: &str) -> Result<(), ReadBincodeFileError> {
        let mut f = File::open(path)?;
        let mut buf = Vec::new();
        f.read_to_end(&mut buf)?;
        let (axes, actions) =
            deserialize::<(HashMap<i32, Axis>, HashMap<i32, Button>)>(buf.as_slice())?;
        self.axes = axes;
        self.actions = actions;
        Ok(())
    }

    pub fn save_bindings(&mut self, path: &str) -> IOResult<()> {
        let mut f = File::create(path)?;
        let result = serialize(&(&self.axes, &self.actions), SizeLimit::Infinite).unwrap();
        f.write_all(result.as_slice())?;
        Ok(())
    }

    pub fn add_axis(&mut self, axis: Axis, id: i32) {
        if self.axes.contains_key(&id) {
            panic!("Axis id values must be unique!");
        }
        self.axes.insert(id, axis);
    }

    pub fn get_axis_value(&self, id: i32) -> Option<f32> {
        if let Some(axis) = self.axes.get(&id) {
            return Some(axis.get_value(self));
        }
        None
    }

    pub fn add_action(&mut self, button: Button, id: i32) {
        if self.actions.contains_key(&id) {
            panic!("Action id values must be unique!");
        }
        self.actions.insert(id, button);
    }

    // Named for consistency with get_axis_value
    pub fn action_pressed(&self, id: i32) -> Option<bool> {
        if let Some(button) = self.actions.get(&id) {
            return Some(self.is_button_pressed(*button));
        }
        None
    }

    pub fn action_down(&self, id: i32) -> Option<bool> {
        if let Some(button) = self.actions.get(&id) {
            return Some(self.is_button_down(*button));
        }
        None
    }

    pub fn action_released(&self, id: i32) -> Option<bool> {
        if let Some(button) = self.actions.get(&id) {
            return Some(self.is_button_released(*button));
        }
        None
    }

    pub fn is_button_pressed(&self, button: Button) -> bool {
        (&self.buttons_pressed).into_iter().any(|&b| b == button) &&
        !(&self.previous_buttons_pressed).into_iter().any(|&b| b == button)
    }

    pub fn is_button_down(&self, button: Button) -> bool {
        (&self.buttons_pressed).into_iter().any(|&b| b == button)
    }

    pub fn is_button_released(&self, button: Button) -> bool {
        !(&self.buttons_pressed).into_iter().any(|&b| b == button) &&
        (&self.previous_buttons_pressed).into_iter().any(|&b| b == button)
    }
}

pub fn process_event(input: &mut Input, input_event: piston_window::Input) {
    match input_event {
        piston_window::Input::Press(button) => {
            // The button may already be here as this event does repeat.
            if !input.buttons_pressed.iter().any(|&item| item == Button::from(button)) {
                input.buttons_pressed.push(Button::from(button));
            }
        }
        piston_window::Input::Release(button) => {
            while let Some(i) = input.buttons_pressed
                .iter()
                .position(|&item| item == Button::from(button)) {
                input.buttons_pressed.swap_remove(i);
            }
        }
        _ => {}
    }
}

pub fn shift_frame(input: &mut Input) {
    input.previous_buttons_pressed = input.buttons_pressed.clone();
}
