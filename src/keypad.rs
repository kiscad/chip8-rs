pub struct Keypad {
    inputs: [bool; 16],
}

impl Keypad {
    pub fn new() -> Self {
        Self {
            inputs: [false; 16],
        }
    }
}
