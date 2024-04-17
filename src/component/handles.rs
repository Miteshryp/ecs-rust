use std::{ops::{Deref, DerefMut}};
use tokio::sync::{OwnedRwLockReadGuard, OwnedRwLockWriteGuard};

use crate::entity::Entity;

use super::Component;



/// 
/// ### Description
/// 
/// This Handle represents an immutable access acquired into a
/// specific component from the world.
/// 
/// This handle is an interface to be used by the user to gain
/// immutable access into a component inside a [`system`](crate::System) function.
/// 
pub struct ComponentHandle<C: Component + 'static> {
    inner: OwnedRwLockReadGuard<C>,
    entity_id: Entity,
}

impl<C: Component + 'static> ComponentHandle<C> {
    pub fn new(lock: OwnedRwLockReadGuard<C>, entity_id: Entity) -> Self {
        ComponentHandle {
            entity_id,
            inner: lock
        }
    }
}
impl<C: Component + 'static> Deref for ComponentHandle<C> {
    type Target = C;
    
    fn deref(&self) -> &Self::Target {
        self.inner.deref()
    }
}




/// 
/// ### Description
/// 
/// This handle represents a mutable access acquired into
/// specific component from the world. 
/// This handle is an interface object used by the user
/// to mutate the world components in systems
/// 
pub struct MutComponentHandle<C: Component + 'static> {
    inner: OwnedRwLockWriteGuard<C>,
    entity_id: Entity,
}
impl<C: Component + 'static> MutComponentHandle<C> {
    pub fn new(lock: OwnedRwLockWriteGuard<C>, entity_id: Entity) -> Self {
        MutComponentHandle {
            inner: lock,
            entity_id
        }
    }
}
impl<C: Component + 'static> Deref for MutComponentHandle<C> {
    type Target = C;
    
    fn deref(&self) -> &Self::Target {
        self.inner.deref()
    }
}
impl<C: Component + 'static> DerefMut for MutComponentHandle<C> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.inner.deref_mut()
    }
}



///
/// ### Description
/// 
/// This is the wrapper struct which allows us to access a 
/// [`component`](Component) lock tuple acquired using the 
/// [crate::CrossComponentCollectionMut] [`system param`](crate::system::param::SystemParam)
/// 
/// This type implements the [Deref] trait to allow us to 
/// directly access the underlying component.
/// 
pub struct ComponentRefHandle<'a,C: Component + 'static> {
    pub(crate) inner: &'a OwnedRwLockReadGuard<C>,
    pub(crate) entity_id: Entity,
    // @TODO: Add the entity_id access here
}
impl<C: Component + 'static> Deref for ComponentRefHandle<'_, C> {
    type Target = C;
    
    fn deref(&self) -> &Self::Target {
        self.inner.deref()
    }
}




///
/// ### Description
/// 
/// This is the wrapper struct which allows us to access a 
/// [Component] lock tuple acquired using the [crate::CrossComponentCollectionMut]
/// [`system param`](crate::system::param::SystemParam)
/// 
/// This type implements the [Deref] and [DerefMut] trait to 
/// allow us to directly access and modify the underlying 
/// component.
/// 
pub struct MutComponentRefHandle<'a,C: Component + 'static> {
    pub(crate) inner: &'a mut OwnedRwLockWriteGuard<C>,
    pub(crate) entity_id: Entity,
    // @TODO: Add the entity_id access here
}
impl<C: Component + 'static> Deref for MutComponentRefHandle<'_, C> {
    type Target = C;
    
    fn deref(&self) -> &Self::Target {
        self.inner.deref()
    }
}
impl<C: Component + 'static> DerefMut for MutComponentRefHandle<'_, C> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.inner.deref_mut()
    }
}
