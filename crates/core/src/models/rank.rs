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
    pub fn to_u8(&self) -> u8 {
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

    pub fn to_string(&self) -> String {
        match self {
            RankModel::None => String::from("None"),
            RankModel::E => String::from("E"),
            RankModel::D => String::from("D"),
            RankModel::C => String::from("C"),
            RankModel::B => String::from("B"),
            RankModel::A => String::from("A"),
            RankModel::Infinite => String::from("Infinite")
        }
    }
}

