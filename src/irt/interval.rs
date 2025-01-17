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
}
