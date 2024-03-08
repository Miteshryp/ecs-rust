# ECS System Project
### Goal - To use this library in the vulkan based game engine to enable easier flow and integration of different (possibly independent) components in the game engine.


# PRIMARY

[x] Determine the design of the project
    [x] Prepare a list of structures required in the ECS setting
    [x] Learn how they interact with one another
    [x] Learn how we can create and store objects of a type by their TypeId 
    
    [x] Seperate out systems from entity managers

    
    [x] Define a consistent API for handlers

    (This uses hashmap, but it also depends on the implementation. Bevy used a custom Sparse HashMap because the TypeId itself actually stores a u128, which I am guessing produces dispursed values when the code is compiled.)
    
[] Complete API for the world class
[] Solve entity id generation issue
[] Write API Documentation
[] Learn to put attributes in derive macros for system derives

# Additional Features (Future Plans)
[] Add Schedules similar to bevy
[] Explore possibility of execution graphs to enable multithreded ECS
[] Create different types of systems

[x] Implement derivable traits across the project (Need to learn how to handle TokenStream first)

# Optimizations
    [] Explore the option to implement the sparse map version for optimizing memory. IMPORTANT: Carefully analyze the pros and cons and implement only if it actually benefits performance in the end.