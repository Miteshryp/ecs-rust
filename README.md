# The ECS System Library in Rust

A framework for writing programs in accordance with the ECS (Entity Component System) paradigm commonly followed in game engines.

## Motivation
This is a sub-project which was initiated as a exploration while I was building my game engine in the rust programming language. Although ECS frameworks exist in the market (some very advanced than this currently aims to be), This project for me has been an excellent way to explore the internals of the rust programming language while at the same time challenging my previous designing skills by stepping into a new domain of API and system design.



# Guide

The ECS system can be easily used by using functions as systems. The library offers many types of parameters which can be used to get and process data from the world.
But we're getting ahead of ourselves, so lets start by creating the first layer of interaction, which starts with the `App` object

```rust
fn main() {
    let app = App::new();
    app.start()
}
```

This simple snippet of code is enough to create a app object. When we start our app, our world is set as active, and keeps on updating until the world remains set to active

We can technically run this app, and the app will run for eternity. This is because we have defined no functionality in our ECS app, so let's learn how to do that

## Entities, Components and Resources
The `App` object stores the state of the application inside a `World` object, which is a member variable stored in the app object and is responsible for updating and recording our instructions and updating data accordingly.

The data in the World can broadly be classified into 2 main categories:
    - Components
    - Resources
This is not the entire picture, but lets try to understand these two first.


### Components
Components are just some data that we can define ourselves which we might want to store in the world by attaching it to an entity. We can attach any number of components to an entity, for which we first need to create an entity.
A component will always need to exist with an entity, and cannot exist as an independent piece of data in the world. We can get associated components and perform operations on them, as we will later see in the Systems section.

We can define our own structures as components by using a simple derive macro that the ECS library provides us through the `ecs_macros` crate inside the library crate.
```rust
use ecs_rust::ecs_macros::Component;

#[derive(Component)]
struct Velocity {
    vel_x: f32,
    vel_y: f32
}
``` 

But BEWARE! We cannot use any component with our ECS system without first registering the component type in the ECS application world.


### Resources
Similar to components, resources are also essentially just a structure that we, as a user, can define ourselves and tell the ECS framework to store in our application. However, unlike components, Resources can only exist as an independent piece of data in the application. We cannot attach a resource to any entity in the world, and further more we can only have one instance of a single type of resource. In other words, the data defined as a Resource is unique in the entire world.

We should use resources when we want to define global state, and then we can perform operations on that state using systems.

Similar to components, we can also create our own resource structure using another derive macro from the `ecs_macros` crate in the library
```rust
#[derive(Resource)]
struct Timer {
    ticks_since_launch: u128
}
```

Also Similar to components, we must also register all types of components which are to be used by our systems.


### Entity
We have talked a lot about an Entity, but what is it really? Well, nothing but 2 numbers! Really, no kidding. An Entity is just an identifier that we use to identify and associate different components in the system. Using this entity id we can read, access, modify and delete any component attached to the entity
Removing the entity automatically deletes all the component instances that were attached to the entity.

So, since we now understand the basic blocks of data in the ECS framework, how can we perform operations on them? We do that using systems.



## Systems in ECS
Just as the `World` object represents the state of our application, i.e. the data that is stored as part of our application processes, the Systems stored in the world object allow us to perform operations on that stored state. 
This library has support for functional systems, so we can simply create a function by functions with special parameter. But what are these special parameters. Well, lets try to understand them.

## System Parameters
System Parameters (or extractors) are structures that are defined by this framework to facilitate communication and access into the world for a user. We can define a function using these different extractors as system parameters, which allow us to interact with component, resources, or any other data which is stored as part of the application.
Let's look at the following example snippet

```rust
fn move_particle(entities: QueryMut<(Position, Speed)>) {
    for (entity_id, position, speed) in entities {
        // process components
        position.x += speed.vel_x;
        position.y += speed.vel_y;

        speed.vel_y -= 9.8;
    }
}
```

This example snippet allows us to grab all entities which have a Position and a Speed component attached to them, and allows us to update their position based on their speed component, and at the same time allows us to add or decrease their speed as well.

There are a number of these extractors which could be used to get access to data stored in our application. All the in-built system parameters are listed below:
- Components
    1. ComponentCollection
    2. ComponentCollectionMut
    3. CrossComponentCollection
    4. CrossComponentCollectionMut
    5. Query
    6. QueryMut
- Resources
    1. ResourceHandle
    2. MutResourceHandle
- Events:
    1. EventReader
    2. EventWriter
- Commands
    1. CommandBufferWriter

(In the future, we might allow custom system parameter)


### Events and World Commands
In the end of last section, the List of system parameters contained 2 sections which we have yet not seen in the guide: Events and Commands. Lets try to understand what these are

Events are another type of data that we can send into the world to be processed by our systems, but unlike any other data stored in the world, this data send it ephemeral in nature. 
In laymen terms, this data is temporary and only live on until the next cycle of update, where the data must be processed by any system that might be interested in reading it. If the data is not processed in this frame, it is lost in the next update cycle, since the events are purged and new events generated in the previous cycle are placed for reception.

But wait, how are these events generated exactly? And who reads them and how? Well, its all us! Lets look at the following code to understand this a bit better.

