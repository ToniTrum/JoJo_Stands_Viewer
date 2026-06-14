use super::RankModel;

#[derive(Debug, Clone)]
pub struct StandModel {
    name: String,
    power: RankModel,
    speed: RankModel,
    range: RankModel,
    power_persistence: RankModel,
    precision: RankModel,
    development_potential: RankModel
}

impl StandModel {
    pub fn new(
        name: String,
        power: RankModel,
        speed: RankModel,
        range: RankModel,
        power_persistence: RankModel,
        precision: RankModel,
        development_potential: RankModel
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

    pub fn power(&self) -> RankModel {
        self.power
    }

    pub fn speed(&self) -> RankModel {
        self.speed
    }

    pub fn range(&self) -> RankModel {
        self.range
    }

    pub fn power_persistence(&self) -> RankModel {
        self.power_persistence
    }

    pub fn precision(&self) -> RankModel {
        self.precision
    }

    pub fn development_potential(&self) -> RankModel {
        self.development_potential
    }
}

impl Default for StandModel {
    fn default() -> Self {
        StandModel {
            name: String::from("No selected"),
            power: RankModel::None,
            speed: RankModel::None,
            range: RankModel::None,
            power_persistence: RankModel::None,
            precision: RankModel::None,
            development_potential: RankModel::None
        }
    }
}