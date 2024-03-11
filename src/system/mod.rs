use std::{
    any::Any,
    cell::{RefCell, RefMut},
    rc::Rc,
};

use crate::{
    entity::{Entity},
    world::{UnsafeWorldContainer, World, WorldArg},
};

pub trait BaseSystem {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;

    fn process_update(&mut self, world: &mut UnsafeWorldContainer);
    fn process_start(&mut self, world: &mut UnsafeWorldContainer);
}

pub trait ComponentSystem {
    /// This is the type of the specific component that the system is
    /// supposed to handle
    type ComponentType;

    /// The update function is called on every component on every update cycle
    fn on_update(
        &self,
        world: &mut WorldArg,
        entity_id: Entity,
        component: RefMut<'_, Self::ComponentType>,
    ) {
    }
    fn on_start(&self, world: &mut WorldArg) {}
}

pub trait InteractiveSystem {
    type ComponentA;
    type ComponentB;

    type EventLaunch;

    fn check_interaction(
        &self,
        world: Rc<RefCell<World>>,
        eid_a: Entity,
        eid_b: Entity,
        component_a: &Self::ComponentA,
        component_b: &Self::ComponentB,
    );
}
