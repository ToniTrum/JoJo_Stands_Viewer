use core::models::StandModel;
use crate::dtos::StandDto;
use super::mapper::Mapper;
use crate::mappers::RankMapper;

pub struct StandMapper;

impl Mapper<StandModel, StandDto> for StandMapper {
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