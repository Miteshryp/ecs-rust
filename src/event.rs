use std::{any::TypeId, collections::HashMap};

use crate::ecs_base::ECSBase;

pub trait ECSevent {}

#[macro_export]
macro_rules! receiver_systems {
    ($($type:ty),*) => {
        vec![$(std::any::TypeId::of::<$type>()),*]
    };
}
