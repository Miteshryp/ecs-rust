# ECS System Project
### Goal - To use this library in the vulkan based game engine to enable easier flow and integration of different (possibly independent) components in the game engine.


# PRIMARY

[] Determine the design of the project
    [x] Prepare a list of structures required in the ECS setting
        - World -> Stores all the objects created in the ECS environment 
        - ComponentHandler -> Responsible for managing and updating a specific type of component.
        - Component -> A data item which can be attached to an entity. This can then be updated by its appropriate system.
        - ComponentSystem -> creates, registers, handles events for a specific type of component defined by its TypeId.
        - Entity -> A wrapper for a collection of components.
        - EntityManager?
    [] Learn how they interact with one another
    [] Learn how we can create and store objects of a type by their TypeId (This uses hashmap, but it also depends on the implementation. Bevy used a custom Sparse HashMap because the TypeId itself actually stores a u128, which I am guessing produces dispursed values when the code is compiled.)
    
# Optimizations
    [] Explore the option to implement the sparse map version for optimizing memory. IMPORTANT: Carefully analyze the pros and cons and implement only if it actually benefits performance in the end.

[] Implement derivable traits across the project (Need to learn how to handle TokenStream first)