mod events;
mod app;
mod component;
mod ecs_base;
mod entity;
mod system;
mod world;
mod schedule;
mod resource;


use app::App;
use resource::Resource;
use component::{Component};
use ecs_base::ECSBase;
// use ecs_macros::{Component, ComponentSystem, Resource};
use ecs_macros::{Component, Event, Resource};
use entity::Entity;
use schedule::{parallel::ParallelSchedule, IntoSchedulable, Schedule};
use system::param::ResourceHandle;
use std::{
    any::Any,
    cell::{Cell, Ref, RefCell, RefMut},
    rc::Rc,
};
// use system::{BaseSystem, ComponentSystem, ResourceSystem};
use world::{ unsafe_world::UnsafeWorldContainer, World};
use crate::events::Event;
use crate::system::{System};

// Testing code

#[derive(Component)]
pub struct TestComponent {
    pub i: u32,
}

// #[derive(ComponentSystem)]
// struct TestSystem;
// impl ComponentSystem for TestSystem {
//     type ComponentType = TestComponent;

//     fn on_update(
//         &self,
//         mut world: &mut WorldArg,
//         entity_id: Entity,
//         component: RefMut<'_, TestComponent>,
//     ) {
//         println!("Component: {}", component.i);
//     }

//     fn on_start(&self, mut world: &mut WorldArg) {
//         let id = world.create_entity();
//         world.add_component_to_entity(id, TestComponent { i: 34 });

//         let id = world.create_entity();
//         world.add_component_to_entity(id, TestComponent { i: 90 });
//     }
// }

#[derive(Component)]
struct NewComponent {
    t: f32,
}



// #[derive(ComponentSystem)]
// struct NewSystem;

// impl ComponentSystem for NewSystem {
//     type ComponentType = NewComponent;

//     fn on_update(
//         &self,
//         world: &mut WorldArg,
//         entity_id: Entity,
//         component: RefMut<'_, Self::ComponentType>,
//     ) {
//         println!("New Component: {}", component.t);
//     }

//     fn on_start(&self, world: &mut WorldArg) {
//         let id = world.create_entity();
//         world.add_component_to_entity(id, NewComponent { t: 20.0 });
//     }
// }


#[derive(Event)]
struct SampleEvent {
    i: i32
}


trait SampleTrait {
    fn print(&self);
}

#[derive(Resource)]
struct SampleResource {
    i: i32
}

impl SampleTrait for SampleResource {
    fn print(&self) {
        println!("{}", self.i);
    }
}

fn test_system(mut handle: ResourceHandle<SampleResource>) {
    let res = handle.get_resource();
    println!("Sys A {}", res.i);
}

fn test_system2(mut handle: ResourceHandle<SampleResource>) {
    let res = handle.get_resource();
    println!("New System {}", res.i);
}


fn param_func(t: (i32, i32)) {

}

struct S1 {

}

struct S2 {

}


impl SampleTrait for (S1, S2) {
    fn print(&self) {
    }
}


fn main() {

    let mut schedule = ParallelSchedule::new();
    schedule.add_boxed(test_system.into_schedulable());
    schedule.add_boxed(test_system2.into_schedulable());

    let world = UnsafeWorldContainer::new();
    let res = SampleResource {i:32};
    world.get_world_mut().add_resource(res);

    loop {
        schedule.run_schedule(&world);
    }

    let res = SampleResource { i: 51 };
    let res2 = SampleResource { i: 51 };
    // test_function((res, res2));
    // let t = (2,3).;
    
}
