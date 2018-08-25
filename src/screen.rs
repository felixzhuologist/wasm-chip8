pub struct Screen {

}

impl Screen {
    pub fn new() -> Screen {
        Screen {}
    }

    pub fn clear(&self) -> () {
    }

    pub fn draw_sprite(&self, x: usize, y: usize, sprite: &[u8]) -> bool {
        // TODO: check bounds of x and y
        true
    }
}
