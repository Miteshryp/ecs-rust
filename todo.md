# ECS System Project
### Goal - To use this library in the vulkan based game engine to enable easier flow and integration of different (possibly independent) components in the game engine.

Another additional goal of this project is to actually explore how this architecture of designing systems can be used outside of game development in general purpose software engineering.


# PRIMARY

[x] Write Extractors

    [x] Interaction between 2 different types of component (this is a set cross product of components specified in the extractor type.) 

    [x] Write iterators for Query and QueryMut extractors
    [x] Write individual Component extractor iterators

[x] Review the parallel execution code

[] Write and rewrite Docs for Literally everything

    - There have been too many architectural changes since the last proper documentation update.

[] Cleanup codebase from un-needed comments

[] Find some way to report the internal dependency bug at compile time
    
    - Might be possible using attribute based macros. We can try to create assert statements inside the function to point out the issue with dependency

[] Think about implementing our own type system
    - TypeId is 128 bits, which is an overkill
    - It might be the case that higher number of bits offer a better resistance to hash collisions, but that we have to research about


[] Think about removal of lock mechanisms
    
    - We have now implemented a dependency based DAG which is able to find unrelated components and execute them in parallel. Since this is now a predicate for every schedule, we can safely say that the proper access of the world resources is now the responsibility of the Schedule, not the world. 
    Hence, it might be wise to move this responsibility completely into schedule. This would make it easier to debug and read the logic in the future.
    
    It could pose some challenges such as rust borrow checker issues to store the resource references, etc. This could only be figured out by actually writing the refactor 


[] Find a solution for multiple world problem
    - Each world has resources, which are shared among scenes, but also
    contains components and systems, which differ across scenes 

[] Think if we need a multiple path dependency injection

    - A parent node can have multiple defined child paths
    - The child paths can be in any order relative to each other in a paralle execution, but they must be in the defined serial order with respect to their common parent node.

[] Rigorously test the parallel scheduling system
    [x] Found event system bug, fixed

[] Integrate testing tools into the project (miri, turmoil, etc)


[x] Design and write a ComponentHandle extractor.
    - This is named ComponentCollection and ComponentCollectionMut

[] Redesign Schedule flow to an acceptable solution.

    [x] Implement relative ordering for systems in parallel schedules
    
    @NOTE: Not doing this, since schedules are only responsible for scheduling. It's the system holder which has some relevance in execution frequency, hence it contains the frequency API.
    [x] Implement frequency trigger for schedules.
    

    - @NOTE: We are not implementing the schedule merge logic right now as it might not be necessary in a lot of cases for a game engine, since most of the systems are going to have to be placed to optimize computation time, hence we should rely upon the parallel scheduler to implement scheduling for us, and keep the dependency injection to a minimum. This is also the reason that we have not designed a divergent path for dependent systems (Parent A is executed before C and B in a relative order, but B and C have no relative order and can execute in parallel)

    - We are facing a problem where we cannot have nested 
    schedules with the API we have for schedulables (The schedulable API is suited for systems only).
    We need to find a solution to that.
    - We need to implement merging of different schedules
        1. Parallel-parallel merge
        2. Parallel-serial merge
        3. Serial-serial merge
 

[x] Design Event flow (See notes.md -> Processing Events Internally)

[] Think about adding a schedule flow dedicated to handling events, which is going to be executed in a specific order.

[] Write tests for all data extractors


# Optional

[] Implement a debugging system for internal types.
    - Think about the design a bit


# Research

[] Impact of RwLocks on performance

[] Learn to put attributes in derive macros for system derives


# Optimizations
    [] Explore the option to implement the sparse map version for optimizing memory. IMPORTANT: Carefully analyze the pros and cons and implement only if it actually benefits performance in the end.
    (This uses hashmap, but it also depends on the implementation. Bevy used a custom Sparse HashMap because the TypeId itself actually stores a u128, which I am guessing produces dispursed values when the code is compiled.)

    [] Do a Detailed Algorithmic analysis of structures used after making the entire system functional
