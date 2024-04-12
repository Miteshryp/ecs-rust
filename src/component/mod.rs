pub mod component_manager;
pub mod handles;

use std::any::{Any, TypeId};

use crate::{
    entity::{entity_manager::EntityManager, Entity},
    world::World,
};

pub trait Component {
    fn get_name() -> String
    where
        Self: Sized;
}

