use std::cmp::Ordering;

use super::traits::StdConversions;

// Represents an unbounded integer value.
#[derive(Debug, Clone)]
pub struct Integer {
    pub store: f64
}

impl Integer {
    pub fn new() -> Integer {
        Integer {
            store: 0.0           
        }
    }

    pub fn from<T: Into<f64>>(i: T) -> Self {
        Integer {
            store: i.into()
        }
    }

    pub fn to_string(&self) -> String {
        self.store.to_string()
    }

    pub fn mult(&self, other: &Integer) -> Integer {
        Integer::from(self.store * other.store)
    }

    pub fn div(&self, other: &Integer) -> Integer {
        Integer::from(self.store / other.store)
    }

    pub fn add(&self, other: &Integer) -> Integer {
        Integer::from(self.store + other.store)
    }

    pub fn sub(&self, other: &Integer) -> Integer {
        Integer::from(self.store - other.store)
    }

    pub fn pow(&self, other: &Integer) -> Integer {
        Integer::from(self.store.powf(other.store))
    }

    pub fn modulo(&self, other: &Integer) -> Integer {
        Integer::from(self.store % other.store)
    }

}

impl PartialEq for Integer {
    fn eq(&self, other: &Self) -> bool {
        self.store == other.store
    }
}

impl StdConversions for Integer {
    fn to_integer(&self) -> Integer {
        Integer::from(self.store)
    }

    fn to_compound_string(&self) -> super::CompoundString {
        super::CompoundString::from(self.store.to_string())
    }

    fn to_bool(&self) -> super::Bool {
        super::Bool::from(self.store != 0.0)
    }
}