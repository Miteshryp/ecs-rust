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
    
    [x] Solve the concurrency issue for parallel systems (See notes - Critical issue regarding parallel systems)
        - We have stored an acquisition mutex lock in the world. The `acquire_acquisition_lock` function allows a system runner to get the acquision lock from the world. No system runner can proceed to acquiring world state without acquiring this acquisition mutex. This is placed in order to prevent a potential starvation in 2 parallel systems. (S1 has A and wants B, S2 has B and wants A, then none of the systems execute).
        - We have updated this design. We now have 2 different types of Schedules that can be incorporated in a world. Each of these 2 systems use different system executors (SystemFunction)

    [x] Write macro implementation of SystemFunction in order to allow multiple user defined parameters(implementing SystemParam) into the function. 
    (
        We dont wanna allow direct user defined types to be a system param, but can we can allow the user to directly implement system param? 
        We can, but what is the utility of that? There are only resources, components or entities to be fetched from the world.
    

        SystemParam -> initialise (*mut World).
        Giving the user a world pointer is veryyyy risky. (Only if we allow direct access of a state in the world, which we dont)
    )

    [] Write the Docs for SystemExecutor architecture

    [] Design a way to store systems in the World
        - Serial and Parallel Schedules are going to be system holders
        - The App will only contain an HashMap of these Schedules in every defined pass (update, render, etc)
        - @TODO: We need to decide whether the number of passes are configurable or predetermined

    [] Implement data extractors

        @TODO Test the implementation

        - Design a dependency design model for systems
        
        - Create Schedules which can identify system dependency
            1. Identify dependency
            2. Place the dependency higher up in the chain and keep the dependents lower
            3. Start executing from the top.
            4. Ensure that lock acquisition are done in a transaction using semaphores

        - [x]  Write read and write functions for EventReaders and EventWriters. These read and write methods are responsible for:
            1. Going through the vec to see if the type matches the one specified to the reader
            2. If yes, convert that type to the required local type and 
            return
            3. In case of writer, the writer should construct the Event and send it as a Box
        - [x] Write Event implemting macros - ECSBase + Event

        
        - [x] Design and Write Query extractor
            - [x] Decide if we need Query to be of different types (Mutable and immutable)
            (We do need that)

        - [] Design and write a ComponentHandle extractor


    [] Find a design which allows us to restrict the type of system (single run, event?, etc) 
    (This should be controlled as an option in a Schedule (systems have to be in a schedule, then this schedule is responsible for determining at what frequency the systems in the schedule should run, much like bevy_ecs))

[] Design and write a CommandBuffer writer for Scheduler, where a CommandBuffer is going to implement a SystemParam trait


[] Write a macro for implementing system param such that the implementor only needs to implement the initialise function, as the implementation of all other functions is common.

[x] Design Event flow (See notes.md -> Processing Events Internally)


# Research

[] Impact of RwLocks on performance

[] Learn to put attributes in derive macros for system derives


# Optimizations
    [] Explore the option to implement the sparse map version for optimizing memory. IMPORTANT: Carefully analyze the pros and cons and implement only if it actually benefits performance in the end.
    (This uses hashmap, but it also depends on the implementation. Bevy used a custom Sparse HashMap because the TypeId itself actually stores a u128, which I am guessing produces dispursed values when the code is compiled.)

    [] Do a Detailed Algorithmic analysis of structures used after making the entire system functional






git commit -m "Changes:
                                         - Changed the Component manager API with storage architecture
                                         - Changed the world API and resource storage architecture
                                         - Changed the macro definitions implemented on function objects
                                         - Added traits on functions
                                         - Added Schedule based traits
                                         - Added Fuzzy Parallel Scheduling
                                         - TODO: App has testing API field in struct. Delete them