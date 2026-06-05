use core::models::RankModel;
use crate::dtos::RankDto;
use super::mapper::Mapper;

pub struct RankMapper;

impl Mapper<RankModel, RankDto> for RankMapper {
    fn to_dto(model: &RankModel) -> RankDto {
        match model {
            RankModel::None => RankDto::None,
            RankModel::E => RankDto::E,
            RankModel::D => RankDto::D,
            RankModel::C => RankDto::C,
            RankModel::B => RankDto::B,
            RankModel::A => RankDto::A,
            RankModel::Infinite => RankDto::Infi
        }
    }

    fn to_model(dto: &RankDto) -> RankModel {
        match dto {
            RankDto::None => RankModel::None,
            RankDto::E => RankModel::E,
            RankDto::D => RankModel::D,
            RankDto::C => RankModel::C,
            RankDto::B => RankModel::B,
            RankDto::A => RankModel::A,
            RankDto::Infi => RankModel::Infinite
        }
    }
}