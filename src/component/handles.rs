use std::{ops::{Deref, DerefMut}};
use tokio::sync::{OwnedRwLockReadGuard, OwnedRwLockWriteGuard};

use super::Component;



/// ### Description
/// 
/// This Handle represents an immutable access acquired into a
/// specific component from the world.
/// This handle is an interface to be used by the user to gain
/// immutable access into a component inside a system function.
pub struct ComponentHandle<C: Component + 'static> {
    inner: OwnedRwLockReadGuard<C>
}

impl<C: Component + 'static> ComponentHandle<C> {
    pub fn new(lock: OwnedRwLockReadGuard<C>) -> Self {
        ComponentHandle {
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




/// ### Description
/// 
/// This handle represents a mutable access acquired into
/// specific component from the world. 
/// This handle is an interface object used by the user
/// to mutate the world components in systems
pub struct MutComponentHandle<C: Component + 'static> {
    inner: OwnedRwLockWriteGuard<C>,
}
impl<C: Component + 'static> MutComponentHandle<C> {
    pub fn new(lock: OwnedRwLockWriteGuard<C>) -> Self {
        MutComponentHandle {
            inner: lock
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



// @TODO: Document
pub struct CrossComponentHandle<'a,C: Component + 'static> {
    pub(crate) inner: &'a OwnedRwLockReadGuard<C>,
}
impl<C: Component + 'static> Deref for CrossComponentHandle<'_, C> {
    type Target = C;
    
    fn deref(&self) -> &Self::Target {
        self.inner.deref()
    }
}




// @TODO: Document
pub struct CrossMutComponentHandle<'a,C: Component + 'static> {
    pub(crate) inner: &'a mut OwnedRwLockWriteGuard<C>,
}
impl<C: Component + 'static> Deref for CrossMutComponentHandle<'_, C> {
    type Target = C;
    
    fn deref(&self) -> &Self::Target {
        self.inner.deref()
    }
}
impl<C: Component + 'static> DerefMut for CrossMutComponentHandle<'_, C> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.inner.deref_mut()
    }
}
