mod world;
mod entity;
mod component;
mod system;
mod app;

use app::App;
use component::{component_manager::ComponentManager, Component, ComponentHandler};
use system::{BaseSystem, ComponentSystem};
use entity::EntityId;
use ecs_macros::{Component, ComponentSystem};
use world::World;
use std::{any::Any, borrow::BorrowMut, cell::RefCell, rc::Rc};


// Testing code

#[derive(Component)]
pub struct TestComponent {
    i: u32,
}


#[derive(ComponentSystem)]
struct TestSystem;
impl ComponentSystem for TestSystem {
    type ComponentType = TestComponent;

    fn on_update(&self, world: Rc<RefCell<World>>, entity_id: EntityId, component: &mut TestComponent) {
        println!("Works");
    }

    fn on_start(&self, world: Rc<RefCell<World>>) {
        println!("It is starting");
    }
}

fn main() {
    let mut app = App::new();
    app.add_component_system(TestSystem {});
    app.start();
}
