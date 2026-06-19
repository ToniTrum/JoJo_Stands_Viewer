use crate::types::Rank;

/// Represents a Stand with its name and statistical attributes.
#[derive(Debug, Clone)]
pub struct StandModel {
    name: String,
    power: Rank,
    speed: Rank,
    range: Rank,
    power_persistence: Rank,
    precision: Rank,
    development_potential: Rank
}

impl StandModel {
    /// Creates a new instance of a `StandModel` with the specified name and attributes.
    ///
    /// # Arguments
    ///
    /// * `name` - The unique name of the Stand.
    /// * `power` - The Power rank.
    /// * `speed` - The Speed rank.
    /// * `range` - The Range rank.
    /// * `power_persistence` - The Power Persistence rank.
    /// * `precision` - The Precision rank.
    /// * `development_potential` - The Developmental Potential rank.
    ///
    /// # Returns
    ///
    /// * An initialized `StandModel` instance containing the provided attributes.
    pub fn new(
        name: String,
        power: Rank,
        speed: Rank,
        range: Rank,
        power_persistence: Rank,
        precision: Rank,
        development_potential: Rank
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

    /// Returns a string slice referencing the Stand's name.
    ///
    /// # Returns
    ///
    /// * A string slice (`&str`) referencing the heap-allocated name of the Stand.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the Stand's Power rank.
    ///
    /// # Returns
    ///
    /// * The `Rank` enum representing the Stand's power level.
    pub fn power(&self) -> Rank {
        self.power
    }

    /// Returns the Stand's Speed rank.
    ///
    /// # Returns
    ///
    /// * The `Rank` enum representing the Stand's speed level.
    pub fn speed(&self) -> Rank {
        self.speed
    }

    /// Returns the Stand's Range rank.
    ///
    /// # Returns
    ///
    /// * The `Rank` enum representing the Stand's maximum operational distance.
    pub fn range(&self) -> Rank {
        self.range
    }

    /// Returns the Stand's Power Persistence rank.
    ///
    /// # Returns
    ///
    /// * The `Rank` enum representing the Stand's ability to maintain its state over time.
    pub fn power_persistence(&self) -> Rank {
        self.power_persistence
    }

    /// Returns the Stand's Precision rank.
    ///
    /// # Returns
    ///
    /// * The `Rank` enum representing the Stand's accuracy and combat control.
    pub fn precision(&self) -> Rank {
        self.precision
    }

    /// Returns the Stand's Developmental Potential rank.
    ///
    /// # Returns
    ///
    /// * The `Rank` enum representing the Stand's capacity to evolve or manifest new skills.
    pub fn development_potential(&self) -> Rank {
        self.development_potential
    }
}

impl Default for StandModel {
    /// Creates a default placeholder state when no Stand is currently selected.
    ///
    /// # Returns
    ///
    /// * A `StandModel` instance with the name "No selected" and all attributes set to `Rank::None`.
    fn default() -> Self {
        StandModel {
            name: String::from("No selected"),
            power: Rank::None,
            speed: Rank::None,
            range: Rank::None,
            power_persistence: Rank::None,
            precision: Rank::None,
            development_potential: Rank::None
        }
    }
}