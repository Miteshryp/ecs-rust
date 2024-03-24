# CORE IDEAS
- Entity is just a wrapper for components which itself is just an id. This id is unique to entity.

- Components are structures which contain data which is contiguously allocated in memory, and all components of a single type are updated together contiguously, leading to better cache locality.

- Components are updated by Systems, which are responsible for creating, managing and handling component object and event associated with them.

- A given entity can only have one instance of a specific type of component.

- These Systems are attached to the App, which launches these systems at appropriate times.

- The World object is the state of the entire ECS system, whereas the systems attached to the world define the behavior of the system.



# IMPLEMENTATION IDEAS

- Since each entity can only have one instance of any given component, the component can be identified through their entity_id in the component manager.

- Do we want to allow entities to be attached to other entities? Probably not.

- We may want functional systems in the design. These allow us to have some advantages
    - It highly decouples logic containers across the codebase, which helps us keep code modularized
    - It optimizes execution since now only functions which contain logic can be executed instead of each flow of each system (in the older design, even if the flow handler had no code to be executed, the derive for every system had to manually go through each dynamically dispatched system, which is highly wasteful )

- Systems registered in the App currently handle a single type of component. We might want to define interactive systems as well where 2 different type of components can interact. These types of systems will generally need to be initiated once the updation cycle for the individual components has been completed.
    - Based on this analysis, we can have 2 types of systems
        1. Interactive systems (Take 2 or maybe more different types of components in handling functions)
        2. Individualistic systems (Take single component in the handling functions)

- There may be 'n' number of such systems (depending on different applications), and hence it is a better design in my opinion to design a task scheduler which launches various systems at their appropriate times.

- We must also implement mechanisms to ensure the following:
    1. One component instance can only belong to a single entity. For this, each component needs to keep track of the entity its currently attached to. This i think will have to be the responsibility of a component manager
    The question is now which manager, an entity manager or a component manager?
    The aim of this mechanism is that if a component is removed from an entity, the component can be removed from memory. The converse however does not stand true, i.e. an empty or orphan entity can exist (one with no components) [Think if we should allow empty entities]

    2. There should only exist one system to handle a single type of component or entity in the world. Hence the world class needs to make sure it only creates new systems for entities or components which are being created for the first time.








# QUESTIONS
 
### How do we ID entities

We create `EntityId`s using the concept of generational indexes (see reference for more), which allows us to reuse existing spots in the entity array without clashing with the old id.







# Resources
Resources are just components in the world which do not have any parent entity. These 'resources' will have to be identified by a special `ResourceId` since they do not have any parents.
Since they are also independent, we do not need to enforce their storage in a contiguous memory block, and hence can be stored in hashmaps (or sparsesets in bevy) to store the Resources.

Just like components can be fetched using the id of the entity they are attached to, resources will be fetched using `ComponentId`s.
We also need to keep checks in place to ensure that there are not more than one instance of a single resource type.



# Functional Systems
### Design
- A functional system is any function defined in the scope whose parameters implement the `SystemParam` type.

- These `SystemParam`s are actually just extractors of state values from a world instance.

- These `extractor` parameters are going to be declared by ECS system and can be used by the user to extract state data from the assigned world to be used in a system function.

### Critical issue regarding parallel systems

Currently, all the `SystemParam`s that need to be passed into a system are being initialised everytime run is called on the system, which means that all `SystemParam`s initialise every frame.

But this could also cause trouble leading to data race conditions.
(2 `EventReader<E>`s creating an empty vector for the same type in the `EventManager`) (This is a non-catastrophic thing, but worse things could happen)
However, we cannot really create a RwLock on the world, since that would essentially force the entire design to be sequential, since the initialise function requires a `&mut World`, and that would require every system to secure a lock to the world before execution, hence essentially making the system sequential.
To solve this, we have to allow multiple `&mut World`s, but we have to ensure that the internal state remains consistent throughout, meaning that if 2 systems are accessing the same resource, they should be `relatively sequential`





### Flow

- Any function declared with compatible parameter fields is extended by the ECS system by implementing the `SystemFunction` trait for it.

- This `SystemFunction` trait adds a `run()` method to the function, hence when the function is passed into a function, it can have this run method.

- The `SystemFunction` uses the world pointer to create  `SystemParam` extractor fields based on the function declaration. This is done using the `init` method in the `SystemParam` trait.

- `SystemParam` trait will have a init method which will take in a `&mut World` type to get full and free access into the way (We may change the type to `&World` if we do not need to call a mut function)




- But this `SystemFunction` is not stored as a raw type (since the type of function is not really deterministic since it is determined by the parameters that the user defined, due to which we cannot directly create a vector and store it), but is rather going to be stored in a `FunctionHolder`, which will implement a `System` type, using which I can execute the system later with appropriate fields 
(TODO: Details of this need to be worked out)
















# The Event System (Heavily inspired by Bevy)
### Creating Interface:
1. The Event system interface can simply be a function that can be called on the world type to push a event on the world. This event can then be processed in the next update cycle of the ECS system.

