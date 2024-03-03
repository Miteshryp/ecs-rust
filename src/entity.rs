
/// A structure for storing entity id
pub struct EntityId {
    id: u128
}


/// Trait to be implemented by every entity existing in the system
pub trait Entity {
    fn entity_id() -> EntityId;
}