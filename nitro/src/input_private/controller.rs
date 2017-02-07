#[derive(Eq, PartialEq, Copy, Clone, Debug, Serialize, Deserialize)]
pub struct ControllerButton {
    pub id: i32,
    pub button: u8,
}