2. The events can be send into the ECS system through EventReaders and EventWriters (similar to bevy).






### Processing Events Internally.
Following is the plan to implement the Event system
- An `Event<E>` schema will contain:
    - TypeId of user event struct
    - user event in a Box structure
- Event will be created by `EventManager`, which will contain all events of the frame.
- `EventManager` will flush the events after the update call.
- `EventManager` will contain events in 2 storages:
    - **events in previous frame:** These are events that might have been launched in the frame of the previous update, or are events that might have been launched by the event handler in the previous frame. This is the vector that will be processed in the current frame, and the one which `EventReader<E>` will read from.
    - **events in current frame:** These are events that are being generated in the current update function or event handler and will get processed in the next frame. This is the vector that `EventWriter<E>` will write events to.

- `EventReader<E>` is like an iterator, a structure which acts like it owns the data but really doesn't (will contain a `PhantomData<E>` field). This is a structure which will only contain a reference of the world event Vec to read events from and fetch events based on the TypeId of `E`

- `EventReader<E>` will enable events to be read across several event handlers at the same time concurrently (HOW?: Event trait can be Send + Sync (How will that help?)) because the array that is read is different from the array that is written to. Since `EventReader<E>` only reads the vector, it can be accessed from multiple threads.

- User can create handler events by function schema: 

```
        fn name(event: EventReader<E>) {
            ...
        }
```


Some things to think about are

##### Do we need all the components to handle a all types of events, or do we need to specify information in the Event schema to define the dependencies?
**Ans.** This question is invalid now. The flaw in previous design was that the component systems and events are 2 different components, and hence they cannot be coupled without facing huge design issues. We are changing the system API to be functional, giving us high modularity, and in this system event handlers will be seperate Systems in themselves.

##### Dependencies will have a sender and a receiver, so how do we enforce the type of the sender
**Ans.** We dont really have dependencies in the new model. Events will be created and read by handling functions through thread safe structures (EventReaders and EventWriters)

##### How do we prevent the user from creating cyclic dependencies of component types?
**Ans.** Again, no longer possible in the new system since the event handling function is a seperate system and does not have the concept of dependencies (Events launched in the event hanbd)

##### What metadata do we need to keep in the Event Schema regarding the event? 
**Ans.** None really, we only need the type id of the Event being stored, which could just be a function in the base Event trait which could be implemented at runtime by a simple derive.

##### Do we allow user to put in custom user data in their Custom Events? If so, how do we get the type definition at the user end to convert them into valid types? 
**Ans.** In the new model, custom data by the user is an inherent trait really. the `EventReader<E>` is going to read only events of type `E` and the `event_reader.read_events()` function will return the event 

##### How do we handle events launched in the event handler function?




Processing events should involve the following steps.
1. We can have an Event trait with type definitions. These type definitions can then be checked by a bool function, and we supply the event to the system only if the the system's input type is the same as the type mentioned in the incoming event. This way not every system will need to deal with every event, removing the load of parsing events on the user side and saving execution time. But this will only work for component specific events.
What we really need to do is actually get all the systems with functions taking in the event type. Problem is right now we are storing systems as structs, hence we cannot really create arbituary functions and plug them in the world and find the functions with the required parameter type. (The question is can we even do that with Fn traits?)

I think this is again a problem of coupling 2 unrelated components of the system. The thing is that events and component systems are 2 different design flows, and a component system should not be responsible for handling event flows. If an event is generated, a event system should be created to handle the event. This event handling logic should not be specific to a component or a resource, although this event logic should be capable of modifying components or resources in the world, hence we should allow user data to be attached on events to enable the user to attach custom data to the events that they create.


(Think more about how this will sit in play with the entire system.)







# Schedule system
In this ECS system, we have defined `flow`s which can be initiated by the App interface to update the state of the world. These flows are defined by different type of systems, however the number of flows are fixed (for now, we might want a way to make this arbituary). Each of the flow is initiated by a function defined in the `BaseSystem` trait and is implemented by the derive call to a type of system.

For scheduling, each of the flow defined in the App interface has to have a seperate scheduling graph, which defines a node as a executor function and dependency systems. This way we can parallelize execution of independent systems to achieve better performance.





# Independent Flows
Currently, the different flows are hardcoded in the App interface and correspondingly in the `BaseSystem` interface as well. The base system interface then calls the appropriately named function in the trait definition to process the flow. We may want to make this process of flow definition an invariable thing by declaring each flow as a trait, and then each inteface function in the `Flow` trait would call the appropriate function in the struct implementation.

The main question here is that do we really need this at all? Are we really going to have a variable number of flows or are we going to pre define it for our frame based requirement?
Defining custom flow might help us take a step towards applying the ECS systems to projects other than games where we can define the flow of our tasks explicitly. 

These flows can then be scheduled independently based on a defined criteria.










# REFERENCES:
EntityId generation based on it's index in the array itself using generational index (Source: Bevy ECS) - [Generational Index article](https://lucassardois.medium.com/generational-indices-guide-8e3c5f7fd594)
