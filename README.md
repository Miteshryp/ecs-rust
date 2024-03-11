# The ECS System Library in Rust

A framework for writing programs in accordance with the ECS (Entity Component System) paradigm commonly followed in game engines.

## Motivation
This is a sub-project which was initiated as a exploration while I was building my game engine in the rust programming language. Although ECS frameworks exist in the market (some very advanced than this currently aims to be), This project for me has been an excellent way to explore the internals of the rust programming language while at the same time challenging my previous designing skills by stepping into a new domain of API and system design.

# Guide

The ECS framework can easily be used by creating the `App` struct in the main runner function.

```
fn main( ) {
    let app = app.new();
    app.start();
}
```

The call to the `start` method kicks off an infinite loop which calls start, update (and possibly other sequences in the future) each iteration untill the world is set to inactive.

The code above is dormant since we haven't attached anything to the `app` instance that we created, so let see how to do that

### Creating Components
Creating components in this framework is as easy as deriving a macro on a struct that we have written out

```rust
#[derive(Component)]
struct NewComponent {
    inner_field: u32
}
```

This struct can now be used as a component in the ECS system. But so far, we have not seen any component interface right? As a matter of fact, there are no interfaces in the `app` instance that tell us about components! Well, to deal with components, we first have to learn to create systems.

### Creating Systems

As in any conventional ECS system, we control the components in the application using systems, which are kind of the logic handlers of the program. Hence, to direct our app to execute any logic, we first need to attach a system to it.

A system in ecs-rust can be easily created using the macros provided with the crate to make it compatible with the `App` class, and then implementing the `ComponentSystem` interface.

```rust
#[derive(ComponentSystem)]
struct NewSystem;
type ComponentType = NewComponent; // declared earlies

impl ComponentSystem for NewSystem {
    fn on_update(&self, world: &mut World, entity: Entity, &mut Comp) {
        // Called every frame
    } 

    fn on_start(&self, world: &mut World) {
        // Called only once at the start of the app
    }
}
```

A system is specific to a component (for now), hence we need to specify the type of component a system has to handle, which we do by specifying the type in the `ComponentType` type field in the `ComponentSystem` interface.

### The World API
As we can now see in our example that we finally have functions where we can access components existing in the world. But the question remains, how do we create these components and attach them to entities.

The answer to this lies in a new type exposed to us through the system interface: The `World` type.
The `World` is the state of the entire world in the progam, and the `World` struct provides us with mechanisms to modify, query and access the state of this world.

Hence, we can create entities in the world by calling `create_entity` on the world class, and the call the `add_component_to_entity` function on the world to attach a registered component to the world.

```rust
    fn on_start(&self, world: &mut World) {
        let new_entity = world.create_entity();
        world.add_component_to_entity::<NewComponent>(new_entity);
    }
```

If we run our program now, you might expect to see an entity being made in memory, but you would actually encounter a paniced crash saying that the "Component not registered for use: NewComponent".
Before we use any component in our program, we need to let the world know that we will be creating a component of that type in order to enable the `World` to create an appropriate manager for it.

We can also register a component type through the `App` interface at the start of the application, and can also dynamically register a component type through the `World` interface. (However currently since we need a system in the `App` instance to run user defined systems, we need to create the system at the start. Dynamic system addition might be added in the future )

```rust
fn main() {
    let app = App::new();
    app.add_component_system(NewSystem{});
}
```

With this, we have successfully registered a user defined system, created an entity in the `on_start` method, which we can modify every iteration by implementing logic in the `on_update` function.