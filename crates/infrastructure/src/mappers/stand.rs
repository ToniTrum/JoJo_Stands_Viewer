use core::models::StandModel;
use crate::dtos::StandDto;
use super::mapper::Mapper;
use crate::mappers::RankMapper;

/// A stateless mapping utility responsible for converting between complex domain `StandModel` structures and flattened infrastructure `StandDto` data layers.
pub struct StandMapper;

impl Mapper<StandModel, StandDto> for StandMapper {
    /// Maps a domain `StandModel` reference into its infrastructure-specific serialized `StandDto` layout.
    ///
    /// # Arguments
    ///
    /// * `model` - A shared reference to the domain `StandModel`.
    ///
    /// # Returns
    ///
    /// * A fully initialized `StandDto` instance containing mapped attribute transfer tokens.
    fn to_dto(model: &StandModel) -> StandDto {
        StandDto::new(
            String::from(model.name()),
            RankMapper::to_dto(&model.power()),
            RankMapper::to_dto(&model.speed()),
            RankMapper::to_dto(&model.range()),
            RankMapper::to_dto(&model.power_persistence()),
            RankMapper::to_dto(&model.precision()),
            RankMapper::to_dto(&model.development_potential())
        )
    }

    /// Maps a raw infrastructure `StandDto` data transfer record back into a domain `StandModel`.
    ///
    /// # Arguments
    ///
    /// * `dto` - A shared reference to the deserialized external `StandDto` data block.
    ///
    /// # Returns
    ///
    /// * A core domain `StandModel` instance.
    fn to_model(dto: &StandDto) -> StandModel {
        StandModel::new(
            String::from(dto.stand()),
            RankMapper::to_model(&dto.power()),
            RankMapper::to_model(&dto.speed()),
            RankMapper::to_model(&dto.range()),
            RankMapper::to_model(&dto.power_persistence()),
            RankMapper::to_model(&dto.precision()),
            RankMapper::to_model(&dto.development_potential())
        )
    }
}