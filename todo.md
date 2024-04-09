# ECS System Project
### Goal - To use this library in the vulkan based game engine to enable easier flow and integration of different (possibly independent) components in the game engine.

Another additional goal of this project is to actually explore how this architecture of designing systems can be used outside of game development in general purpose software engineering.


# PRIMARY

[] Investigate the potential memory bug
    We are accessing components in the component system using a RefCell, which returns a Ref or RefMut (depending on accessor).
    What is supposed to happen if within a frame, when there are multiple references given out to extractors, on of the system calls a `remove_entity` or `remove_component_from_entity` function while other read references are still alive? This could result in a crash. 
    `remove_entity` requires a mutable world, but as we know every system practically has a mutable access to the world parallely, which is a problem in this case.

    We need to think about component removal mechanism. Maybe we can have another SystemParam (WorldRef) which can push an entity to be deleted into an array in the world, and then at the start of the next frame we delete those entities.

    Taking a page from bevy's book, we are going to be updating the world through command queues. This decision comes with the realisation that it is practically impossible to safely edit the world while multiple systems are running in parallel

[x] Seperate out Serial and Parallel SystemFunction executors

[] Redesign the System API to be functional
    
    [x] Figure out a way to pass functions with specific structure as arguments into our own functions. (Functions binded to SystemFunction trait)

    [x] Solve the concurrency issue for parallel systems (See notes - Critical issue regarding parallel systems)
        - We have stored an acquisition mutex lock in the world. The `acquire_acquisition_lock` function allows a system runner to get the acquision lock from the world. No system runner can proceed to acquiring world state without acquiring this acquisition mutex. This is placed in order to prevent a potential starvation in 2 parallel systems. (S1 has A and wants B, S2 has B and wants A, then none of the systems execute).

    [x] Write macro implementation of SystemFunction in order to allow multiple user defined parameters(implementing SystemParam) into the function. 
    (
        We dont wanna allow direct user defined types to be a system param, but can we can allow the user to directly implement system param? 
        We can, but what is the utility of that? There are only resources, components or entities to be fetched from the world.
    

        SystemParam -> initialise (*mut World).
        Giving the user a world pointer is veryyyy risky.
        No, user cant have a pointer.
    )

    [x] Allowing multiple types of parameters into the function will have extra load of fetching those resources from the World, based on user parameters that we do not know. Find a solution to this problem. 
    (We restrict the types of SystemParams by predefining them.
    These types in turn can fetch an arbitrarily defined type (which is valid for the struct) from the world)

    [x] Think about a way to store the acquired SystemFunction in the app struct as a system.

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
        
        - [x] Write Resource extractors
            -  [x] ResourceHandle -> accesses resources immutably
            -  [x] MutResourceHandle -> accesses resources mutably
        
        - [x] Design and Write Query extractor
            - [] Decide if we need Query to be of different types (Mutable and immutable)

        - [] Design and write a ComponentHandle extractor


    [] Find a design which allows us to restrict the type of system (single run, event?, etc) 
    (This should be controlled as an option in a Schedule (systems have to be in a schedule, then this schedule is responsible for determining at what frequency the systems in the schedule should run, much like bevy_ecs))

[] Design and write a CommandBuffer writer for Scheduler, where a CommandBuffer is going to implement a SystemParam trait

[x] Figure out how to store functions with different parameters in the world struct. (Bind each struct that we want to allow to be passed as a SystemParam. This trait will be implemented automatically for different ECS structs through derive macros.)

[x] Design Event flow (See notes.md -> Processing Events Internally)

[x] Think about how can we execute systems in parallel by identifying dependencies of a system. (System Dependency graphs as DAGs, each independent system is executed parallely in arbituary order).

[] Learn to put attributes in derive macros for system derives

# Research

[] Impact of RwLocks on performance



# Optimizations
    [] Explore the option to implement the sparse map version for optimizing memory. IMPORTANT: Carefully analyze the pros and cons and implement only if it actually benefits performance in the end.
    (This uses hashmap, but it also depends on the implementation. Bevy used a custom Sparse HashMap because the TypeId itself actually stores a u128, which I am guessing produces dispursed values when the code is compiled.)
