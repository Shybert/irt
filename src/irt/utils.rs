pub fn lerp(v0: f32, v1: f32, t: f32) -> f32 {
    return v0 + t * (v1 - v0);
}

/// Constant used to accommodate for floating-point rounding error.
pub const EPSILON: f32 = 0.00001;
pub fn approx_equals(a: f32, b: f32) -> bool {
    return (a - b).abs() < EPSILON;
}

#[derive(Debug, Copy, Clone)]
pub struct Degrees(pub f32);
impl Degrees {
    pub fn as_f32(self) -> f32 {
        return self.0;
    }

    pub fn to_radians(self) -> Radians {
        return Radians(self.0.to_radians());
    }
}
#[derive(Debug, Copy, Clone)]
pub struct Radians(f32);
impl Radians {
    pub fn as_f32(self) -> f32 {
        return self.0;
    }

    pub fn to_degrees(self) -> Degrees {
        return Degrees(self.0.to_degrees());
    }
}
