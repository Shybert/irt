#[derive(Clone, Copy)]
pub enum Axis {
    X,
    Y,
    Z,
}
impl Axis {
    pub fn iter() -> impl Iterator<Item = Axis> {
        return [Axis::X, Axis::Y, Axis::Z].into_iter();
    }
}
