// ***WARNING***:
// Many trait impls are highly dependent on the fact that this enum is identical to
// the keycodes enum from SDL2
enum_from_primitive! {
    #[derive(Eq, PartialEq, Copy, Clone, Debug, Serialize, Deserialize)]
    pub enum MouseButton {
        Unknown,
        Left,
        Right,
        Middle,
        X1,
        X2,
        Button6,
        Button7,
        Button8,
    }
}
