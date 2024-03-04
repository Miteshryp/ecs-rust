use std::any::{Any, TypeId};

use super::component::Component;


pub trait SystemHandler<Comp>
where
    Comp: Component + Sized,
    Self: 'static + Sized
{
    // fn update();
    fn new() -> Self;

    fn get_id() -> TypeId {
        TypeId::of::<Self>()
    }
}




/// Contains the interface for implementing the logic for handling a specific type of component
pub struct ComponentSystem<Comp, Handler>  
where
    Comp: Component + Sized,
    Handler: SystemHandler<Comp>
{
    system_id: TypeId,
    components: Vec<Comp>,

    handler: Handler
}

impl<Comp, Handler> ComponentSystem<Comp, Handler>
where
    Comp: Component + Sized + 'static,
    Handler: SystemHandler<Comp> + 'static
 {

    /// System initialisation function. Used to create a new system to handle the specified type of component
    pub fn new() -> Self {
        ComponentSystem {
            system_id: TypeId::of::<Self>(),
            components: vec![],
            handler: Handler::new()
        }
    }

    /// 
    pub fn add_component_to_system(&mut self, component: Comp) {
        self.components.push(component)
    }


    /// Returns the id of the component being handled by the system.
    /// This function is used to recognize whether the given system is suitable
    /// for handling the component in question.
    pub fn get_component_id(&self) -> TypeId {
        // self.system_id
        TypeId::of::<Comp>()
    }

    pub fn get_system_id(&self) -> TypeId {
        self.system_id
    }
}





// Testing code
pub struct TestComponent {
    i: u32
}
impl Component for TestComponent {}

pub struct TestSystemHandler;
impl SystemHandler<TestComponent> for TestSystemHandler {
    fn new() -> Self {
        Self {}
    }

    // fn update(TestComponent) {...}
}

fn test_main() {
    let mut system = ComponentSystem::<TestComponent, TestSystemHandler>::new();
    system.add_component_to_system(TestComponent {i: 23});
}