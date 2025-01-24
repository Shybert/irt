#[derive(Debug, Clone, Copy)]
pub struct Interval {
    pub min: f32,
    pub max: f32,
}
impl Interval {
    pub fn new(min: f32, max: f32) -> Self {
        return Self { min, max };
    }

    pub fn empty() -> Self {
        return Self::new(f32::INFINITY, f32::NEG_INFINITY);
    }

    pub fn from_intervals(a: &Self, b: &Self) -> Self {
        return Self::new(a.min.min(b.min), a.max.max(b.max));
    }

    pub fn size(&self) -> f32 {
        return self.max - self.min;
    }

    pub fn surrounds(&self, x: f32) -> bool {
        return self.min < x && x < self.max;
    }

    pub fn clamp(&self, x: f32) -> f32 {
        if x < self.min {
            return self.min;
        } else if x > self.max {
            return self.max;
        }
        return x;
    }

    pub fn expand(&self, delta: f32) -> Self {
        let padding = delta / 2.;

        return Interval::new(self.min - padding, self.max + padding);
    }
}
