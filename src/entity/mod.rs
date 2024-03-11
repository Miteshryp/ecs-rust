pub mod entity_manager;

use std::{fmt::Debug, hash::Hash};



/// The repr declaration is necessary to facilitate better code generation
/// by LLVM for functions which might need to convert the struct into 
/// a series of bits. (This is the explanation given in the bevy source code,
/// but they have a function to bits for the entity id)
#[repr(C, align(8))]
#[derive(Debug)]
pub struct Entity {
    index: u32,
    generation: u32
}

impl Entity {
    // /// Supposed to be useful for optimizing codegen for things like
    // /// PartialOrd le, gt, ge operators.
    // /// But since I do not understand this at this point in time(10th march 2023),
    // /// I will not proceed with to_bits approach for the moment
    // /// Once I completely understand the nuances, I will proceed with the approach
    // /// mentioned in the bevy_ecs code.
    // pub fn to_bits(&self) -> u64 {
    //     (self.index as u64) << 32 | (self.generation as u64)
    // }
}
impl Clone for Entity{
    fn clone(&self) -> Self {
        Self { index: self.index.clone(), generation: self.generation.clone() }
    }
}
impl Copy for Entity {}

impl PartialEq for Entity {
    fn eq(&self, other: &Self) -> bool {
        self.index == other.index && self.generation == other.generation
    }
}
impl Eq for Entity {}

impl Hash for Entity{
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.index.hash(state);
        self.generation.hash(state);
    }
}