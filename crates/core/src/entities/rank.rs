#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RankEntity {
    None,
    E,
    D,
    C,
    B,
    A,
    Infinite,
}

impl RankEntity {
    pub fn value(&self) -> u8 {
        match self {
            RankEntity::None => 0,
            RankEntity::E => 1,
            RankEntity::D => 2,
            RankEntity::C => 3,
            RankEntity::B => 4,
            RankEntity::A => 5,
            RankEntity::Infinite => 6
        }
    }
}

