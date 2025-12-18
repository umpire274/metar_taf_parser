#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Visibility {
    CAVOK,
    Single {
        prevailing: u16,
    },
    WithMinimum {
        prevailing: u16,
        minimum: u16,
        direction: VisibilityDirection,
    },
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum VisibilityDirection {
    N,
    NE,
    E,
    SE,
    S,
    SW,
    W,
    NW,
}
