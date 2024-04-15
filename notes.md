# CORE IDEAS
- Entity is just a wrapper for components which itself is just an id. This id is unique to entity.

- Components are structures which contain data which is contiguously allocated in memory, and all components of a single type are updated together contiguously, leading to better cache locality.

- Systems are the logical unit for everything in the World and define the functionality of the App

- A given entity can only have one instance of a specific type of component.

- These Systems are attached to the App, which launches these systems at appropriate times.

- The World object is the state of the entire ECS system, whereas the systems attached to the world define the behavior of the system.



# IMPLEMENTATION IDEAS

- Since each entity can only have one instance of any given component, the component can be identified through their entity_id in the component manager.

- Do we want to allow entities to be attached to other entities.

- We have functional systems in the design. These allow us to have some advantages
    - It highly decouples logic containers across the codebase, which helps us keep code modularized

    - It optimizes execution since now only functions which contain logic can be executed instead of each flow of each system (in the older design, even if the flow handler had no code to be executed, the derive for every system had to manually go through each dynamically dispatched system, which is highly wasteful)


- Functional systems allow us to take `SystemParam` types into the function as parameters. Once these functions are inserted into a schedule in order to run on a World, these parameters automatically fetch the required resource locks from the world and supply them to the 
system function through in built methods.

Each type of system param may have different behavior based on what purpose it serves. 
ex: `ResourceHandle` only supplies a single resource, whereas a `Query` or `QueryMut` component supplies multiple things. `Query` is a system param which gets us a list of entities with the required components along with the components attached to the entities.
We can also have interactive systems which supply us an iterator over the cross product of 2 different types of component sets.


- We must also implement mechanisms to ensure the following:
    1. One component instance can only belong to a single entity. For this, each component needs to keep track of the entity its currently attached to. This I think will have to be the responsibility of a component manager

    2. There should only exist one system to handle a single type of component or entity in the world. Hence the world class needs to make sure it only creates new systems for entities or components which are being created for the first time.








# QUESTIONS
 
### How do we ID entities

We create `EntityId`s using the concept of generational indexes (see reference for more), which allows us to reuse existing spots in the entity array without clashing with the old id.







# Resources
Resources are just components in the world which do not have any parent entity. These 'resources' will have to be identified by a special `ResourceId` since they do not have any parents.

Since they are also independent, we do not need to enforce their storage in a contiguous memory block, and hence can be stored in hashmaps (or sparsesets in bevy) to store the Resources.

Just like components can be fetched using the id of the entity they are attached to, resources will be fetched using the Type of Resource required.
We also need to keep checks in place to ensure that there are not more than one instance of a single resource type.



# Functional Systems
### Design
- A functional system is any function defined in the scope whose parameters implement the `SystemParam` type.

- These `SystemParam`s are extractors responsible for extracting the state values from a world instance in a safe manner for execution on schedules.

- These `extractor` parameters are going to be declared by ECS system and can be used by the user to extract state data from the assigned world to be used in a system function.

### Critical issue regarding parallel systems


##### Problem
Currently, all the `SystemParam`s that need to be passed into a system are being initialised everytime run is called on the system, which means that all `SystemParam`s try to initialise every frame.

But this could also cause trouble leading to data race conditions.

However, we cannot really create a RwLock on the world, since that would essentially force the entire design to be sequential, since the initialise function requires a `&mut World`, and that would require every system to secure a lock to the world before execution, hence essentially making the system sequential.


##### Solution adapted
To solve this, we have to allow multiple `&mut World`s, but we have to ensure that the internal state remains consistent throughout, meaning that if 2 systems are accessing the same resource, they should be `relatively sequential`

In simple words, each resource defined in the World structure needs to be individualistically locked using RwLocks. `Extractors` will then own the locks returned by the extraction function.

No 2 conflicting extractors should run at the same time to cause lock owning conflict. This is the responsibility of a Scheduler. 





### Functional system architecture

- Any function declared with compatible parameter fields is extended by the ECS system by implementing various traits (`SystemMarker`, `SystemExtractor` and `SystemExecutor`) for it.

- This `SystemExecutor` trait adds a `run()` method to the function, hence when the function is stored in a schedule, it can have this run method to run the function

- The `SystemExtractor` uses the world pointer to initialise  `SystemParam` extractor fields based on the function declaration. This is done using the `initialise` method in the `SystemParam` trait.

- `SystemParam` trait will have a `initialise` method which will take in a `&mut World` type to get full and free access into the world.




- But this `SystemMarker` is not stored as a raw type (since the type of function is not really deterministic since it is determined by the parameters that the user defined, due to which we cannot directly create a vector and store it), but is rather going to be stored in a `System` (which is just a structure to hold a function and its data), which will implement a `Schedulable` type, using which I can add the system to a schedule.





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


##### Dependencies will have a sender and a receiver, so how do we enforce the type of the sender
**Ans.** We dont really have dependencies in the new model. Events will be created and read by handling functions through thread safe structures (EventReaders and EventWriters)

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
All the systems are stored in the `App` struct, which stores these systems inside a `Schedule` type object. This schedule is primarily responsible for ensuring that each function parameter initialises in a safe manner.

These schedules themselves are then held inside App using a `SystemHolder` struct. The system holder then sequentially runs these schedule in the order tha the `SystemHolder`s were registered in. 
A `SystemHolder` can be registered in an App using the `register_flow` function. The holders (or flows) registered first will have a priority in execution.

At the end of each flow execution, the command buffer is flushed and changes are made to the world which were initiated inside the system.

// @TODO: Extend the document after finalising and completing the frequency feature.



# Parallel Dependency Graph based Scheduling
- The dependency graph will expose a single layer at a time, which will consist of all the nodes which have an in-degree of zero
- What this means is that the systems in that layer do not conflict with any other system which has pending execution
- So this means that these systems could have parallel access into the world and still ensure that no 2 resources in the world are being accessed at the same time. Hence, these systems are safe to run in parallel.
- Hence, given that this assertion stays true, we can let the `UnsafeWorld` type be a `Send + Sync` type

- This dependecy graph can also contain injected dependecies (Forcing a system to execute after another one) for 2 systems even if there is no conflict between them.

## Task List
[] Internal Type Id system for world resources
[] Create a bloom filter based on dependencies for a system.
[] Design a struct to store dependency graph (acyclic)
    - An enclosing DAG struct
    - Definition of a node
        - system (func + dependencies)
    - Definition of an edge
        - Find a conflict in dependencies.
        - Find any enforced dependencies.
[] Construct that dependency graph for all systems in a schedule.
[] Traverse the graph and initialise-run the systems
    - Find the nodes with in-degree 0
    - Execute and remove those nodes
    - Update the graph to find new in-degrees

There can only be one schedule running at any given time, since the context (essentially the graph, which ensures that parallel access to the world is safe) is local to a schedule

## Internal Dependency Conflicts
@TODO: Write a detailed note




# REFERENCES:
EntityId generation based on it's index in the array itself using generational index (Source: Bevy ECS) - [Generational Index article](https://lucassardois.medium.com/generational-indices-guide-8e3c5f7fd594)
