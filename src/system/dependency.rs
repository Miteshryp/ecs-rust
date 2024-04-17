use super::param::SystemParam;
use std::any::{Any, TypeId};






/// ### Description
/// 
/// Structure used to keep a record of the underlying resources
/// being accessed by the system in a GraphNode
/// 
/// [SystemMetadata] differs from [crate::system::dependency::SystemDependencies]
/// is that [SystemMetadata] stores information about the world-based resources
/// being accessed by the system (which is used to check for cross dependency conflicts
/// as well as internal dependencies)
/// whereas the [SystemDependencies] struct stores [SystemParam] level information
/// (which means the typeids of [SystemParam]s themselves)
pub struct SystemMetadata {
    /// Collection of metadata about the world resources being
    /// accessed by all system params collectively.
    /// 
    /// TypeId stored here is the id of world based resources,
    /// not the Id of SystemParam type.
    pub(crate) dependency_metadata: hashbrown::HashMap<TypeId, bool>,
}
impl Clone for SystemMetadata {
    fn clone(&self) -> Self {
        Self {
            dependency_metadata: self.dependency_metadata.clone(),
        }
    }
}

impl SystemMetadata {
    pub fn new() -> Self {
        Self {
            dependency_metadata: hashbrown::HashMap::new(),
        }
    }

    /// ### Description
    /// 
    /// Cross checks with another node's [SystemMetadata] to find
    /// and report conflict.
    /// If any common resource is found in one of the following states
    /// in the 2 [SystemMetadata]s, we can consider them to be in conflict:
    /// 
    /// 1. Mutable - Mutable Conflict
    /// 2. Read - Mutable or Mutable - Read Conflict
    pub fn is_resource_clashing(&self, other_metadata: &SystemMetadata) -> bool {
        let other_dependencies = other_metadata.get_world_resource_dependencies();

        for (rid, is_other_mut) in other_dependencies {
            if self.dependency_metadata.contains_key(&rid) {
                let is_self_mut = *self.dependency_metadata.get(&rid).unwrap();
                if is_self_mut  || is_other_mut {
                    return true;
                }
            }
        }

        return false;
    }

    /// ### Description
    /// 
    /// Used to update the record of world-based resource accesses the 
    /// system requires. 
    /// This function is called in the variadic parameter definition of
    /// functions defined in the macro definition of 
    /// [`SystemExtractor`](crate::system::SystemExtractor) for different
    /// [`Func`](crate::system::SystemMarker)s
    /// 
    /// This function is responsible for 2 main things
    /// 1. Keeping a track record of dependency for scheduler to check and
    /// schedule the system accordingly
    /// 2. Checking and reporting any possible internal dependency conflicts
    /// See notes.md for more on Internal Dependency Error
    pub fn push_dependency_metadata<S: SystemParam + 'static>(&mut self) {

        // Internal dependency check
        let world_resource_ids = S::get_resource_access_type();
        for rid in world_resource_ids {
            if self.dependency_metadata.contains_key(&rid) {
                // check for collision based on type of access
                // mut-mut collision
                // mut-read collision

                if *self.dependency_metadata.get(&rid).unwrap() || S::is_resource_access_mut() {
                    panic!("Internal dependency error found")
                }
            } else {
                self.dependency_metadata
                    .insert(rid, S::is_resource_access_mut());
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




/// ### Description
/// 
/// A struct used to store information about the acquired 
/// resource locks of world based resources
/// 
/// This structure is supposed to store locks temporarily inside
/// [`system parameter`][SystemParam] types.
/// Hence the locks must be freed using [SystemDependencies::pop_dependency]
/// API once their usage is complete
pub struct SystemDependencies {
    /// Collection of parameters which store a 
    /// lock to world based resource
    dependencies: hashbrown::HashMap<TypeId, Box<dyn Any>>,
}

impl SystemDependencies {
    pub fn new() -> Self {
        Self {
            dependencies: hashbrown::HashMap::new(),
        }
    }


    /// ### Description
    /// 
    /// Adds a system parameter into the dependencies if it does not
    /// conflict with a stored  [`dependency parameter`](SystemParam)
    /// 
    /// The method panics if duplicated system param insertions are
    /// found.
    pub fn push_dependency<S: SystemParam + 'static>(&mut self, dep_param: S) {
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
    /// Used to relieve dependencies from a system. 
    /// Fetching a dependency which does not exist in the function will result in a crash
    pub fn pop_dependency<S: SystemParam + 'static>(&mut self) -> Box<S> {
        match self.dependencies.remove(&TypeId::of::<S>()) {
            Some(x) => {
                // Downcasting to appropriate type of box
                match x.downcast::<S>() {
                    Ok(boxed) => boxed,
                    Err(_) => {
                        let error_str = r#"Box downcast failed in pop_dependencies: 
                        Fatal Error occured.
                        "#;

                        log::error!("{error_str}");
                        panic!("{error_str}");
                    }
                }
            }
            None => panic!("Dependency TypeId does not match with the System Param"),
        }
    }
}

