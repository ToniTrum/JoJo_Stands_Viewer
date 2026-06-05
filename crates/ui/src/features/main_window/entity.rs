use std::sync::Arc;

use core::services::StandService;
use core::models::StandModel;

pub struct MainWindowEntity {
    stand_service: Arc<StandService>,
    stands: Vec<StandModel>,
    selected_stand_name: Option<String>,
}

impl MainWindowEntity {
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
}