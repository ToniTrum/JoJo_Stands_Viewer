use serde::{Serialize, Deserialize};

/// A Data Transfer Object (DTO) representing a Stand's statistical rank during serialization and deserialization.
#[derive(Debug, Clone, Copy, Serialize)]
pub enum RankDto {
    None,
    E,
    D,
    C,
    B,
    A,
    Infi
}

impl From<&str> for RankDto {
    /// Maps a raw string slice representation of a rank into a matching `RankDto` variant.
    ///
    /// # Arguments
    ///
    /// * `s` - A raw string slice parsed from the data source.
    ///
    /// # Returns
    ///
    /// * The corresponding `RankDto` variant, defaulting to `RankDto::None` if the input string is unrecognized.
    fn from(s: &str) -> Self {
        match s {
            "None" => RankDto::None,
            "E" => RankDto::E,
            "D" => RankDto::D,
            "C" => RankDto::C,
            "B" => RankDto::B,
            "A" => RankDto::A,
            "Infi" => RankDto::Infi,
            _ => RankDto::None,
        }
    }
}

impl<'de> Deserialize<'de> for RankDto {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>
    {
        let s = String::deserialize(deserializer)?;
        Ok(RankDto::from(s.as_str()))
    }
}