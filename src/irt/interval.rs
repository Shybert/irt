pub struct Interval {
    pub min: f32,
    pub max: f32,
}
impl Interval {
    pub fn new(min: f32, max: f32) -> Self {
        return Self { min, max };
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
}
