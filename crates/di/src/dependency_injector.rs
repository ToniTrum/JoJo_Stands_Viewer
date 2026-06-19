use std::sync::Arc;
use std::path::Path;
use gpui::Global;

use core::repositories::StandRepository;
use core::services::StandService;
use infrastructure::file::PathManager;
use infrastructure::repositories::CsvStandRepository;

/// A centralized dependency injection container that manages the application's global state lifetimes.
pub struct DependencyInjector {
    path_manager: Arc<PathManager>,
    stand_service: Arc<StandService>,
}

impl Global for DependencyInjector {}

impl DependencyInjector {
    /// Initializes the entire application dependency graph by assembling infrastructure and core service layers.
    ///
    /// # Arguments
    ///
    /// * `base_dir` - An object implementing `AsRef<Path>` that points to the application's root execution directory.
    ///
    /// # Returns
    ///
    /// * A `Result` containing the fully wired `DependencyInjector` container on success, or an IO/parsing startup error wrapped in a `Box`.
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

    /// Provides a thread-safe, reference-counted clone of the `PathManager` service.
    ///
    /// # Returns
    ///
    /// * An `Arc<PathManager>` pointer managing asset paths.
    pub fn path_manager(&self) -> Arc<PathManager> {
        self.path_manager.clone()
    }

    /// Provides a thread-safe, reference-counted clone of the `StandService`.
    ///
    /// # Returns
    ///
    /// * An `Arc<StandService>` pointer used for performing application operations.
    pub fn stand_service(&self) -> Arc<StandService> {
        self.stand_service.clone()
    }
}