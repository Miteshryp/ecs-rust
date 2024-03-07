mod world;
mod entity;
mod component;
mod system;

use component::{component_system::ComponentSystem, Component, ComponentHandler};
use ecs_macros::Component;
use world::World;
/*  Example Snippet 

     let mut world = World::new();
     world = world
         .register::<T1>::()
         .register::<T2>::()

    world.start();
*/


// Testing code

#[derive(Component)]
pub struct TestComponent {
    i: u32,
}

pub struct TestSystemHandler;
impl ComponentHandler<TestComponent> for TestSystemHandler {
    fn new() -> Self {
        Self {}
    }

    // fn update(TestComponent) {...}
}

fn main() {
    let mut world = World::new();
    world.register_component_with_handler::<TestComponent, TestSystemHandler>();
    world.create_entity();
    // component_system.add_component_to_entity(entity_id, component)
}
