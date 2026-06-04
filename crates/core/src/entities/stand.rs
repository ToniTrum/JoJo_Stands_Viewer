use super::RankEntity;

#[derive(Debug, Clone)]
pub struct StandEntity {
    name: String,
    power: RankEntity,
    speed: RankEntity,
    range: RankEntity,
    power_persistence: RankEntity,
    precision: RankEntity,
    development_potential: RankEntity
}

impl StandEntity {
    pub fn new(
        name: String,
        power: RankEntity,
        speed: RankEntity,
        range: RankEntity,
        power_persistence: RankEntity,
        precision: RankEntity,
        development_potential: RankEntity
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

    pub fn power(&self) -> RankEntity {
        self.power
    }

    pub fn speed(&self) -> RankEntity {
        self.speed
    }

    pub fn range(&self) -> RankEntity {
        self.range
    }

    pub fn power_persistence(&self) -> RankEntity {
        self.power_persistence
    }

    pub fn precision(&self) -> RankEntity {
        self.precision
    }

    pub fn development_potential(&self) -> RankEntity {
        self.development_potential
    }
}