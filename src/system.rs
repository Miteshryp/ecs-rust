


pub struct SystemId {
    id: u128
}
impl Eq for SystemId {}

/// Contains the interface for implementing the logic for handling a specific type of component
pub struct System<Component: Sized> {
    system_id: SystemId,
    components: Vec<Component>,
}

impl<Component: Sized> System<Component> {

    /// System initialisation function. Used to create a new system to handle the specified type of component
    pub fn new() -> System<Component> {
        System {
            components: vec![]
        }
    }

    /// 
    pub fn add_component_to_system(&mut self) {

    }


    /// Returns the id of the component being handled by the system.
    /// This function is used to recognize whether the given system is suitable
    /// for handling the component in question.
    pub fn get_component_id(&self) -> SystemId {
        self.system_id
    }
}