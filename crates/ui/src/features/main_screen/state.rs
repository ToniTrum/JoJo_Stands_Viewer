use std::sync::Arc;

use core::services::StandService;
use core::models::StandModel;

/// Represents the presentation and interactive state container for the main screen.
pub struct MainScreenState {
    stand_service: Arc<StandService>,
    stands: Vec<StandModel>,
    selected_stand_name: Option<String>,
}

impl MainScreenState {
    /// Constructs a new `MainScreenState`, instantly pre-fetching the available record sets.
    ///
    /// # Arguments
    ///
    /// * `stand_service` - A thread-safe shared reference pointer to the core `StandService` instance.
    ///
    /// # Returns
    ///
    /// * An initialized state machine payload with no initial selection targeted.
    pub fn new(stand_service: Arc<StandService>) -> Self {
        let stands = stand_service.get_all();

        Self {
            stand_service,
            stands,
            selected_stand_name: None
        }
    }

    /// Provides a slice reference of all pre-cached `StandModel` entities.
    ///
    /// # Returns
    ///
    /// * A slice sequence containing compiled model payloads.
    pub fn stands(&self) -> &[StandModel] {
        &self.stands
    }

    /// Exposes a shared reference to the unique identifier string of the currently active selection.
    ///
    /// # Returns
    ///
    /// * An `Option` reference wrapping the selected target item name key.
    pub fn selected_stand_name(&self) -> &Option<String> {
        &self.selected_stand_name
    }

    /// Resolves and fetches the full model payload corresponding to the active user selection state identifier.
    ///
    /// It queries the business service layer dynamically based on the stored text key index.
    ///
    /// # Returns
    ///
    /// * `Some(StandModel)` if an identity key is selected and verified by the repository backend, or `None`.
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

    /// Mutates the selection index parameters to target a new focused record entry point.
    ///
    /// # Arguments
    ///
    /// * `name` - The concrete string name identity token representing the newly focused model item.
    pub fn select_stand(&mut self, name: String) {
        self.selected_stand_name = Some(name);
    }
}