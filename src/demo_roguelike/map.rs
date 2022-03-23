pub struct Map {
    width: i32,
    height: i32,
}

impl Map {
    pub fn new(width: i32, height: i32) -> Self {
        Map::new_with_default(width, height)
    }

    pub fn new_with_default(width: i32, height: i32) -> Self {
        assert!(width > 0, "width must be greater than 0!");
        assert!(height > 0, "height must be greater than 0!");
        Map { width, height }
    }

    pub fn width(&self) -> i32 {
        self.width
    }

    pub fn height(&self) -> i32 {
        self.height
    }
}
