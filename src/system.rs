use std::{any::{Any, TypeId}, collections::HashMap};

use crate::{entity::EntityId};

use super::component::Component;

pub trait ComponentHandler<Comp>
where
    Comp: Component,
    Self: 'static
{
    // fn update();
    fn new() -> Self 
        where Self: Sized;
    

    fn get_id() -> TypeId 
        where Self: Sized
    {
        TypeId::of::<Self>()
    }
}

pub trait System {
}


/// Contains the interface for implementing the logic for handling a specific type of component
/// 
/// @NOTE: One entity can only have one instance of a type of component, so maybe component id is useless
///     For a component, we can simply use the entity id to identify the component attached to the entity
pub struct ComponentSystem<Comp>
where
    Comp: Component,
{
    system_id: TypeId,

    /// Contiguous array of components which enables us to do faster
    /// update operations due to better cache locality.
    components: Vec<Comp>,

    
    /// Stores the entity ids of the components in the components vector 
    /// in the same order as the components are presented in the array 
    /// above.
    entities: Vec<EntityId>,

    /// [EntityId] to 'index' mapping to boost lookup
    /// 
    /// NOTE: We use [EntityId] to identify a component in the system since 
    ///     each component in existence has to be attached to an entity,
    ///     therefore for a component system managing a specific type of component,
    ///     each component is attached to a unique entity, hence we can use
    ///     entity id as the identifier for the components
    component_hash_map: HashMap<EntityId, usize>,


    /// Handler attached to the system object for enabling custom updation
    /// of the components in the system
    handler: Box<dyn ComponentHandler<Comp>>
}
impl<Comp> System for ComponentSystem<Comp>
where
    Comp: Component + 'static,
{}


// Public functions of the ComponentSystem struct
impl<Comp> ComponentSystem<Comp>
where
    Comp: Component + 'static,
{

    /// System initialisation function. Used to create a new system to handle the specified type of component
    pub fn new<Handler>() -> Self 
    where
        Handler: ComponentHandler<Comp>
    {
        ComponentSystem {
            system_id: TypeId::of::<Self>(),
            components: vec![],
            entities: vec![],
            component_hash_map: HashMap::new(),
            handler: Box::new(Handler::new())
        }
    }

    /// Adds a component into the system based on the stack-build object passed
    /// as a parameter
    pub fn add_component_to_entity(&mut self, entity_id: EntityId, component: Comp) {
        if self.component_hash_map.contains_key(&entity_id) {
            // Entity already has the component attached to it.
            // @TODO: Handle error
            return;
        }

        let component_index = self.components.len();
        self.components.push(component);
        self.component_hash_map.insert(entity_id, component_index).unwrap();
    }


    /// Removes a component from the entity if the component was attached,
    pub fn remove_component_from_entity(&mut self, entity_id: EntityId) {
        
        if !self.component_hash_map.contains_key(&entity_id) {
            // Entity does not have the component attached to it, hence removal is not possible
            // @TODO: Handle error
            return;
        }

        let components_length = self.components.len();
        let component_index = self.component_hash_map.get(&entity_id).unwrap();

        // O(1) removal time, without disturbing majority of the elements indexes
        self.components.swap_remove(*component_index);
        self.entities.swap_remove(*component_index);

        
        // Updating the lookup table with the replaced indexes
        if components_length - 1 != *component_index {
            self.component_hash_map.insert(self.entities[*component_index], *component_index).unwrap();
        }
        self.component_hash_map.remove(&entity_id).unwrap();
    }


    /// Returns the id of the component being handled by the system.
    pub fn get_component_id() -> TypeId {
        TypeId::of::<Comp>()
    }

    pub fn get_system_id(&self) -> TypeId {
        self.system_id
    }
}





// Testing code
pub struct TestComponent {
    i: u32
}
impl Component for TestComponent {}

pub struct TestSystemHandler;
impl ComponentHandler<TestComponent> for TestSystemHandler {
    fn new() -> Self {
        Self {}
    }

    // fn update(TestComponent) {...}
}

fn test_main() {
    let mut component_system = ComponentSystem::<TestComponent>::new::<TestSystemHandler>();
    // component_system.add_component_to_entity(entity_id, component)
}