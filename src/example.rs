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
use app::App;
use component::{Component};
use ecs_base::ECSBase;
use ecs_macros::{Component, Event, Resource};
use entity::Entity;
use log::{logger, Level, Metadata, Record};
use resource::Resource;
use schedule::{parallel::ParallelSchedule, ScheduleHolderFrequency, Schedule};
use system::param::{CommandBufferWriter, ComponentCollection, ComponentCollectionMut, CrossComponentCollection, CrossComponentCollectionMut, EventReader, EventWriter, MutResourceHandle, QueryMut, ResourceHandle};
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
    component.execute_handler(|a, b| {
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


#[derive(Component)]
struct Collider {
    member: i32
}
impl Collider {
    pub fn check_collision(&self, b: &Collider) -> bool {
        return true;
    }
}

#[derive(Event, Debug)]
struct CollisionEvent {
    collision_A: Entity,
    collision_B: Entity
}
impl Clone for CollisionEvent {
    fn clone(&self) -> Self {
        Self { collision_A: self.collision_A.clone(), collision_B: self.collision_B.clone() }
    }
}

fn init_collider_entities(writer: CommandBufferWriter) {
    writer.add_command(|world: &mut World| {
        for i in 1..=2 {
            let id=  world.create_entity();
            world.add_component_to_entity(id, Collider{ member: i});
        }
    })
}

fn check_collision(mut collision: CrossComponentCollection<Collider>, event_writer: EventWriter) {
    collision.execute_handler(move |a, b| {
        if a.check_collision(&b) {
            event_writer.send_event(CollisionEvent {
                collision_A: a.entity_id,
                collision_B: b.entity_id,
            })
        }
    });    
}
static mut counter: i32 = 0;

fn on_collision(mut event_reader: EventReader<CollisionEvent>, command: CommandBufferWriter) {
    let collision_events: Vec<CollisionEvent> = event_reader.read_events().into_iter().map(|e: &CollisionEvent| e.clone()).collect();
    for event in collision_events {
        unsafe {
            counter += 1;
            log::info!("Event exists!: {:?}", counter);
        }
        command.add_command(move |world: &mut World| {
            world.remove_entity(event.collision_A);
            world.remove_entity(event.collision_B);
        })
    }
}

fn collider_update(components: ComponentCollection<Collider>) {
    for c in components {
        println!("Component found: {}", c.member);
    }
}


struct ConsoleLogger;

impl log::Log for ConsoleLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Info
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            println!("{} - {}", record.level(), record.args());
        }
    }

    fn flush(&self) {}
}
static LOGGER: ConsoleLogger = ConsoleLogger;

fn main() {
    let mut app = App::new();
    let _  = log::set_logger(&LOGGER).map(|()| log::set_max_level(log::LevelFilter::Error)).unwrap();

    app.register_component::<NewComponent>();

    let mut once_schedule = ParallelSchedule::new();
    // once_schedule.add_boxed(init.into_schedulable());
    once_schedule.add(init_collider_entities);


    let mut schedule = ParallelSchedule::new();

    schedule.add(check_collision);
    schedule.add(on_collision);
    schedule.add(collider_update);

    // Init holder
    let init_index = app.register_schedule_holder(schedule::ScheduleHolderFrequency::Once);
    let update = app.register_schedule_holder(schedule::ScheduleHolderFrequency::Always);
    
    app.register_component::<Collider>();
    app.add_to_holder_index(init_index, once_schedule);
    app.add_to_holder_index(update, schedule);

    app.start();
}
