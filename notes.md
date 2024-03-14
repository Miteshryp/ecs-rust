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








# OUTDATED QUESTIONS

### Is this System responsible for keeping coordination between the component and the entity to which it is attached?

This question has 2 potential solutions I can think of right now. The first is something where both the entity system and component system keep a record of the number of components attached to an entity, and both operate independently to update their own record.
The flaw in this design is that I might actually not have the need to keep the record of components in the entity system (I don't even know for sure if there is really a need for an entity system)

### Why do we need an Entity System?

- To keep track of what components are attached to an entity.
- Entity creation and management api
    1. create entity
    2. attach components to entities
    
- Catch entity events? (What exactly are events in this?)
- For establishing a system where 2 components in a single entity can communicate with one another.
    - Component communication should happen through events for an entity.
    - Entity system should be responsible for launching as well as handling events.
    - Events must be able to be launched by the Component Handler as well. (This is how a component will be able to initiate events)





# VALID QUESTIONS

### What does a System do?

1. In laymens terms, we can say that system is something that updates a component. It defines the behavior for updating component data and handling interaction between components of similar types.
 
### How do we ID entities [Unanswered]

A few ways to do this. 
- First is that we can try to generate uuid for the entity. The flaw with this is that the uuid is a 128 bit struct, which might increase the memory cost
- The second option is that we can put a serial generator in the EntityManger structure. The flaw with this approach might come when we want to work on multiple threads, as the serial generator will not be locally cached in the cache row, which could lead to serious performance hits due to consistency.

Going for a hardware dependent option is a better approach in this case since it will free us of some metadata present in the EntityManager, not to mention the option for us to generate entities in parallel units.
Although, thinking about this more, I realise that the entity manager is a single object in a world, which has to be accessed by all parallel units anyways. So since there is a single point of control anyways, it might not make much sense to do this. 
But if we make id generation dependent on local units, the sync overhead might be minimized.


### Why can't we attach system (handlers) to the component managers

The system needs the following attributes as argument into its methods for proper functioning
- Owner World's API access.
- List of components of similar type for handling interactions.
- entity_id access for the owner entity.
- current component to be acted on

Now, if system is bounded into the component manager, the world API would end up containing the managers. This leads to a problem where we are unable to pass the parent structure as an argument in the member functions because the `&mut self` cannot have multiple instances.
If we try to solve this using just `self`, we still run into another issue where we are still not able to pass the `World` into the member function because `self` is partially decomposed (because we accessed entity_manager), hence the world struct is not bound to be valid.

The solution to this problem is to distinctly seperate out the state of the world from its logic, and pass the state into the logic during updates






## Exploring the problem encountered in the vulkan-engine
ECS framework defined should help us to get better modularity in the project.
One of the current problems that I was facing was not able to attach a Renderer to the VulkanInstance after constructing the instance. The reason that was happening was that the renderer required references to data in the vulkan instance struct, since vulkan instance contained several global parameters necessary for all rendering processes (such as swapchain images, framebuffer indexes, logical device, etc).

This is because we cannot really store a reference to the vulkan instance object in the renderer because we cannot define a lifetime for it.
Furthermore, we are also not able to pass the entire instance object reference in the render() function call because we are storing the vulkan renderer inside the vulkan instance, and calling a method on the renderer partially borrows the instance object, hence the instance object cannot really be passed into the renderer. Hence, we run into this cyclic issue which is difficult to solve.

The problem in this scenario is high coupling of data and logic. If we take a step back and think about the goal of each component, we can simplify the components as  `VulkanInstance` containing the state of the vulkan instance, and the `DeferredRenderer` being a logical component which provides output as a function of an input which is this vulkan state. 
However, rust borrow checker forces us to design programs with modularity, which we are not doing by enclosing the renderer in the vulkan state itself.

Hence to solve this, we can take advantage of the ECS system that we have created to generalize this plugin pattern for all coupling problems. 
In this case, we can think of both state (VulkanInstance) and renderer and the renderer as 2 different components (actually resource because we only have single instance of the 2 things, but we'll implement that in the future in the ECS framework) where renderer is updated by taking this vulkan state as an input.

The problem that is gonna arise is that some changes in the vulkan state might need to trigger a change to resource. In this sense, these are dependent resource, hence we actually need an event system in the ECS system, which can automatically launch handlers for dependent resources.
If Resource B is dependent on Resource A, then any change to resource A should launch a handler on Resource B, which can change resource B as it sees fit by looking at the Resource A in the system. 
(This will have to be implemented using the event system. An event can have type arguments which defines anchor and dependent resources. Any change to the anchor resource should hence push an event with that resource as an anchor, and then the event should be processed with the anchor and dependent resource as parameter to the handler) 

# Resources
Resources are just components in the world which do not have any parent entity. These 'resources' will have to be identified by a special `ResourceId` since they do not have any parents.
Since they are also independent, we do not need to enforce their storage in a contiguous memory block, and hence can be stored in hashmaps (or sparsesets in bevy) to store the Resources.

Just like components can be fetched using the id of the entity they are attached to, resources will be fetched using `ComponentId`s.
We also need to keep checks in place to ensure that there are not more than one instance of a single resource type.


# Implementing the Event System
### Creating Interface:
1. The Event system interface can simply be a function that can be called on the world type to push a event on the world. This event can then be processed in the next update cycle of the ECS system.

2. The events can be send into the ECS system through EventReaders and EventWriters (similar to bevy), but I need to assess the benefits of this approach. One possible benefit of this system is that we may access the event buffer in the world across different threads, but we would anyways need a lock mechanism to push the event on the world event buffer.

### Processing Events Internally.
Some things to think about are
- Do we need all the components to handle a all types of events, or do we need to specify information in the Event schema to define the dependencies?
- Dependencies will have a sender and a receiver, so how do we enforce the type of the sender (through derive implementations)
- How do we prevent the user from creating cyclic dependencies of component types?
- What metadata do we need to keep in the Event Schema regarding the event? 
- Do we allow user to put in custom user data in their Custom Events? If so, how do we get the type definition at the user end to convert them into valid types? 
    - (derive implementations).
    - Resource definitions - Resources are literally single existing data, so we may allow the user to attach a resource type trait to the event, and we can define the resource derive macro to extract data from the resource in the right format (with carefully cut out safety) and pass the user data into the handlers.

Processing events should involve the following steps.
1. We can have a Event trait with type definitions. These type definitions can then be checked by a bool function, and we supply the event to the system only if the the input type is the incoming event. This way not every system will need to deal with every event, removing the load of parsing events on the user side and saving execution time.
(Think more about how this will sit in play with the entire system.)








# REFERENCES:
EntityId generation based on it's index in the array itself using generational index (Source: Bevy ECS) - [Generational Index article](https://lucassardois.medium.com/generational-indices-guide-8e3c5f7fd594)