use std::any::{Any, TypeId};

pub trait ECSBase {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
    fn type_id() -> TypeId
    where
        Self: Sized + 'static,
    {
        TypeId::of::<Self>()
    }
}
