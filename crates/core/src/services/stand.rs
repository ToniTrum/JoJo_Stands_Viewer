use std::sync::Arc;

use crate::repositories::StandRepository;
use crate::entities::StandEntity;

pub struct StandService {
    stand_repository: Arc<dyn StandRepository>,
}

impl StandService {
    pub fn new(stand_repository: Arc<dyn StandRepository>) -> Self {
        Self { 
            stand_repository 
        }
    }

    pub fn get_all(&self) -> Vec<StandEntity> {
        self.stand_repository.get_all()
    }

    pub fn get_by_name(&self, name: &str) -> Option<StandEntity> {
        self.stand_repository.get_by_name(name)
    }
}