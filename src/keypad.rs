pub struct Keypad {

}

impl Keypad {
    pub fn new() -> Keypad {
        Keypad {}
    }

    pub fn is_key_down(&self, key: u8) -> bool {
        // TODO: check bounds of key
        true
    }

    pub fn wait_for_key_down(&self) -> u8 {
        0
    }
}
