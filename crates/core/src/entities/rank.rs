use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Rank {
    None,
    E,
    D,
    C,
    B,
    A,
    Infinite,
}
