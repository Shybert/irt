pub fn degrees_to_radians(degrees: f32) -> f32 {
    return degrees * std::f32::consts::PI / 180.;
}

pub fn lerp(v0: f32, v1: f32, t: f32) -> f32 {
    return v0 + t * (v1 - v0);
}

/// Constant used to accommodate for floating-point rounding error.
pub const EPSILON: f32 = 0.00001;
pub fn approx_equals(a: f32, b: f32) -> bool {
    return (a - b).abs() < EPSILON;
}
