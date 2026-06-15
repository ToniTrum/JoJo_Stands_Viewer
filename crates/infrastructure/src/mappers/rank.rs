use core::types::Rank;
use crate::dtos::RankDto;
use super::mapper::Mapper;

pub struct RankMapper;

impl Mapper<Rank, RankDto> for RankMapper {
    fn to_dto(model: &Rank) -> RankDto {
        match model {
            Rank::None => RankDto::None,
            Rank::E => RankDto::E,
            Rank::D => RankDto::D,
            Rank::C => RankDto::C,
            Rank::B => RankDto::B,
            Rank::A => RankDto::A,
            Rank::Infinite => RankDto::Infi
        }
    }

    fn to_model(dto: &RankDto) -> Rank {
        match dto {
            RankDto::None => Rank::None,
            RankDto::E => Rank::E,
            RankDto::D => Rank::D,
            RankDto::C => Rank::C,
            RankDto::B => Rank::B,
            RankDto::A => Rank::A,
            RankDto::Infi => Rank::Infinite
        }
    }
}