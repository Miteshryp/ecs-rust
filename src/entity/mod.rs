pub mod entity_manager;

use std::hash::Hash;

pub type EntityId = u32;

pub struct Entity {
    id: EntityId,
}
