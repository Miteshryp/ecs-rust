mod app;
mod component;
mod ecs_base;
mod entity;
mod events;
mod resource;
mod schedule;
mod system;
mod world;
mod tests;

use crate::events::Event;
use crate::system::System;
use app::App;
use component::Component;
use ecs_base::ECSBase;
use ecs_macros::{Component, Event, Resource};
use entity::Entity;
use resource::Resource;
use schedule::{parallel::ParallelSchedule, schedulable::IntoSchedulable, FlowFrequency, Schedule};
use std::{any::Any, sync::mpsc::channel};
use system::param::{CommandBufferWriter, ComponentCollectionMut, CrossComponentCollectionMut, EventReader, EventWriter, MutResourceHandle, QueryMut, ResourceHandle};
use world::{command_type::CommandFunction, unsafe_world::UnsafeWorldContainer, World};

#[derive(Component)]
struct NewComponent {
    t: f32,
}

#[derive(Event)]
struct SampleEvent {
    i: i32,
}

#[derive(Resource)]
struct SampleResource {
    i: i32,
}

fn init(writer: CommandBufferWriter) {
    println!("Here");
    writer.add_command(|world: &mut World| {
        let s = SampleResource { i: 45 };
        world.add_resource(s);
    });
    let some_value = 4;

    writer.add_command(move |world: &mut World| {
        for i in 0..3 {
            let id = world.create_entity();
            world.add_component_to_entity(
                id,
                NewComponent {
                    t: some_value as f32 * i as f32,
                },
            );
        }
    });
}

fn component_iter(mut components: ComponentCollectionMut<NewComponent>) {
    for c in components {
        println!("Value is: {}", c.t);
    }
}

fn cross_component_system(mut component: CrossComponentCollectionMut<NewComponent>, commands: CommandBufferWriter) {
    component.handler(|a, b| {
        println!("Comp A: {} [] Comp B: {}", a.t, b.t);
    });

    commands.add_command(|world| {
        world.set_active(false);
    })
}
 
fn query_system(mut comp_query: QueryMut<(Entity, NewComponent)>) {
    for c in comp_query {
        println!("EntityID: {:?}, Component Value: {}", c.0, c.1.t);
    }
}



fn main() {
    let mut app = App::new();

    app.register_component::<NewComponent>();

    let mut once_schedule = ParallelSchedule::new();
    once_schedule.add_boxed(init.into_schedulable());

    let mut schedule = ParallelSchedule::new();

    // schedule.add_boxed(query_system.into_schedulable());
    // schedule.add_boxed(component_iter.into_schedulable());
    schedule.add_boxed(cross_component_system.into_schedulable());

    // Init holder
    let init_index = app.register_flow(schedule::FlowFrequency::Once);
    let update = app.register_flow(schedule::FlowFrequency::Always);
    
    // app.register_component::()
    app.add_to_flow(init_index, once_schedule);
    app.add_to_flow(update, schedule);

    app.start();
    // loop {
    //     // app.update();
    // }
}
