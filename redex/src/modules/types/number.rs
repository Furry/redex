use bit_vec::BitVec;
pub struct Number {
    pub bits: BitVec
}

impl Number {
    pub fn new() -> Self {
        Self {
            bits: BitVec::new()
        }
    }

    pub fn from<T: Into<i128>>(value: T) -> Self {
        let mut number = Self::new();
        let mut value = value.into();
        while value != 0 {
            number.bits.push(value & 1 == 1);
            value >>= 1;
        }
        number
    }

    // pub fn mult(&self, other: &Self) -> Self {
    //     let mut number = Self::new();
    //     for (index, bit) in self.bits.iter().enumerate() {
    //         if bit {
    //             number.bits.or(&other.bits << index);
    //         }
    //     }
    //     number
    // }

    pub fn add(&self, other: &Self) -> Self {
        let mut number = Self::new();
        let mut carry = false;
        for (index, bit) in self.bits.iter().enumerate() {
            let other_bit = other.bits.get(index).unwrap_or(false);
            let mut result = bit ^ other_bit ^ carry;
            carry = (bit && other_bit) || (bit && carry) || (other_bit && carry);
            number.bits.push(result);
        }
        number
    }

    // pub fn sub(&self, other: &Self) -> Self {
    //     let mut number = Self::new();
    //     let mut carry = false;
    //     for (index, bit) in self.bits.iter().enumerate() {
    //         let other_bit = other.bits.get(index).unwrap_or(false);
    //         let mut result = bit ^ other_bit ^ carry;
    //         carry = (bit && other_bit) || (bit && carry) || (other_bit && carry);
    //         number.bits.push(result);
    //     }
    //     number
    // }

    // pub fn div(&self, other: &Self) -> Self {
    //     let mut number = Self::new();
    //     let mut remainder = Self::new();
    //     for (index, bit) in self.bits.iter().enumerate() {
    //         remainder.bits.push(bit);
    //         if remainder.bits.len() > other.bits.len() {
    //             remainder.bits.pop();
    //         }
    //         if remainder.bits.len() == other.bits.len() {
    //             if remainder.bits >= other.bits {
    //                 remainder.bits = remainder.bits.sub(&other.bits).bits;
    //                 number.bits.push(true);
    //             } else {
    //                 number.bits.push(false);
    //             }
    //         }
    //     }
    //     number
    // }

    pub fn into_i128(&self) -> i128 {
        let mut value = 0;
        for (index, bit) in self.bits.iter().enumerate() {
            if bit {
                value |= 1 << index;
            }
        }
        value
    }
}