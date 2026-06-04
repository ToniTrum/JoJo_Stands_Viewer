use core::entities::StandEntity;
use crate::dtos::StandDto;
use super::mapper::Mapper;
use crate::mappers::RankMapper;

pub struct StandMapper;

impl Mapper<StandEntity, StandDto> for StandMapper {
    fn to_dto(entity: &StandEntity) -> StandDto {
        StandDto::new(
            String::from(entity.name()),
            RankMapper::to_dto(&entity.power()),
            RankMapper::to_dto(&entity.speed()),
            RankMapper::to_dto(&entity.range()),
            RankMapper::to_dto(&entity.power_persistence()),
            RankMapper::to_dto(&entity.precision()),
            RankMapper::to_dto(&entity.development_potential())
        )
    }

    fn to_entity(dto: &StandDto) -> StandEntity {
        StandEntity::new(
            String::from(dto.name()),
            RankMapper::to_entity(&dto.power()),
            RankMapper::to_entity(&dto.speed()),
            RankMapper::to_entity(&dto.range()),
            RankMapper::to_entity(&dto.power_persistence()),
            RankMapper::to_entity(&dto.precision()),
            RankMapper::to_entity(&dto.development_potential())
        )
    }
}