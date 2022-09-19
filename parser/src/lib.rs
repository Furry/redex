#![feature(generator_trait)]

pub mod modules;
pub mod structures;

#[macro_use]
extern crate derive_new;

pub struct Branch {
    left: Box<Branch>, 
    right: Option<Box<Branch>>
}

pub trait Evaluable<T> {
    fn evaluate(&self) -> T;
}
pub trait MaybeEvaluable<T> {
    // todo: Custom error type.
    fn try_evaluate(&self) -> Result<T, ()>;
}

