use std::fmt::{Display, Formatter};
use crate::node::NodeIndex;

pub type EdgeCost = i64;

pub struct EdgePath {
    pub path: Vec<Edge>
}

impl EdgePath {
    pub fn distance(&self) {

        //self.path.iter().map(|edge| edge.cost).collect()
    }
}


#[derive(Clone)]
pub struct Edge {
    pub src: NodeIndex,
    pub trg: NodeIndex,
    pub cost: EdgeCost,
}

impl Display for Edge {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} -> {}, cost: {}", self.src, self.trg, self.cost)
    }
}

impl Edge {
    pub fn new(src: NodeIndex, trg: NodeIndex, cost: EdgeCost) -> Self {
        Self { src, trg, cost }
    }

    pub fn default() -> Self {
        Self { src: 0, trg: 0, cost: 0 }
    }
}