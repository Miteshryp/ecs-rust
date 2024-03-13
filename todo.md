# ECS System Project
### Goal - To use this library in the vulkan based game engine to enable easier flow and integration of different (possibly independent) components in the game engine.

Another additional goal of this project is to actually explore how this architecture of designing systems can be used outside of game development in general purpose software engineering.


# PRIMARY

[] Learn to put attributes in derive macros for system derives

[] Think about how can we execute systems in parallel by identifying dependencies of a system.
[] Design the system scheduler for task deployment. First, analyze if it is really needed. I am thinking in a parallel setting it can be useful.
[] Think about creating different types of systems

# Additional Features (Future Plans)
[] Design a solution to event system.
[] Add Schedules similar to bevy. These schedules will run different systems (I think).
[] Explore possibility of execution graphs to enable multithreded ECS

# Optimizations
    [] Explore the option to implement the sparse map version for optimizing memory. IMPORTANT: Carefully analyze the pros and cons and implement only if it actually benefits performance in the end.
    (This uses hashmap, but it also depends on the implementation. Bevy used a custom Sparse HashMap because the TypeId itself actually stores a u128, which I am guessing produces dispursed values when the code is compiled.)