pub mod component_system; 

use std::any::TypeId;

pub trait Component {
    fn get_name() -> String;
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
}