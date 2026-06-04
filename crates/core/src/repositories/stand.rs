use crate::entities::StandEntity;

pub trait StandRepository: Send + Sync {
    fn get_all(&self) -> Vec<StandEntity>;
    fn get_by_name(&self, name: &str) -> Option<StandEntity>;
}