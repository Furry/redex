use super::traits::StdConversions;

#[derive(Debug, Clone, PartialEq)]
pub struct CompoundString {
    pub store: String
}

impl CompoundString {
    pub fn new() -> CompoundString {
        CompoundString {
            store: String::new()
        }
    }

    pub fn from(value: String) -> CompoundString {
        CompoundString {
            store: value
        }
    }
}

impl StdConversions for CompoundString {
    fn to_integer(&self) -> super::Integer {
        super::Integer::from(self.store.parse::<f64>().unwrap())
    }

    fn to_compound_string(&self) -> CompoundString {
        CompoundString {
            store: self.store.clone()
        }
    }

    fn to_bool(&self) -> super::Bool {
        super::Bool::from(self.store.len() > 0)
    }
}