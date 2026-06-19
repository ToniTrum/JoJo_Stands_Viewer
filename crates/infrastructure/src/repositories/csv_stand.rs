use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use csv::Reader;

use core::models::StandModel;
use core::repositories::StandRepository;
use crate::dtos::StandDto;
use crate::mappers::{Mapper,StandMapper};
use crate::file::PathManager;

pub struct CsvStandRepository {
    items: Vec<StandModel>,
}

/// An implementation of `StandRepository` that loads and caches data from a flat CSV file.
impl CsvStandRepository {
    /// Initializes the repository by parsing records from a CSV file.
    ///
    /// # Arguments
    ///
    /// * `path_manager` - A shared reference to the utility managing environment file paths.
    ///
    /// # Returns
    ///
    /// * A `Result` containing the initialized `CsvStandRepository` state on success, or a dynamic wrapper of any IO/Parsing error.
    pub fn new(path_manager: &PathManager) -> Result<Self, Box<dyn Error>> {
        let mut items = Vec::new();

        let path = path_manager.csv_path();
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let mut rdr = Reader::from_reader(reader);

        for result in rdr.deserialize() {
            let record: StandDto = result?;
            items.push(StandMapper::to_model(&record));
        }

        Ok(Self { items })
    }
}

impl StandRepository for CsvStandRepository {
    /// Returns a cloned snapshot collection of all cached domain models.
    ///
    /// # Returns
    ///
    /// * A `Vec<StandModel>` populated with copies of the Stand entities.
    fn get_all(&self) -> Vec<StandModel> {
        self.items.clone()
    }

    /// Searches for a specific Stand by its unique string name in the local cached vector.
    ///
    /// # Arguments
    ///
    /// * `name` - A string slice referencing the Stand's unique lookup key.
    ///
    /// # Returns
    ///
    /// * An `Option<StandModel>` containing a cloned model instance if matched, or `None`.
    fn get_by_name(&self, name: &str) -> Option<StandModel> {
        self.items.iter().find(|s| s.name() == name).cloned()
    }
}