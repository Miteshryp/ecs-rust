use std::{any::{Any, TypeId}, ops::DerefMut};
use super::param::SystemParam;

pub struct SystemMetadata {
    // Collection of metadata about the world resources being 
    // accessed by all system params collectively.
    // TypeId stored here is the id of world based resources,
    // not the Id of SystemParam type.
    pub(crate) dependency_metadata: hashbrown::HashMap<TypeId, bool>
}
impl Clone for SystemMetadata {
    fn clone(&self) -> Self {
        Self { dependency_metadata: self.dependency_metadata.clone() }
    }
}

impl SystemMetadata {
    pub fn new() -> Self {
        Self {
            dependency_metadata: hashbrown::HashMap::new(),
        }
    }

    // Checking with another system's metadata for conflicts
    pub fn is_resource_clashing(&self, other_metadata: &SystemMetadata) -> bool {
        let other_dependencies = other_metadata.get_world_resource_dependencies();
        
        for (rid, is_mut) in other_dependencies {
            if self.dependency_metadata.contains_key(&rid) {
                if *self.dependency_metadata.get(&rid).unwrap() || is_mut {
                    return true;
                }
            }
        }

        return false;
    }

    pub fn push_dependency_metadata<S: SystemParam + 'static>(&mut self) {
        // Internal dependency check
        let world_resource_ids = S::get_resource_access_type();
        for rid in world_resource_ids {
            if self.dependency_metadata.contains_key(&rid) {
                // check for collision based on type of access
                // mut-mut collision
                // mut-read collision

                if *self.dependency_metadata.get(&rid).unwrap() 
                    || 
                S::is_resource_access_mut() {
                    panic!("Internal dependency error found")
                }
            } else {
                self.dependency_metadata.insert(rid, S::is_resource_access_mut());
            }
        }
    }

    ///
    /// ### Description
    /// 
    /// Returns the [TypeId]s of world based resources being
    /// accessed by the system along with the type of access.
    /// 
    /// This metadata is used by schedule to properly conduct 
    /// system execution without conflicts.
    pub fn get_world_resource_dependencies(&self) -> hashbrown::HashMap<TypeId, bool> {
        self.dependency_metadata.clone()
    }
}



// @TODO: Document
pub struct SystemDependencies {
    // Collection of parameters which store a lock
    dependencies: hashbrown::HashMap<TypeId, Box<dyn Any>>,
}

impl SystemDependencies {

    pub fn new() -> Self {
        Self {
            dependencies: hashbrown::HashMap::new(),
            // world_access_types: hashbrown::HashMap::new()
        }
    }
    
    // pub fn push_dependency_metadata<S: SystemParam + 'static>(&mut self) {
    //     // Internal dependency check
    //     let world_resource_ids = S::get_resource_access_type();
    //     for rid in world_resource_ids {
    //         if self.world_access_types.contains_key(&rid) {
    //             // check for collision based on type of access
    //             // mut-mut collision
    //             // mut-read collision

    //             if *self.world_access_types.get(&rid).unwrap() 
    //                 || 
    //             S::is_resource_access_mut() {
    //                 panic!("Internal dependency error found")
    //             }
    //         } else {
    //             self.world_access_types.insert(rid, S::is_resource_access_mut());
    //         }
    //     }
    // }

    pub fn push_dependency<S: SystemParam + 'static>(&mut self, dep_param: S) {
        // @TODO: Maybe check for internal dendency error here itself?

        if !self.dependencies.contains_key(&S::type_id()) {
            self.dependencies.insert(S::type_id(), Box::new(dep_param));
        } else {
            let err_string = "Multiple dependencies of the same type cannot exist in a system";
            log::error!("{err_string}");
            panic!("{err_string}");
        }
    }

    /// ### Description
    /// 
    /// Used to get dependencies for a system. Fetching a dependency
    /// which does not exist in the function will result in a crash
    pub fn pop_dependency<S: SystemParam + 'static>(&mut self) -> Box<S> {
        match self.dependencies.remove(&TypeId::of::<S>()) {
            Some(mut x) => {

                // Downcasting to appropriate type of box
                match x.downcast::<S>() {
                    Ok(boxed) => boxed,
                    Err(_) => {
                        let error_str = r#"Box downcast failed in pop_dependencies: 
                        Fatal Error occured.
                        "#;

                        log::error!("{error_str}");
                        panic!("{error_str}");
                    },
                }
            },
            None => panic!("Dependency TypeId does not match with the System Param"),
        }

        //@TODO: Understand what to do with the world_access_types here
    }

    
    /// Returns the Set of TypeIds of System Param, not their 
    /// underlying resources.
    // pub fn get_dependencies(&self) -> Vec<TypeId>{
    pub fn get_system_param_dependencies(&self) -> hashbrown::HashSet<TypeId> {
        self.dependencies.keys().into_iter().cloned().collect()
    }

    // // @TODO: Return the type of access along with this
    // // Cannot just return keys, since multiple read accesses 
    // // into a world resource is totally legal.
    // // But if we check for conflicts using just resource intersection,
    // // it is going to give wrong results.
    // pub fn get_world_resource_dependencies(&self) -> hashbrown::HashSet<TypeId> {
    //     self.world_access_types.keys().into_iter().clone().collect()
    // }
}