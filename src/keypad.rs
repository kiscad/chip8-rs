pub struct Keypad {
    inputs: [bool; 16],
}

impl Keypad {
    pub fn new() -> Self {
        Self {
            inputs: [false; 16],
        }
    }

    pub fn is_pressed(&self, key: u8) -> bool {
        self.inputs[key as usize]
    }
}
