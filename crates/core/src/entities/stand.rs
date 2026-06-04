use super::Rank;

#[derive(Debug, Clone)]
pub struct StandEntity {
    name: String,
    power: Rank,
    speed: Rank,
    range: Rank,
    power_persistence: Rank,
    precision: Rank,
    development_potential: Rank
}

impl StandEntity {
    pub fn new(
        name: String,
        power: Rank,
        speed: Rank,
        range: Rank,
        power_persistence: Rank,
        precision: Rank,
        development_potential: Rank
    ) -> Self {
        StandEntity {
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