pub struct Point {
    x: f32,
    y: f32
}

impl Point {
    pub fn new() -> Point {
        Point { x: rand::random(), y: rand::random() }
    }

    fn size(&self) -> f32 {
        (self.x.powf(2.0) + self.y.powf(2.0)).powf(0.5)
    }

    pub fn within_unit_circle(&self) -> bool {
        self.size() <= 1.0
    }
}
