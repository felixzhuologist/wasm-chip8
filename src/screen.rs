/// 64 x 32 monochrome display. (0, 0) is the top left pixel
/// and (63, 31) is the bottom right pixel
pub struct Screen {
    pixels: [u64; 32],
}

fn get_mask(sprite_row: u8, x: usize) -> u64 {
    let sprite_row = sprite_row as u64;
    if x > 56 {
        sprite_row.rotate_right((x - 56) as u32)
    } else {
        sprite_row << (56 - x)
    }
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
        // TODO: return Result?
        if x > 63 || y > 31 {
            return false
        }

        let mut collision = false;
        for i in 0..sprite.len() {
            let row = (y + i) % 32;
            let sprite_mask = get_mask(sprite[i], x);
            let matched_bits = self.pixels[row] & sprite_mask;
            collision = collision || matched_bits > 0;
            self.pixels[row] ^= sprite_mask;
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
    fn mask() {
        // mask for lefmost byte
        assert_eq!(get_mask(1 << 7, 0), 1 << 63);
        // mask for rightmost byte
        assert_eq!(get_mask(5, 56), 5);
        // wrapping mask
        assert_eq!(get_mask(0b00011000, 60), ((1 << 63) + 1));
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
    fn sprite_wrap_y() {
        let mut screen = Screen::new();
        // should wrap so that bottom 2 and top 2 rows are written to
        assert!(!screen.draw_sprite(0, 30, &[255, 255, 255, 255]));
        assert_eq!(screen.pixels[0] >> 56, 255);
        assert_eq!(screen.pixels[1] >> 56, 255);
        assert_eq!(screen.pixels[30] >> 56, 255);
        assert_eq!(screen.pixels[31] >> 56, 255);
    }

    #[test]
    fn sprite_wrap_x() {
        let mut screen = Screen::new();
        assert!(!screen.draw_sprite(60, 0, &[0b00111100]));
        // should wrap so that left 2 and right 2 columns are written to
        let expected_row = (1 << 63) + (1 << 62) + 3;
        assert_eq!(screen.pixels[0], expected_row);
    }

    #[test]
    fn sprite_y() {
        let mut screen = Screen::new();
        // test that the y index and sprite index are different
        screen.draw_sprite(5, 10, &[1]);
    }
}
