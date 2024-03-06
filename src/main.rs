
mod world;
mod entity;
mod component;
mod system;

use component::{component_system::ComponentSystem, Component, ComponentHandler};
/*  Example Snippet 

     let mut world = World::new();
     world = world
         .register::<T1>::()
         .register::<T2>::()

    world.start();
*/


// Testing code
pub struct TestComponent {
    i: u32,
}
impl Component for TestComponent {
    fn get_name() -> String {
        String::from("TestComponent")
    }
}

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


fn main() {
}
