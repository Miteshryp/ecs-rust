use std::{
    any::Any,
    cell::{RefCell, RefMut},
    rc::Rc,
};

use crate::{
    component::resource::{Resource, ResourceId}, entity::Entity, event::ECSevent, world::{UnsafeWorldContainer, World, WorldArg}
};

pub trait BaseSystem {

    // System flows (Accessed by the App interface)
    fn process_update(&mut self, world: &mut UnsafeWorldContainer) {}
    fn process_start(&mut self, world: &mut UnsafeWorldContainer) {}
    fn process_events(&mut self, world: &mut UnsafeWorldContainer) {}
}

pub trait ResourceSystem {
    type ResourceType;

    fn initialise(&self, world: &mut WorldArg) -> Self::ResourceType {
        todo!();
    }

    fn on_start(&self, world: &mut WorldArg) {
    }

    fn on_update(&self, world: &mut WorldArg, resource_id: &mut Self::ResourceType) {

    }

    fn handle_event<E: ECSevent>() {

    }
}



/// System interface for handling components.
/// 
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
