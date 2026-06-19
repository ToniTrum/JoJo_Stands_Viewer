use crate::models::StandModel;

/// A thread-safe repository trait defining data access operations for Stands.
pub trait StandRepository: Send + Sync {
    /// Retrieves all available Stands from the storage source.
    ///
    /// # Returns
    ///
    /// * A `Vec<StandModel>` containing a collection of all stored Stand models.
    fn get_all(&self) -> Vec<StandModel>;

    /// Retrieves a specific Stand model by its unique name identifier.
    ///
    /// # Arguments
    ///
    /// * `name` - A string slice referencing the unique name of the Stand to search for.
    ///
    /// # Returns
    ///
    /// * An `Option<StandModel>` containing `Some(StandModel)` if a match is found, or `None` if it does not exist.
    fn get_by_name(&self, name: &str) -> Option<StandModel>;
}