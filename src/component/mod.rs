pub mod component_manager;
pub mod resource;

use std::any::{Any, TypeId};

use crate::{
    entity::{entity_manager::EntityManager, Entity},
    world::World,
};

pub trait Component {
    // fn as_any(&self) -> &dyn Any;
    // fn as_any_mut(&mut self) -> &mut dyn Any;

    fn get_name() -> String
    where
        Self: Sized;
}

