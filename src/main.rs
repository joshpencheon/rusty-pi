struct Point {
    x: f32,
    y: f32
}

impl Point {
    fn new() -> Point {
        Point { x: rand::random(), y: rand::random() }
    }

    fn size(&self) -> f32 {
        (self.x.powf(2.0) + self.y.powf(2.0)).powf(0.5)
    }

    fn within_unit_circle(&self) -> bool {
        self.size() <= 1.0
    }
}

struct Tally {
    hits: usize,
    total: usize
}

impl Tally {
    fn new() -> Tally {
        Tally { hits: 0, total: 0 }
    }

    fn count(&mut self, hit: bool) -> () {
        if hit { self.hits += 1 }
        self.total += 1;
    }

    // fn misses(&self) -> usize {
    //     self.total - self.hits
    // }

    fn hit_rate(&self) -> f32 {
        self.hits as f32 / self.total as f32
    }
}

fn main() {
    let mut tally = Tally::new();

    for _ in 0..1000000 {
        let hit = Point::new().within_unit_circle();
        tally.count(hit);
    }

    let pi = 4.0 * tally.hit_rate();
    println!("after {} iterations, estimate of π: {}", tally.total, pi)
}
