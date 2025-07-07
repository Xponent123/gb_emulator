#[derive(PartialEq, Copy, Clone)]
pub enum GbMode {
    Classic,
    ColorAsClassic,
    Color,
}

#[derive(PartialEq, Copy, Clone)]
pub enum GbSpeed {
    Single = 1,
    Double = 2,
}
