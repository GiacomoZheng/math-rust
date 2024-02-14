use std::fmt::Debug;
pub trait BasicObject : Debug + Clone + PartialEq + Eq {}
pub trait BasicObjectCopy : BasicObject + Copy {}

impl BasicObject for char {}

pub use general_marco::{BasicObject, BasicObjectCopy};