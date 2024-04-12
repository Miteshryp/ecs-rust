mod app;
mod component;
mod ecs_base;
mod entity;
mod events;
mod resource;
mod schedule;
mod system;
mod world;

use crate::events::Event;
use crate::system::System;
use app::App;
use component::Component;
use ecs_base::ECSBase;
use ecs_macros::{Component, Event, Resource};
use resource::Resource;
use schedule::{parallel::ParallelSchedule, FlowFrequency, IntoSchedulable, Schedule};
use std::{any::Any, sync::mpsc::channel};
use system::param::{CommandBufferWriter, ResourceHandle};
use world::{command_type::CommandFunction, unsafe_world::UnsafeWorldContainer, World};

// Testing code

#[derive(Component)]
pub struct TestComponent {
    pub i: u32,
}

#[derive(Component)]
struct NewComponent {
    t: f32,
}

#[derive(Event)]
struct SampleEvent {
    i: i32,
}

trait SampleTrait {
    fn print(&self);
}

#[derive(Resource)]
struct SampleResource {
    i: i32,
}

impl SampleTrait for SampleResource {
    fn print(&self) {
        println!("{}", self.i);
    }
}

// fn init(writer: CommandBufferWriter) {
//     println!("Here");
//     writer.add_command(|world: &mut World| {
//         let s = SampleResource {i:45};
//         world.add_resource(s);
//     })
// }

fn test_system(mut handle: ResourceHandle<SampleResource>) {
    let res = handle.get_resource();
    println!("Sys A {}", res.i);
}

fn test_system2(mut handle: ResourceHandle<SampleResource>) {
    let res = handle.get_resource();
    println!("New System {}", res.i);
}

fn param_func(t: (i32, i32)) {}

struct S1 {}

struct S2 {}

impl SampleTrait for (S1, S2) {
    fn print(&self) {}
}

fn main() {
    let mut app = App::new();

    // let mut once_schedule = ParallelSchedule::new();
    // once_schedule.add_boxed(init.into_schedulable());

    let mut schedule = ParallelSchedule::new();
    schedule.add_boxed(test_system.into_schedulable());
    schedule.add_boxed(test_system2.into_schedulable());

    // Init flow
    // let init_index = app.register_flow(schedule::FlowFrequency::Once);
    let update = app.register_flow(schedule::FlowFrequency::Always);

    // app.register_component::()
    // app.add_to_flow(init_index, once_schedule);
    app.add_to_flow(update, schedule);

    loop {
        app.update();
    }
}
