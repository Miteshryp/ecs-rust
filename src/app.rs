use std::{cell::RefCell, rc::Rc};

use crate::{component::Component, system::{BaseSystem, ComponentSystem}, world::World};

pub struct App {

    world: Rc<RefCell<World>>,
    systems: Vec<Box<dyn BaseSystem>>,
}

impl App {
    pub fn new() -> Self {
        App {
            // world: RefCell::new(World::new()),
            // world: Rc::new(World::new()),
            world: Rc::new(RefCell::new(World::new())),
            systems: vec![]
        }    
    }

    pub fn add_component_system<Sys>(&mut self, system: Sys)
    where
        Sys: BaseSystem + ComponentSystem + 'static,
        <Sys as ComponentSystem>::ComponentType: Component + 'static
    {
        self.systems.push(Box::new(system));
        self.world.borrow_mut().register_component::<<Sys as ComponentSystem>::ComponentType>();
    }

    // @TODO: Add different types of systems
    // @TODO: Add schedules functionality

    pub fn start(&mut self) {
        for system in &mut self.systems {
            system.process_start(self.world.clone());
        }

        self.world.borrow_mut().set_active(true);

        while self.world.as_ref().borrow().is_active() {
            self.update();
        }
    }

    pub fn update(&mut self) {
        for system in &mut self.systems {
            system.process_update(self.world.clone());
        }
    }
}