use std::sync::Arc;

use crate::repositories::StandRepository;
use crate::models::StandModel;

/// A service layer structural component providing high-level business logic operations for Stands.
pub struct StandService {
    stand_repository: Arc<dyn StandRepository>,
}

impl StandService {
    /// Creates a new instance of a `StandService` injected with a thread-safe repository implementation.
    ///
    /// # Arguments
    ///
    /// * `stand_repository` - An atomically reference-counted pointer (`Arc`) wrapping a dynamic `StandRepository` trait object.
    ///
    /// # Returns
    ///
    /// * An initialized `StandService` instance.
    pub fn new(stand_repository: Arc<dyn StandRepository>) -> Self {
        Self { 
            stand_repository 
        }
    }

    /// Fetches a complete collection of all stored Stand models via the underlying repository.
    ///
    /// # Returns
    ///
    /// * A `Vec<StandModel>` containing all available Stands.
    pub fn get_all(&self) -> Vec<StandModel> {
        self.stand_repository.get_all()
    }

    /// Fetches a specific Stand model by its unique name identifier via the underlying repository.
    ///
    /// # Arguments
    ///
    /// * `name` - A string slice referencing the unique name of the Stand.
    ///
    /// # Returns
    ///
    /// * An `Option<StandModel>` containing `Some(StandModel)` if found, or `None` if no match exists.
    pub fn get_by_name(&self, name: &str) -> Option<StandModel> {
        self.stand_repository.get_by_name(name)
    }
}