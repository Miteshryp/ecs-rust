use std::any::Any;

pub trait ECSBase {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
    fn downcast_to_ref<T: ECSBase + Sized + 'static>(&self) -> &T where Self: Sized + 'static;
    fn downcast_to_ref_mut<T: ECSBase + Sized + 'static>(&mut self) -> &mut T where Self: Sized + 'static;
}
