use ecs_macros::{Component, Event, Resource};

use crate::{
    app::App, component::Component, entity::Entity, events::Event, resource::Resource, schedule::{self, parallel::ParallelSchedule, schedulable::IntoSchedulable, Schedule}, system::param::{
        CommandBufferWriter, EventReader, EventWriter, MutResourceHandle, QueryMut, ResourceHandle,
    }, world::World, ECSBase
};

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
    let some_value = 34;

    writer.add_command(move |world: &mut World| {
        let id = world.create_entity();
        world.add_component_to_entity(
            id,
            NewComponent {
                t: some_value as f32,
            },
        );
    });
}

fn query_system(mut comp_query: QueryMut<(Entity, NewComponent)>) {}

fn test_system(mut handle: ResourceHandle<SampleResource>) {
    // println!("Sys A {}", handle.i);
}

fn test_system2(mut handle: ResourceHandle<SampleResource>, world_writer: CommandBufferWriter) {
    // println!("New System {}", handle.i);

    if handle.i > 150 {
        world_writer.add_command(|world: &mut World| {
            world.set_active(false);
        });
    }
}

fn mut_res_sys(mut handle: MutResourceHandle<SampleResource>, writer: EventWriter) {
    // println!("This is the mutable system");
    handle.i += 1;

    if handle.i % 50 == 0 {
        println!("Sent event");
        writer.send_event(SampleEvent { i: handle.i / 50 });
    }
}

fn event_reader(reader: EventReader<SampleEvent>) {
    for event in reader.read_events() {
        println!("Event Received: {}", event.i);
    }
}

fn ordered_to_system2(mut handle: ResourceHandle<SampleResource>) {
    // println!("Ordered system");
}

#[test]
fn parallel_scheduler_test() {
    let mut app = App::new();

    app.register_component::<NewComponent>();

    let mut once_schedule = ParallelSchedule::new();
    once_schedule.add_boxed(init.into_schedulable());

    let mut schedule = ParallelSchedule::new();
    schedule.add_boxed(test_system.into_schedulable());
    schedule.add_boxed(event_reader.into_schedulable());
    // schedule.add_boxed(test_system2.into_schedulable());

    schedule.add_ordered(test_system2.before(ordered_to_system2));
    schedule.add_boxed(mut_res_sys.into_schedulable());

    // Init flow
    let init_index = app.register_schedule_holder(schedule::ScheduleHolderFrequency::Once);
    let update = app.register_schedule_holder(schedule::ScheduleHolderFrequency::Always);

    // app.register_component::()
    app.add_to_holder_index(init_index, once_schedule);
    app.add_to_holder_index(update, schedule);

    app.start();
    loop {
        app.update();
    }
}
