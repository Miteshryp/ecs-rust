use std::any::Any;

use crate::{component::Component, entity::EntityId};

pub trait System {
    /// As Any trait is implemented to facilitate downcasting 
    /// into the appropriate system type by the event manager
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;

    // Functions for updating the system
    // fn update(... Parameters);
}