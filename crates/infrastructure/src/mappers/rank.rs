use core::entities::RankEntity;
use crate::dtos::RankDto;
use super::mapper::Mapper;

pub struct RankMapper;

impl Mapper<RankEntity, RankDto> for RankMapper {
    fn to_dto(entity: &RankEntity) -> RankDto {
        match entity {
            RankEntity::None => RankDto::None,
            RankEntity::E => RankDto::E,
            RankEntity::D => RankDto::D,
            RankEntity::C => RankDto::C,
            RankEntity::B => RankDto::B,
            RankEntity::A => RankDto::A,
            RankEntity::Infinite => RankDto::Infi
        }
    }

    fn to_entity(dto: &RankDto) -> RankEntity {
        match dto {
            RankDto::None => RankEntity::None,
            RankDto::E => RankEntity::E,
            RankDto::D => RankEntity::D,
            RankDto::C => RankEntity::C,
            RankDto::B => RankEntity::B,
            RankDto::A => RankEntity::A,
            RankDto::Infi => RankEntity::Infinite
        }
    }
}