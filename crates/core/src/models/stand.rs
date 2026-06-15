use crate::types::Rank;

#[derive(Debug, Clone)]
pub struct StandModel {
    name: String,
    power: Rank,
    speed: Rank,
    range: Rank,
    power_persistence: Rank,
    precision: Rank,
    development_potential: Rank
}

impl StandModel {
    pub fn new(
        name: String,
        power: Rank,
        speed: Rank,
        range: Rank,
        power_persistence: Rank,
        precision: Rank,
        development_potential: Rank
    ) -> Self {
        StandModel {
            name,
            power,
            speed,
            range,
            power_persistence,
            precision,
            development_potential
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn power(&self) -> Rank {
        self.power
    }

    pub fn speed(&self) -> Rank {
        self.speed
    }

    pub fn range(&self) -> Rank {
        self.range
    }

    pub fn power_persistence(&self) -> Rank {
        self.power_persistence
    }

    pub fn precision(&self) -> Rank {
        self.precision
    }

    pub fn development_potential(&self) -> Rank {
        self.development_potential
    }
}

impl Default for StandModel {
    fn default() -> Self {
        StandModel {
            name: String::from("No selected"),
            power: Rank::None,
            speed: Rank::None,
            range: Rank::None,
            power_persistence: Rank::None,
            precision: Rank::None,
            development_potential: Rank::None
        }
    }
}