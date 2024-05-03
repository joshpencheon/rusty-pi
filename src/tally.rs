use std::fmt;

pub struct Tally {
    hits: usize,
    total: usize
}

impl Tally {
    pub fn new() -> Self {
        Self { hits: 0, total: 0 }
    }

    pub fn count(&mut self, hit: bool) -> () {
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

impl std::ops::AddAssign for Tally {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            total: self.total + other.total,
            hits: self.hits + other.hits,
        }
    }
}

impl fmt::Display for Tally {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let pi = 4.0 * self.hit_rate();
        write!(f, "with {} hits tallied after {} iterations, π ≈ {}", self.hits, self.total, pi)
    }
}

