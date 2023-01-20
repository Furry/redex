use std::cmp::Ordering;

use super::traits::StdConversions;

// Represents an unbounded integer value.
#[derive(Debug, Clone)]
pub struct Integer {
    pub store: Vec<u8>
}

impl Integer {
    pub fn new() -> Integer {
        Integer {
            store: Vec::new()
        }
    }

    pub fn from(value: i64) -> Integer {
        let mut integer = Integer::new();
        integer.store = value.to_be_bytes().to_vec();
        integer
    }

    pub fn from_bytes(bytes: Vec<u8>) -> Integer {
        let mut integer = Integer::new();
        integer.store = bytes;
        integer
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        self.store.clone()
    }

    pub fn to_string(&self) -> String {
        let mut string = String::new();
        for byte in self.store.iter() {
            string.push_str(&byte.to_string());
        }
        // Remove all leading zeros
        string.trim_start_matches('0').to_string()
    }

    // Math Expressions
    pub fn add(&self, other: &Integer) -> Integer {
        let mut carry = 0;
        let mut result = Vec::new();
        let mut i = 0;
        while i < self.store.len() || i < other.store.len() {
            let mut sum = carry;
            if i < self.store.len() {
                sum += self.store[i];
            }
            if i < other.store.len() {
                sum += other.store[i];
            }
            carry = sum / 10;
            result.push(sum % 10);
            i += 1;
        }
        if carry > 0 {
            result.push(carry);
        }
        Integer::from_bytes(result)
    }

    pub fn multiply(&self, other: &Integer) -> Integer {
        let mut result = Integer::from(0);
        let mut i = 0;
        while i < other.store.len() {
            let mut carry = 0;
            let mut product = Vec::new();
            let mut j = 0;
            while j < i {
                product.push(0);
                j += 1;
            }
            j = 0;
            while j < self.store.len() {
                let mut sum = carry;
                sum += self.store[j] * other.store[i];
                carry = sum / 10;
                product.push(sum % 10);
                j += 1;
            }
            if carry > 0 {
                product.push(carry);
            }
            result = result.add(&Integer::from_bytes(product));
            i += 1;
        }
        result
    }

    pub fn divide(&self, other: &Integer) -> Integer {
        let mut result = Integer::from(0);
        let mut remainder = Integer::from(0);
        let mut i = self.store.len();
        while i > 0 {
            i -= 1;
            remainder = remainder.multiply(&Integer::from(10));
            remainder = remainder.add(&Integer::from(self.store[i] as i64));
            let mut quotient = Integer::from(0);
            let mut j = 0;
            while j <= 9 {
                let product = Integer::from(j).multiply(other);
                if product > remainder {
                    break;
                }
                quotient = Integer::from(j);
                j += 1;
            }
            result = result.multiply(&Integer::from(10));
            result = result.add(&quotient);
            remainder = remainder.subtract(&quotient.multiply(other));
        }
        result
    }

    pub fn subtract(&self, other: &Integer) -> Integer {
        let mut borrow = 0;
        let mut result = Vec::new();
        let mut i = 0;
        while i < self.store.len() || i < other.store.len() {
            let mut difference = borrow;
            if i < self.store.len() {
                difference += self.store[i];
            }
            if i < other.store.len() {
                difference -= other.store[i];
            }
            if difference < 0 {
                borrow = u8::MAX;
                difference += 10;
            } else {
                borrow = 0;
            }
            result.push(difference);
            i += 1;
        }
        Integer::from_bytes(result)
    }
}

// Impl eq, partial eq, and cmp for Integer
impl PartialEq for Integer {
    fn eq(&self, other: &Integer) -> bool {
        self.to_string() == other.to_string()
    }
}

impl Eq for Integer {}

// Impl partial ord for Integer
impl PartialOrd for Integer {
    fn partial_cmp(&self, other: &Integer) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// Impl cmp
impl Ord for Integer {
    fn cmp(&self, other: &Integer) -> Ordering {
        self.to_string().cmp(&other.to_string())
    }
}

// StdConversions
impl StdConversions for Integer {
    fn to_integer(&self) -> Integer {
        self.clone()
    }

    fn to_compound_string(&self) -> super::CompoundString {
        super::CompoundString::from(self.to_string())
    }

    fn to_bool(&self) -> super::Bool {
        if self.to_string() == String::from("0") {
            super::Bool::from(false)
        } else {
            super::Bool::from(true)
        }
    }
}