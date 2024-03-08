pub mod component_manager; 

use std::any::{Any, TypeId};

use crate::{entity::{entity_manager::EntityManager, EntityId}, world::World};

pub trait Component {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;

    fn get_name() -> String where Self: Sized;
}


pub trait ComponentHandler<Comp>
where
    Comp: Component,
    Self: 'static,
{
    // fn update();
    fn new() -> Self
    where
        Self: Sized;

    fn get_id() -> TypeId
    where
        Self: Sized,
    {
        TypeId::of::<Self>()
    }

    // What all do we need to update the component?
    // - World API
    // - Component to be updated
    // - entity id of the component
    fn on_update(&mut self, world: &mut World, component: &mut Comp, entity_id: EntityId);
}