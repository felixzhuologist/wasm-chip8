pub struct Keypad {
    /// 16 key hexadecimal keypad. keys[i] is true if it is currently
    /// being pressed
    pub keys: [bool; 16],
}

// TODO: check bounds of key method args
impl Keypad {
    pub fn new() -> Keypad {
        Keypad { keys: [false; 16] }
    }

    pub fn is_key_down(&self, key: u8) -> bool {
        self.keys[key as usize]
    }

    pub fn get_first_key_down(&self) -> Option<u8> {
        for i in 0..16 {
            if self.is_key_down(i) {
                return Some(i)
            }
        }
        None
    }

    pub fn key_down(&mut self, key: u8) {
        self.keys[key as usize] = true;
    }

    pub fn key_up(&mut self, key: u8) {
        self.keys[key as usize] = false;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn set_keys() {
        let mut keypad = Keypad::new();
        for i in 0..16 {
            assert!(!keypad.is_key_down(i));
        }

        keypad.key_down(0);
        keypad.key_down(15);
        assert!(keypad.is_key_down(0));
        assert!(keypad.is_key_down(15));

        keypad.key_up(0);
        keypad.key_down(9);
        assert!(!keypad.is_key_down(0));
        assert!(keypad.is_key_down(9));
        assert!(keypad.is_key_down(15));
    }
}