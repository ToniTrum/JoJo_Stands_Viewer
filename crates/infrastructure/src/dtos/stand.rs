use serde::{Serialize, Deserialize};

use super::RankDto;


/// A Data Transfer Object (DTO) used for flat-file deserialization and serialization of Stand records.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StandDto {
    #[serde(rename = "Stand")]
    stand: String,
    #[serde(rename = "PWR")]
    pwr: RankDto,
    #[serde(rename = "SPD")]
    spd: RankDto,
    #[serde(rename = "RNG")]
    rng: RankDto,
    #[serde(rename = "PER")]
    per: RankDto,
    #[serde(rename = "PRC")]
    prc: RankDto,
    #[serde(rename = "DEV")]
    dev: RankDto
}

impl StandDto {
    /// Creates a new instance of a `StandDto` with explicit raw transfer components.
    ///
    /// # Arguments
    ///
    /// * `stand` - The raw name string of the Stand.
    /// * `pwr` - The mapped raw power rank token.
    /// * `spd` - The mapped raw speed rank token.
    /// * `rng` - The mapped raw range rank token.
    /// * `per` - The mapped raw persistence rank token.
    /// * `prc` - The mapped raw precision rank token.
    /// * `dev` - The mapped raw development potential rank token.
    ///
    /// # Returns
    ///
    /// * An initialized `StandDto` transfer state payload container.
    pub fn new(
        stand: String,
        pwr: RankDto,
        spd: RankDto,
        rng: RankDto,
        per: RankDto,
        prc: RankDto,
        dev: RankDto
    ) -> Self {
        StandDto {
            stand,
            pwr,
            spd,
            rng,
            per,
            prc,
            dev
        }
    }

    /// Accesses the raw name of the Stand.
    ///
    /// # Returns
    ///
    /// * A string slice referencing the internal name identifier.
    pub fn stand(&self) -> &str {
        &self.stand
    }

    /// Accesses the raw short power rank data token.
    ///
    /// # Returns
    ///
    /// * The `RankDto` enumeration token value for power.
    pub fn power(&self) -> RankDto {
        self.pwr
    }

    /// Accesses the raw short speed rank data token.
    ///
    /// # Returns
    ///
    /// * The `RankDto` enumeration token value for speed.
    pub fn speed(&self) -> RankDto {
        self.spd
    }

    /// Accesses the raw short range rank data token.
    ///
    /// # Returns
    ///
    /// * The `RankDto` enumeration token value for range.
    pub fn range(&self) -> RankDto {
        self.rng
    }

    /// Accesses the raw short power persistence rank data token.
    ///
    /// # Returns
    ///
    /// * The `RankDto` enumeration token value for power persistence.
    pub fn power_persistence(&self) -> RankDto {
        self.per
    }

    /// Accesses the raw short precision rank data token.
    ///
    /// # Returns
    ///
    /// * The `RankDto` enumeration token value for precision.
    pub fn precision(&self) -> RankDto {
        self.prc
    }

    /// Accesses the raw short developmental potential rank data token.
    ///
    /// # Returns
    ///
    /// * The `RankDto` enumeration token value for developmental potential.
    pub fn development_potential(&self) -> RankDto {
        self.dev
    }
}