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
