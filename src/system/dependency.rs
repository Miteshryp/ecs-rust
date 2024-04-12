use std::{any::{Any, TypeId}, ops::DerefMut};
use super::param::SystemParam;



// @TODO: Document
pub struct SystemDependencies {
    dependencies: hashbrown::HashMap<TypeId, Box<dyn Any>>
}

impl SystemDependencies {

    pub fn new() -> Self {
        Self {
            dependencies: hashbrown::HashMap::new()
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
    }

    pub fn push_dependency<S: SystemParam + 'static>(&mut self, dep_param: S) {
        if !self.dependencies.contains_key(&S::type_id()) {
            self.dependencies.insert(S::type_id(), Box::new(dep_param));
        } else {
            let err_string = "Multiple dependencies of the same type cannot exist in a system";
            log::error!("{err_string}");
            panic!("{err_string}");
        }
    }
}