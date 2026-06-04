use std::sync::Arc;
use std::path::Path;

use core::repositories::StandRepository;
use core::services::stand::StandService;
use infrastructure::file::PathManager;
use infrastructure::repositories::CsvStandRepository;

pub struct AppContainer {
    pub stand_service: Arc<StandService>,
}

impl AppContainer {
    pub fn new(base_dir: impl AsRef<Path>) -> Result<Self, Box<dyn std::error::Error>> {
        let path_manager = PathManager::new(base_dir, "assets", "stands");

        let csv_stand_repository = CsvStandRepository::new(&path_manager)?;
        let stand_repository: Arc<dyn StandRepository> = Arc::new(csv_stand_repository);

        let stand_service = Arc::new(StandService::new(stand_repository));

        Ok(Self { stand_service })
    }
}