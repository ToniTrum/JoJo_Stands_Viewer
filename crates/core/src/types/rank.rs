/// Represents the statistical rank of a Stand's attribute in JoJo's Bizarre Adventure.
/// 
/// Used to classify capabilities such as Power, Speed, Range, Power Persistence, Precision, and Developmental Potential.
#[derive(Debug, Clone, Copy)]
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
    /// Converts the rank into a numeric value.
    ///
    /// # Returns
    ///
    /// * A `u8` integer mapping from 0 (None) to 6 (Infinite) representing the attribute level.
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

    /// Converts the rank into a `String`.
    ///
    /// # Returns
    ///
    /// * A heap-allocated `String` containing the official text name of the rank.
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