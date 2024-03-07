pub mod component_system; 

use std::any::TypeId;

pub trait Component {
    fn get_name() -> String where Self: Sized;
    fn into_component_type(&self) -> &Self where Self: Sized;
    fn into_component_type_mut(&mut self) -> &mut Self where Self: Sized;
}


pub trait ComponentHandler<Comp>
where
    Comp: Component,
    Self: 'static,
{
    // fn update();
    fn new() -> Self
    where
        Self: Sized;

    fn get_id() -> TypeId
    where
        Self: Sized,
    {
        TypeId::of::<Self>()
    }

    // What all do we need to update the component?
    fn on_update() {

    }
}