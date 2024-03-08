use std::{any::Any, cell::RefCell, rc::Rc};

use crate::{
    component::Component,
    entity::{entity_manager::EntityManager, EntityId},
    world::World, TestSystem,
};

pub trait EcsManager {
    /// As Any trait is implemented to facilitate downcasting
    /// into the appropriate system type by the event manager
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}



pub trait BaseSystem {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;

    fn process_update(&mut self, world: Rc<RefCell<World>>);
    fn process_start(&mut self, world: Rc<RefCell<World>>);
}



pub trait ComponentSystem
{
    /// This is the type of the specific component that the system is
    /// supposed to handle
    type ComponentType;
    
    /// The update function is called on every component on every update cycle
    fn on_update(&self, world: Rc<RefCell<World>>, entity_id: EntityId, component: &mut Self::ComponentType);
    fn on_start(&self, world: Rc<RefCell<World>>) {}
}