use std::fmt::{Display, Formatter};
use crate::Coords;

pub type NodeIndex = i64;

#[derive(Clone)]
pub struct Node {
    pub coords: Coords,
    pub offset: NodeIndex,
}

impl Display for Node {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}; {}", self.coords, self.offset)
    }
}

impl Node {
    pub fn new(coords: Coords, offset: i64) -> Self {
        Self { coords, offset }
    }
    pub fn default() -> Self {
        Self { coords: Coords::default() , offset: NodeIndex::MAX }
    }
}
