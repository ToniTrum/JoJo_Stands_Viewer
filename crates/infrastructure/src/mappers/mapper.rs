/// A generic data mapping abstraction layer used to convert data structures between different architectural layers.
///
/// Typically implemented within the infrastructure layer to decouple core domain models from external Data Transfer Objects (DTOs).
///
/// # Type Parameters
///
/// * `Model` - The core domain model representing business logic rules.
/// * `Dto` - The data transfer representation..
pub trait Mapper<Model, Dto> {
    /// Maps a core domain model reference into its data transfer object counterpart.
    ///
    /// # Arguments
    ///
    /// * `model` - A shared reference (`&Model`) to the core entity being converted.
    ///
    /// # Returns
    ///
    /// * An instance of the corresponding `Dto`.
    fn to_dto(model: &Model) -> Dto;

    /// Maps a data transfer object reference back into its core domain model representation.
    ///
    /// # Arguments
    ///
    /// * `dto` - A shared reference (`&Dto`) to the data transfer object layout being converted.
    ///
    /// # Returns
    ///
    /// * An instance of the corresponding core `Model`.
    fn to_model(dto: &Dto) -> Model;
}