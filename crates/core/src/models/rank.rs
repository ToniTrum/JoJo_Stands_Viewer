#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RankModel {
    None,
    E,
    D,
    C,
    B,
    A,
    Infinite,
}

impl RankModel {
    pub fn value(&self) -> u8 {
        match self {
            RankModel::None => 0,
            RankModel::E => 1,
            RankModel::D => 2,
            RankModel::C => 3,
            RankModel::B => 4,
            RankModel::A => 5,
            RankModel::Infinite => 6
        }
    }
}

