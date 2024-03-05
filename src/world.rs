use crate::{component::Component, system::{ComponentHandler, ComponentSystem, System}};


pub struct World {
    systems: Vec<Box<dyn System>>,
}

impl World {
    pub fn new() -> Self {
        Self {
            systems: vec![]
        }
    }

    pub fn register_component_with_handler<T, Handler>(&mut self, handler: Handler)
    where
        T: Component + Sized + 'static,
        Handler: ComponentHandler<T>
    {
        self.systems.push(Box::new(ComponentSystem::<T>::new::<Handler>()));
    }
}
