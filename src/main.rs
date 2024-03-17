mod app;
mod component;
mod ecs_base;
mod entity;
mod event;
mod system;
mod world;

use app::App;
use component::{resource::Resource, Component};
use ecs_base::ECSBase;
use ecs_macros::{Component, ComponentSystem, Resource};
use entity::Entity;
use std::{
    any::Any,
    cell::{Cell, Ref, RefCell, RefMut},
    rc::Rc,
};
use system::{BaseSystem, ComponentSystem, ResourceSystem};
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
    t: f32,
}

impl ECSBase for Timer
where
    Self: Sized + 'static,
{
    fn as_any(&self) -> &dyn Any {
        self as &dyn Any
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self as &mut dyn Any
    }

    fn downcast_to_ref<T: ECSBase + Sized + 'static>(&self) -> &T
    where
        Self: Sized,
    {
        self.as_any().downcast_ref::<T>().unwrap()
    }

    fn downcast_to_ref_mut<T: ECSBase + Sized + 'static>(&mut self) -> &mut T
    where
        Self: Sized,
    {
        self.as_any_mut().downcast_mut::<T>().unwrap()
    }
}

impl Resource for Timer {
    fn get_name(&self) -> String {
        String::from("Timer")
    }
}
struct Timer {
    i: f32,
}


impl BaseSystem for TimerSystem {
    fn process_update(&mut self, world_container: &mut UnsafeWorldContainer) {
        self.on_update(
            &mut world_container.get_world_mut(),
            world_container.get_world_mut().get_resource_mut(), // Type is inferred by argument type
        );
    }

    fn process_start(&mut self, world_container: &mut UnsafeWorldContainer) {}
    fn process_events(&mut self, world_container: &mut UnsafeWorldContainer) {
        // @Design: How are event launched and stored in the world?
        // 1. Extract events targeted (and broadcasted) to this resource type and run handle_events
        //      a. Iterate through the events pushed in the world
        //      b. Find events which have the system in the `receiver_types`
    }
}
struct TimerSystem;
impl ResourceSystem for TimerSystem {
    type ResourceType = Timer;

    fn on_update(&self, world: &mut WorldArg, resource: &mut Timer) {
        
    }
}

#[derive(ComponentSystem)]
struct NewSystem;

impl ComponentSystem for NewSystem {
    type ComponentType = NewComponent;

    fn on_update(
        &self,
        world: &mut WorldArg,
        entity_id: Entity,
        component: RefMut<'_, Self::ComponentType>,
    ) {
        println!("New Component: {}", component.t);
    }

    fn on_start(&self, world: &mut WorldArg) {
        let id = world.create_entity();
        world.add_component_to_entity(id, NewComponent { t: 20.0 });
    }
}


fn test_function(k: i32) -> f32 {
    k as f32 / 2.4
}


// // &Component, &Resource, Event, ...
// fn add_flow(world: &mut World, resource: dyn ECSBase) {

// }

fn main() {
    let func_var: Box<dyn Fn(i32) -> f32> = Box::new(test_function);
    let k = (func_var)(23);
    println!("Test {k}");
}
