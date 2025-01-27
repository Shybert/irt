use strum::EnumIter;

#[derive(EnumIter, Clone, Copy)]
pub enum Axis {
    X,
    Y,
    Z,
}
