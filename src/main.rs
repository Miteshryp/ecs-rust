mod app;
mod component;
mod entity;
mod system;
mod world;

use app::App;
use component::Component;
use ecs_macros::{Component, ComponentSystem};
use entity::Entity;
use std::{
    any::Any,
    cell::{Cell, Ref, RefCell, RefMut},
    rc::Rc,
};
use system::{BaseSystem, ComponentSystem};
use world::{UnsafeWorldContainer, World, WorldArg};

// Testing code

#[derive(Component)]
pub struct TestComponent {
    pub i: u32,
}

#[derive(ComponentSystem)]
struct TestSystem;
impl ComponentSystem for TestSystem {
    type ComponentType = TestComponent;

    fn on_update(
        &self,
        mut world: &mut WorldArg,
        entity_id: Entity,
        component: RefMut<'_, TestComponent>,
    ) {
        println!("Component: {}", component.i);
    }

    fn on_start(&self, mut world: &mut WorldArg) {
        let id = world.create_entity();
        world.add_component_to_entity(id, TestComponent { i: 34 });

        let id = world.create_entity();
        world.add_component_to_entity(id, TestComponent { i: 90 });
    }
}


#[derive(Component)]
struct NewComponent {
    t: f32
}

#[derive(ComponentSystem)]
struct NewSystem;

impl ComponentSystem for NewSystem {
    type ComponentType = NewComponent;
    
    fn on_update(&self, world: &mut WorldArg, entity_id: Entity, component: RefMut<'_, Self::ComponentType>) {
        println!("New Component: {}", component.t);
    }

    fn on_start(&self, world: &mut WorldArg) {
        let id = world.create_entity();
        world.add_component_to_entity(id, NewComponent { t: 20.0 });
    }
}


fn main() {
    let mut app = App::new();
    app.add_component_system(TestSystem {});
    app.add_component_system(NewSystem {});
    app.start();
}
