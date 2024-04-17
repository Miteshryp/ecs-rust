# ECS System Project
### Goal - To use this library in the vulkan based game engine to enable easier flow and integration of different (possibly independent) components in the game engine.

Another additional goal of this project is to actually explore how this architecture of designing systems can be used outside of game development in general purpose software engineering.


# PRIMARY



[] Prepare README.md
    
    [] Prepare a diagram for Event and Command Buffer behavior

    [] Prepare a diagram to explain schedules and schedule holder

    [] Prepare a diagram to give an overview of scheduling logic used by the parallel scheduler

[] Find a solution for multiple world problem
    
    - Each world has resources, which are shared among scenes, but also
    contains components and systems, which differ across scenes 
    - With this context, we'll first need to define what multiple world really means, only after which we can assess whether we really need it or not


[] Rigorously test the parallel scheduling system

    [] Integrate testing tools into the project (miri, turmoil, etc)

    [] Write tests for all data extractors


# Optional

[] Implement a debugging system for internal types.
    - Think about the design a bit

[] Think if we need a multiple path dependency injection

    - A parent node can have multiple defined child paths
    - The child paths can be in any order relative to each other in a paralle execution, but they must be in the defined serial order with respect to their common parent node.

[] Think about adding a schedule flow dedicated to handling events, which is going to be executed in a specific order.

[x] Think about removal of lock mechanisms
    
    @POST ANALYSIS
    This is not at all required, since all the locks stored maximum of a Semaphore(u32) and a pointer to the data.
    The OwnedLock returned by the world contains a pointer to the data in the vec. This will never cause cache locality issues since the entire vector would become available to the cache for processing of specific vectors if even one component is fetched from it.
    Hence, since the memory remains hot, the cache coherency exists.
    All a lock really does is just increase the size of each components by a few bytes, which I dont think is worth compromising a solid read-write based defense.

[] Think about implementing our own type system

    - TypeId is 128 bits, which is an overkill
    - It might be the case that higher number of bits offer a better resistance to hash collisions, but that we have to research about

[] Find some way to report the internal dependency bug at compile time

    - Might be possible using attribute based macros. We can try to create assert statements inside the function to point out the issue with dependency



# ABANDONED


[x] Redesign Schedule flow to an acceptable solution.

    @NOT DOING THE FOLLOWING    
    - We are facing a problem where we cannot have nested 
    schedules with the API we have for schedulables (The schedulable API is suited for systems only).
    We need to find a solution to that.
    - We need to implement merging of different schedules
        1. Parallel-parallel merge
        2. Parallel-serial merge
        3. Serial-serial merge



# Research

[x] Impact of RwLocks on performance

[] Learn to put attributes in derive macros for system derives




# Optimizations

[] Add different types of storage option by using a single storage object (Vec for contiguous, hashmap for high insert-delete, etc)

    [] Explore the option to implement the sparse map version for optimizing memory. IMPORTANT: Carefully analyze the pros and cons and implement only if it actually benefits performance in the end.
    (This uses hashmap, but it also depends on the implementation. Bevy used a custom Sparse HashMap because the TypeId itself actually stores a u128, which I am guessing produces dispursed values when the code is compiled.)

    [] Do a Detailed Algorithmic analysis of structures used after making the entire system functional