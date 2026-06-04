pub trait Mapper<Entity, Dto> {
    fn to_dto(entity: &Entity) -> Dto;
    fn to_entity(dto: &Dto) -> Entity;
}