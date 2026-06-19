use core::types::Rank;
use crate::dtos::RankDto;
use super::mapper::Mapper;

/// A stateless mapping utility responsible for converting between domain `Rank` enums and infrastructure `RankDto` representations.
pub struct RankMapper;

impl Mapper<Rank, RankDto> for RankMapper {
    /// Maps a domain `Rank` reference into its infrastructure-specific `RankDto` counterpart.
    ///
    /// # Arguments
    ///
    /// * `model` - A shared reference to the domain `Rank` enum variant.
    ///
    /// # Returns
    ///
    /// * The corresponding `RankDto` enum variant used for transfer layouts.
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

    /// Maps an infrastructure `RankDto` reference back into its domain `Rank` representation.
    ///
    /// # Arguments
    ///
    /// * `dto` - A shared reference to the `RankDto` data serialization variant.
    ///
    /// # Returns
    ///
    /// * The corresponding core domain `Rank` enum variant.
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