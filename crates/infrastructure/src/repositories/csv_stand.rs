use std::error::Error;
use std::fs::File;
use std::io::BufReader;

use core::repositories::stand::StandRepository;
use core::models::stand::StandModel;
use crate::dtos::StandDto;
use crate::file::PathManager;
use crate::mappers::StandMapper;
use crate::mappers::mapper::Mapper;

pub struct CsvStandRepository {
    items: Vec<StandModel>,
}

impl CsvStandRepository {
    pub fn new(path_manager: &PathManager) -> Result<Self, Box<dyn Error>> {
        let mut items = Vec::new();

        let path = path_manager.csv_path();
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let mut rdr = csv::Reader::from_reader(reader);

        for result in rdr.deserialize() {
            let record: StandDto = result?;
            items.push(StandMapper::to_model(&record));
        }

        Ok(Self { items })
    }
}

impl StandRepository for CsvStandRepository {
    fn get_all(&self) -> Vec<StandModel> {
        self.items.clone()
    }

    fn get_by_name(&self, name: &str) -> Option<StandModel> {
        self.items.iter().find(|s| s.name() == name).cloned()
    }
}