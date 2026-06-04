use serde::{Serialize, Deserialize};

use super::RankDto;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StandDto {
    #[serde(rename = "Name")]
    name: String,
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
    pub fn new(
        name: String,
        pwr: RankDto,
        spd: RankDto,
        rng: RankDto,
        per: RankDto,
        prc: RankDto,
        dev: RankDto
    ) -> Self {
        StandDto {
            name,
            pwr,
            spd,
            rng,
            per,
            prc,
            dev
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn power(&self) -> RankDto {
        self.pwr
    }

    pub fn speed(&self) -> RankDto {
        self.spd
    }

    pub fn range(&self) -> RankDto {
        self.rng
    }

    pub fn power_persistence(&self) -> RankDto {
        self.per
    }

    pub fn precision(&self) -> RankDto {
        self.prc
    }

    pub fn development_potential(&self) -> RankDto {
        self.dev
    }
}