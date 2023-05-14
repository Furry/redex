use super::traits::StdConversions;

#[derive(Debug, Clone, PartialEq)]
pub struct Bool {
    pub store: bool
}

impl Bool {
    pub fn new() -> Bool {
        Bool {
            store: false
        }
    }

    pub fn from(value: bool) -> Bool {
        Bool {
            store: value
        }
    }
}

impl StdConversions for Bool {
    fn to_integer(&self) -> super::Integer {
        if self.store {
            super::Integer::from(1)
        } else {
            super::Integer::from(0)
        }
    }

    fn to_compound_string(&self) -> super::CompoundString {
        super::CompoundString::from(self.store.to_string())
    }

    fn to_bool(&self) -> Bool {
        Bool {
            store: self.store
        }
    }
}