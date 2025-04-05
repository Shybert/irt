pub fn degrees_to_radians(degrees: f32) -> f32 {
    return degrees * std::f32::consts::PI / 180.;
}

pub fn lerp(v0: f32, v1: f32, t: f32) -> f32 {
    return v0 + t * (v1 - v0);
}
