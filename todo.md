# ECS System Project
### Goal - To use this library in the vulkan based game engine to enable easier flow and integration of different (possibly independent) components in the game engine.

Another additional goal of this project is to actually explore how this architecture of designing systems can be used outside of game development in general purpose software engineering.


# PRIMARY

[] Write and rewrite Docs for Literally everything

    - There have been too many architectural changes since the last proper documentation update.


[x] Fix internal dependency bug
    - [] Test if this is fixed.

[x] Fixing the dependency metadata fetching API (The metadata that we receive right now does not indicate the type of access - immutable/mutable, since we are simply returning a set of dependencies rn. We need to change this)

[x] Decoupling metadata and runtime dependency objects from the `SystemDependency` struct.


@TODO: Test this implementation vigorously
[x] Find a way to insert a node in DAG. (Topological sort.)
    - Sort based on in-degree
    - When inserting a new node, check for dependencies from lowest to highest in-order. At every level of iteration, remove the nodes with indegree 0 and proceed to the next iteration with updated in-degrees.
    - If the node to be inserted does not clash with a node of 0 in-order in an iteration, insert it with them and exit the insertion.




[x] Redesign the System API to be functional

[] Design and write a ComponentHandle extractor.

[] Redesign Schedule flow to an acceptable solution.

    [] Implement relative ordering for systems in parallel schedules
    [] Implement frequency trigger for schedules.
    
    - We are facing a problem where we cannot have nested 
    schedules with the API we have for schedulables (The schedulable API is suited for systems only).
    We need to find a solution to that.
    - We need to implement merging of different schedules
        1. Parallel-parallel merge
        2. Parallel-serial merge
        3. Serial-serial merge


[x] Design Event flow (See notes.md -> Processing Events Internally)

[] Implement the Designed Event flow

[] Write tests for all data extractors

[] Write Extractors
    [] Interaction between 2 different types of component (this is a set cross products of components specified in the extractor type.) 

# Optional

[] Implement a debugging system for internal types.



# Research

[] Impact of RwLocks on performance

[] Learn to put attributes in derive macros for system derives


# Optimizations
    [] Explore the option to implement the sparse map version for optimizing memory. IMPORTANT: Carefully analyze the pros and cons and implement only if it actually benefits performance in the end.
    (This uses hashmap, but it also depends on the implementation. Bevy used a custom Sparse HashMap because the TypeId itself actually stores a u128, which I am guessing produces dispursed values when the code is compiled.)

    [] Do a Detailed Algorithmic analysis of structures used after making the entire system functional
