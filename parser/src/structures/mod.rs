pub mod math;

pub struct Branch<T> {
    pub leaf: Option<T>,
    pub left: Box<Branch<T>>,
    pub right: Option<Box<Branch<T>>>
}

pub trait Branchable {
    
}