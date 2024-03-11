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




# REFERENCES:
EntityId generation based on it's index in the array itself using generational index (Source: Bevy ECS) - [Generational Index article](https://lucassardois.medium.com/generational-indices-guide-8e3c5f7fd594)