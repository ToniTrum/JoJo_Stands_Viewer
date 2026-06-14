use std::sync::Arc;
use std::path::Path;
use gpui::Global;

use core::repositories::StandRepository;
use core::services::StandService;
use infrastructure::file::PathManager;
use infrastructure::repositories::CsvStandRepository;

pub struct DependencyInjector {
    path_manager: Arc<PathManager>,
    stand_service: Arc<StandService>
}

impl Global for DependencyInjector {}

impl DependencyInjector {
    pub fn init(base_dir: impl AsRef<Path>) -> Result<Self, Box<dyn std::error::Error>> {
        let path_manager = Arc::new(PathManager::new(base_dir, "assets", "jojo-stands"));

        let csv_stand_repository = CsvStandRepository::new(&path_manager)?;
        let stand_repository: Arc<dyn StandRepository> = Arc::new(csv_stand_repository);
        let stand_service = Arc::new(StandService::new(stand_repository));

        Ok(Self {
            path_manager,
            stand_service
        })
    }

    pub fn path_manager(&self) -> Arc<PathManager> {
        self.path_manager.clone()
    }

    pub fn stand_service(&self) -> Arc<StandService> {
        self.stand_service.clone()
    }
}