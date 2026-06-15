#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Rank {
    None,
    E,
    D,
    C,
    B,
    A,
    Infinite,
}

impl Rank {
    pub fn to_u8(&self) -> u8 {
        match self {
            Rank::None => 0,
            Rank::E => 1,
            Rank::D => 2,
            Rank::C => 3,
            Rank::B => 4,
            Rank::A => 5,
            Rank::Infinite => 6
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Rank::None => String::from("None"),
            Rank::E => String::from("E"),
            Rank::D => String::from("D"),
            Rank::C => String::from("C"),
            Rank::B => String::from("B"),
            Rank::A => String::from("A"),
            Rank::Infinite => String::from("Infinite")
        }
    }
}

