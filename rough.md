# Pros of structural system

- Ability to declare system specific state
- All handlers are present in a single place
- Better intellisense support due to fixed system type declarations

# Cons of structural system
- Right now creating a new flow requires manually doing the following
    - Defining flow initiating function in App.
    - Declare the flow handling function in the BaseSystem.
    - Creating a macro for type of system to handle the flow. 
    - High data and function coupling. (Can fireback in future, as experienced in vulkan-test)



# Pros of functional system
- Type independent declaration
- Suitable function names
- Potentially better modularity (Functions are not coupled together, which could benefit us in future designs)
- Gives us ability to define custom flows easily (Only need to declare a flow initiating function, and add it to the world.) 
-   



# Bevy Functional Systems
- `(increase_counter, print_counter)` -> Implements IntoSystemConfig<Marker>, which has into_configs function
- `(increase_counter, print_counter)` -> Implements IntoSystem<In, Out, Marker>
- `(increase_counter, print_counter)` -> Implements System
    
- `SystemParamFunction` is implemented for `FnMut`, `Fn`, [function_system.rs, line 620, 634]
- `FunctionSystem` is a struct which stores a system function
- `into_system` function creates and returns a `FunctionSystem` struct, which stores a `SystemParamFunction` field, which is the function that we supplied
- this is getting called while creating system configs in into_configs function.

- `SystemParamFunction` implements the `System` trait as well to inherit basic system properties [function_system.rs, line 443]
- `func.run` is called to execute the funtion. This run is executed in the 

- `into_configs` -> gives system configs


# Scheduling
- Serial systems should be able to contain Parallel systems, and vice-versa
- Both types of systems should also be able to directly contain their appropriate executors (SerialSystem and ParallelSystem), so they should implement Schedulable trait

- We need to design a seperate function holder for parallel systems, which 