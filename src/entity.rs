use std::hash::Hash;

pub type EntityId = u32;

/// A structure for storing entity id
// pub struct EntityId(u32);
// impl PartialEq for EntityId{
//     fn eq(&self, other: &Self) -> bool {
//         self.0 == other.0
//     }
// }
// impl PartialOrd for EntityId {
//     fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
//         self.0.partial_cmp(&other.0)
//     }
// }
// impl Hash for EntityId{
//     fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
//         self.0.hash(state);
//     }
// }
// impl Eq for EntityId {}
// impl Copy for EntityId {}

// /// Trait to be implemented by every entity existing in the system
pub trait Entity {
    fn entity_id() -> EntityId;
}