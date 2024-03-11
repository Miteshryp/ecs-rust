use std::{any::Any, cell::{RefCell, RefMut}, rc::Rc};

use crate::{
    component::Component,
    entity::{entity_manager::EntityManager, Entity},
    world::{World, WorldArg, UnsafeWorldContainer}, TestSystem,
};

pub trait EcsManager {
    /// As Any trait is implemented to facilitate downcasting
    /// into the appropriate system type by the event manager
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;

    fn new() -> Self where Self: Sized;
    fn remove_component_from_entity(&mut self, entity_id: Entity);
    fn has_component(&self, entity_id: Entity) -> bool;
}



pub trait BaseSystem {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;

    // fn process_update(&mut self, world: &RefCell<World>);
    // fn process_start(&mut self, world: &RefCell<World>);

    // fn process_update(&mut self, world: &mut World);
    // fn process_start(&mut self, world: &mut World); 

    fn process_update(&mut self, world: &mut UnsafeWorldContainer);
    fn process_start(&mut self, world: &mut UnsafeWorldContainer); 
}



pub trait ComponentSystem
{
    /// This is the type of the specific component that the system is
    /// supposed to handle
    type ComponentType;
    
    /// The update function is called on every component on every update cycle
    // fn on_update(&self, world: &mut WorldArg<'_>, entity_id: Entity, component: RefMut<'_, Self::ComponentType>) {}
    // fn on_start(&self, world: &mut WorldArg<'_>) {}

    fn on_update(&self, world: &mut WorldArg, entity_id: Entity, component: RefMut<'_, Self::ComponentType>) {}
    fn on_start(&self, world: &mut WorldArg) {}
}


pub trait InteractiveSystem {
    type ComponentA;
    type ComponentB;

    type EventLaunch;

    fn check_interaction(&self, world: Rc<RefCell<World>>, eid_a: Entity, eid_b: Entity, component_a: &Self::ComponentA, component_b: &Self::ComponentB);
}