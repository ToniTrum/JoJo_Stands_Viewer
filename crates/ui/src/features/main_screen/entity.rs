use std::sync::Arc;

use core::services::StandService;
use core::models::StandModel;

pub struct MainScreenEntity {
    stand_service: Arc<StandService>,
    stands: Vec<StandModel>,
    selected_stand_name: Option<String>,
}

impl MainScreenEntity {
    pub fn new(stand_service: Arc<StandService>) -> Self {
        let stands = stand_service.get_all();

        Self {
            stand_service,
            stands,
            selected_stand_name: None
        }
    }

    pub fn stands(&self) -> &[StandModel] {
        &self.stands
    }

    pub fn selected_stand_name(&self) -> &Option<String> {
        &self.selected_stand_name
    }

    pub fn selected_stand(&self) -> Option<StandModel> {
        let selected_name = self.selected_stand_name.clone();
        match selected_name {
            None => None,
            Some(name) => match self.stand_service.get_by_name(&name.as_str()) {
                None => None,
                Some(stand) => Some(stand)
            }
        }
    }

    pub fn select_stand(&mut self, name: String) {
        self.selected_stand_name = Some(name);
    }
}