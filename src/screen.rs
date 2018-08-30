/// 64 x 32 monochrome display. (0, 0) is the top left pixel
/// and (63, 31) is the bottom right pixel
pub struct Screen {
    pixels: [u64; 32],
}

/// Return the byte of the row of pixels that would be overwritten by a sprite
/// with the specified width at index x. Assumes that the sprite would be in bounds.
/// Intuitively, this shifts the row so that the affected byte can be compared
/// to the sprite row
fn get_old_byte(row: u64, x: usize, sprite_width: usize) -> u8 {
    (row >> (64 - sprite_width - x)) as u8
}

/// Given a sprite row with 8 pixels, shift it to create a mask that can be
/// applied to a row of pixels of size 64
fn get_mask(sprite_row: u8, x: usize) -> u64 {
    (sprite_row as u64) << (56 - x)
}

// TODO: bounds checking on usize inputs
impl Screen {
    /// Initializes a new blank screen
    pub fn new() -> Screen {
        Screen { pixels: [0; 32] }
    }

    /// Unsets all pixels on the screen
    pub fn clear(&mut self) {
        for i in 0..32 {
            self.pixels[i] = 0;
        }
    }

    pub fn get_pixel(&self, x: usize, y: usize) -> bool {
        ((self.pixels[y] >> (63 - x)) & 1) == 1
    }

    /// Draw the provided sprite with the top left corner at (x, y).
    /// If the sprite would be clipped, it does not get drawn (TODO: wrap instead)
    pub fn draw_sprite(&mut self, x: usize, y: usize, sprite: &[u8]) -> bool {
        let sprite_height = sprite.len();
        let sprite_width = 8;

        if x + sprite_width > 64 || y + sprite_height > 32 {
            return false
        }

        let mut collision = false;
        for i in 0..sprite_height {
            // any bit being set means that we would be unsetting a set pixel
            let overwritten_bits = 
                get_old_byte(self.pixels[i + y], x, sprite_width) & sprite[i];
            collision = collision || overwritten_bits > 0;
            self.pixels[i + y] ^= get_mask(sprite[i], x);
        }

        collision
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn get_pixel() {
        let mut screen = Screen::new();
        screen.pixels[0] |= 1 << 63;
        assert!(screen.get_pixel(0, 0));

        screen.pixels[10] |= 1;
        assert!(screen.get_pixel(63, 10));
    }

    #[test]
    fn old_byte() {
        // rightmost byte
        assert_eq!(get_old_byte(17, 56, 8), 17);
        // leftmost byte
        assert_eq!(get_old_byte(17 << 56, 0, 8), 17);
        // somewhere in between
        assert_eq!(get_old_byte(255, 52, 8), 15);
    }

    #[test]
    fn mask() {
        // mask for lefmost byte
        assert_eq!(get_mask(1 << 7, 0), 1 << 63);
        // mask for rightmost byte
        assert_eq!(get_mask(5, 56), 5);
    }

    #[test]
    fn sprite_simple() {
        let mut screen = Screen::new();
        // draw a 8x2 rectangle in the top left corner
        assert!(!screen.draw_sprite(0, 0, &[255, 255]));
        assert_eq!(screen.pixels[0] >> 56, 255);
        assert_eq!(screen.pixels[1] >> 56, 255);
        assert_eq!(screen.pixels[2], 0);

        // erase the left half of the rectangle
        assert!(screen.draw_sprite(0, 0, &[0b11110000, 0b11110000]));
        assert_eq!(screen.pixels[0] >> 56, 15);
        assert_eq!(screen.pixels[1] >> 56, 15);
        assert_eq!(screen.pixels[2], 0);
    }

    #[test]
    fn sprite_oob() {
        let mut screen = Screen::new();
        assert!(!screen.draw_sprite(57, 3, &[1, 2, 3, 4]));
        for i in 3..7 {
            assert_eq!(screen.pixels[i], 0);
        }
        
        assert!(!screen.draw_sprite(5, 29, &[1, 2, 3, 4]));
        for i in 5..9 {
            assert_eq!(screen.pixels[i], 0);    
        }
    }

    #[test]
    fn sprite_y() {
        let mut screen = Screen::new();
        screen.draw_sprite(5, 10, &[1]);
    }
}
