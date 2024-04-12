# ECS System Project
### Goal - To use this library in the vulkan based game engine to enable easier flow and integration of different (possibly independent) components in the game engine.

Another additional goal of this project is to actually explore how this architecture of designing systems can be used outside of game development in general purpose software engineering.


# PRIMARY

[x] Investigate the potential memory bug
    We are accessing components in the component system using a RefCell, which returns a Ref or RefMut (depending on accessor).
    What is supposed to happen if within a frame, when there are multiple references given out to extractors, on of the system calls a `remove_entity` or `remove_component_from_entity` function while other read references are still alive? This could result in a crash. 
    `remove_entity` requires a mutable world, but as we know every system practically has a mutable access to the world parallely, which is a problem in this case.

    We need to think about component removal mechanism. Maybe we can have another SystemParam (WorldRef) which can push an entity to be deleted into an array in the world, and then at the start of the next frame we delete those entities.

    Taking a page from bevy's book, we are going to be updating the world through command queues. This decision comes with the realisation that it is practically impossible to safely edit the world while multiple systems are running in parallel


[] Redesign the System API to be functional

    [] Write the Docs for SystemExecutor architecture


    [x] Implement data extractors
        @TODO Test the implementation

        - Design a dependency design model for systems
        
        - Create Schedules which can identify system dependency
            1. Identify dependency
            2. Place the dependency higher up in the chain and keep the dependents lower
            3. Start executing from the top.
            4. Ensure that lock acquisition are done in a transaction using semaphores

        - [] Design and write a ComponentHandle extractor


    [] Find a design which allows us to restrict the type of system (single run, event?, etc) 
    (This should be controlled as an option in a Schedule (systems have to be in a schedule, then this schedule is responsible for determining at what frequency the systems in the schedule should run, much like bevy_ecs))

[] Redesign Schedule flow to an acceptable solution.
    - We are facing a problem where we cannot have nested 
    schedules with the API we have for schedulables (The schedulable API is suited for systems only).
    We need to find a solution to that.


[x] Design Event flow (See notes.md -> Processing Events Internally)

[] Implement the Design flow

[] Write tests for all data extractors

# Optional

[] Implement a debugging system for internal types.



# Research

[] Impact of RwLocks on performance

[] Learn to put attributes in derive macros for system derives


# Optimizations
    [] Explore the option to implement the sparse map version for optimizing memory. IMPORTANT: Carefully analyze the pros and cons and implement only if it actually benefits performance in the end.
    (This uses hashmap, but it also depends on the implementation. Bevy used a custom Sparse HashMap because the TypeId itself actually stores a u128, which I am guessing produces dispursed values when the code is compiled.)

    [] Do a Detailed Algorithmic analysis of structures used after making the entire system functional