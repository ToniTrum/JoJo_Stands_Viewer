pub trait Mapper<Model, Dto> {
    fn to_dto(model: &Model) -> Dto;
    fn to_model(dto: &Dto) -> Model;
}