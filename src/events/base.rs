use std::{any::TypeId};

use crate::ecs_base::ECSBase;


// Base event type. To be included using a derive macro
// @TODO: Create event derive macro
pub trait Event: ECSBase {
    fn type_id() -> TypeId where Self: Sized + 'static {
        TypeId::of::<Self>()
    }
}

#[macro_export]
macro_rules! receiver_systems {
    ($($type:ty),*) => {
        vec![$(std::any::TypeId::of::<$type>()),*]
    };
}
