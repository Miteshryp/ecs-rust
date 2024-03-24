# ECS System Project
### Goal - To use this library in the vulkan based game engine to enable easier flow and integration of different (possibly independent) components in the game engine.

Another additional goal of this project is to actually explore how this architecture of designing systems can be used outside of game development in general purpose software engineering.


# PRIMARY

[] Redesign the System API to be functional
    
    [x] Figure out a way to pass functions with specific structure as arguments into our own functions. (Functions binded to SystemFunction trait)

    [] Solve the concurrency issue for parallel systems (See notes - Critical issue regarding parallel systems)

    [] Write macro implementation of SystemFunction in order to allow multiple user defined parameters(implementing SystemParam) into the function. 
    (We dont wanna allow direct user defined types to be a system param, but can we can allow the user to directly implement system param? 
    
    SystemParam -> initialise (&mut world).
    Giving the user a mutable reference is kindof risky imo.
    @TODO: Think about this some more.)

    [x] Allowing multiple types of parameters into the function will have extra load of fetching those resources from the World, based on user parameters that we do not know. Find a solution to this problem. 
    (We restrict the types of SystemParams by predefining them.
    These types in turn can fetch an arbitrarily defined type (which is valid for the struct) from the world)

    [x] Think about a way to store the acquired SystemFunction in the app struct as a system.

    [] Design a way to store systems in the World

    [] Implement data extractors

        @TODO Test the implementation
        - [x]  Write read and write functions for EventReaders and EventWriters. These read and write methods are responsible for:
            1. Going through the vec to see if the type matches the one specified to the reader
            2. If yes, convert that type to the required local type and 
            return
            3. In case of writer, the writer should construct the Event and send it as a Box
        - [x] Write Event implemting macros - ECSBase + Event
        
        - [x] Write Resource extractors
            -  [x] ResourceHandle -> accesses resources immutably
            -  [x] MutResourceHandle -> accesses resources mutably
        
        - [] Design and Write Query extractor

    [] Find a design which allows us to restrict the type of system (single run, event?, etc) 
    (This should be controlled as an option in a Schedule (systems have to be in a schedule, then this schedule is responsible for determining at what frequency the systems in the schedule should run, much like bevy_ecs))

[x] Figure out how to store functions with different parameters in the world struct. (Bind each struct that we want to allow to be passed as a SystemParam. This trait will be implemented automatically for different ECS structs through derive macros.)

[x] Design Event flow (See notes.md -> Processing Events Internally)

[x] Think about how can we execute systems in parallel by identifying dependencies of a system. (System Dependency graphs as DAGs, each independent system is executed parallely in arbituary order).

[] Learn to put attributes in derive macros for system derives

# Research

[] Impact of RwLocks on performance



# Optimizations
    [] Explore the option to implement the sparse map version for optimizing memory. IMPORTANT: Carefully analyze the pros and cons and implement only if it actually benefits performance in the end.
    (This uses hashmap, but it also depends on the implementation. Bevy used a custom Sparse HashMap because the TypeId itself actually stores a u128, which I am guessing produces dispursed values when the code is compiled.)