```rust
#[derive(Event, Clone)]
struct CollisionEvent {
    collision_A: Entity,
    collision_B: Entity
}

fn check_collision(mut collision: CrossComponentCollection<Collider>, event_writer: EventWriter) {
    collision.handler(|a, b| {
        if a.check_collision(&b) {
            event_writer.send_event(CollisionEvent {
                collision_A: a.entity_id,
                collition_B: b.entity_id,
            })
        }
    })    
}

fn on_collision(event_reader: EventReader<CollisionEvent>, command: CommandBufferWriter) {
    let collision_events: Vec<CollisionEvent> = event_reader.read_events().into_iter().map(|e| e.clone()).collect();

    for event in collision_events {
        command.add_command(|world: &mut World| {
            world.remove_entity(event.collision_A),
            world.remove_entity(event.collision_B)
        })
    }
}
```

In this code, the `CrossComponentCollection` parameter allows us to check for interactions between all cross-product tuple combinations of the components present in the application.

This way, we can check for interactions between every combination of components present in our application.

In the above example, if we find a collision occuring between 2 colliders, we emit an event from the same system. This event will now be registered in the next update cycle to be read by the reader system. The reader system could collect each event vector in its suited manner and can perform operations on it.



Now, If you notice the above code example carefully, you might find a big issue with this, which can cause potentially problematic behavior.

What happens with the `check_collision` function in the next frame? 
Well, since the reader function will get executed in this frame, it should delete the components, so maybe it should not execute right? Well not really. Beside from lack of guarantee of execution of independent systems in a parallel schedule, this also has something to do with the behaviour of the `CommandBufferWriter` parameter


### Editing World using Command Buffers
The `CommandBufferWriter` is a system parameter which is used to queue world based commands in the app. These commands are executed at the end of every schedule holder execution. Hence, the functions in the same schedule will not get to notice the changes made to the world by any function. 

Another thing to note here is that the order of commands which have been inserted into the world cannot be guarenteed unless the systems have been defined using induced dependency (more on this later).

If you try to run the example in the previous section, you will notice that the `check_collision` component continues to run in the next frame as well, and emits another set of events to be read in the next frame even though the components should have dissapeared by now. However, the components are removed from the world only after the event is read. Since the event is read this frame, the command is queued for execution at the end of this frame. Hence the events will not be emitted in the next frame.

But what about the events that have been executed in the frame which deletes the components? They will still have to be received by the event reader in the next frame which tries to delete the components from the world again. Since these components do not exist any longer, a warning is emitted without taking any action on the world.

Although we were able to get away with events in the above example, this may not always be the case due to the deferred nature of event emission and reception. Hence, we must keep this behavior in mind while writing Event and Command buffer systems.


## Schedules and Holders
So far, we have created many system functions and learned how different parameters behave with them. But none of this has any relation to the code snippet we wrote at the start of this guide:
```rust
fn main() {
    let app = App::new();
    app.start();
}
```

So lets learn how to add on to this app object now that we have created different Systems

In order to run our systems on the ECS application created above, we must learn to interact with 2 more subsystems:
1. Schedulers
2. ScheduleHolders (also called Flows)

### Schedulers
Schedulers is an object that is responsible for determining the order of execution of the system inserted into it such that no two systems can access a specific resource of the world object at the same time. For parallel scheduler(which is the only supported scheduler as of now), this ensures that there is no situation in the execution scheduling process that might result in a resource starvation situation for any 2 systems executing in parallel.

We can create a schedule as follows and start adding systems that we've created previously into it.
```rust
    let schedule = ParallelSchedule::new();
    schedule.add(check_collision);
    schedule.add(on_collision);
```

The parallel scheduler now automatically creates an optimized execution plan to execute the systems in parallel. To get to know more about how this is done, please refer to the component specific documentation.

With that done, we can now integrate the created schedule into the application. But to do this, we must learn about one more component: The System Holder

### System Holders
A System Holder (or flow) is simply a wrapper around multiple schedules which are executed in a serial order. When multiple parallel schedules are added in a system holder, they are executed in a serial fashion in the order that they were inserted into the holder.
System Holder allows us to group a bunch of schedules together and define certain runtime configurations on them
(As of now, we can only configure the frequency of the entire holder. We might add some new functionality in the future)

A single app instance can have multiple such System Holder, which are executed in the same order that they were executed. We can register a holder spot in the app instance as follows:

```rust
let system_holder_index = app.register_system_holder(ScheduleHolderFrequency::Always);
```
This code snippet registers a system holder inside the app instance and assigns a execution frequency to the holder. The index returned to us can now be used to insert a schedule into the system holder


So, lets weave in everything that has been mentioned yet and finally run our ECS application using the following code snippet after defining the system functions;
```rust
fn main() {
    let app = App::new();
    
    let initialiser = ParallelSchedule::new();
    let schedule = ParallelSchedule::new();
    
    initialiser.add(init_entities_system);
    
    schedule.add(on_collision);
    schedule.add(check_collisions);

    let init_index = app.register_schedule_holder(SchedulableHolderFrequency::Once);
    
    let updation_index = app.register_schedule_holder(SchedulableHolderFrequency::Always);

    app.register_component::<Collider>();
    app.add_to_holder_index(init_index, initialiser);
    app.add_to_holder_index(updation_index, schedule);

    app.start();
}
```


There are many more nuances in various types of system parameters, which you are free to explore by looking at the source code.
This library is an attempt to provide a usable, efficient and functional ECS API for your projects. Any feedback, suggestions or contributions to the project are welcome.

If you find any bugs or improvement opportunities, feel free to raise a PR on this repository so I can look into it.


