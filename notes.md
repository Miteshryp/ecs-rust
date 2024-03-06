# CORE IDEAS
- Entity is just a wrapper for components which itself is just an id. This id is unique to entity.

- Components are structures which contain data which is contiguously allocated in memory, and all components of a single type are updated together contiguously.

- Components are updated by Systems, which are responsible for creating, managing and handling component object and event associated with them.

- World is a universal container which contains all the systems, entities and components within them. This world is what contains the main update loop to be called each frame.

# IMPLEMENTATION IDEAS

- Systems will have a common type of creation function, hence this function should take the type of the component rather than the component object, since if we take a component object created by the user and insert that into the system array, the components of different entities will not be contiguous in memory.

- This we could do by storing the typeid in a system and identifying whether the type supplied to us is the one which matches the one which that system wants.

- We should be able to attach handlers to each system, hence we need a standard format definition (defined function arguments) for a handler function.

- Handlers should be able to take in a component and perform an operation on it. The question remains how do we take and store this handler in a system struct?

- We must also implement mechanisms to ensure the following:
    1. One component instance can only belong to a single entity. For this, each component needs to keep track of the entity its currently attached to. This i think will have to be the responsibility of a system
    The question is now which system, an entity system or a component system?
    The aim of this mechanism is that if a component is removed from an entity, the component can be removed from memory. The converse however does not stand true, i.e. an empty or orphan entity can exist (one with no components) [Think if we should allow empty entities]

    2. There should only exist one system to handle a single type of component or entity in the world. Hence the world class needs to make sure it only creates new systems for entities or components which are being created for the first time.




# QUESTIONS

### What does a System do?

1. In laymens terms, we can say that system is something that updates a component. In more technical definition, a system is responsible for handling the state change event for a certain type of component.

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