use crate::models::StandModel;

pub trait StandRepository: Send + Sync {
    fn get_all(&self) -> Vec<StandModel>;
    fn get_by_name(&self, name: &str) -> Option<StandModel>;
}