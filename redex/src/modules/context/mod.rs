pub mod lowlevel;

use self::lowlevel::LowLevel;

use super::datatypes::Value;

pub struct Context {
    pub lowlevel: LowLevel
}

impl Context {
    pub fn new() -> Self {
        Self {
            lowlevel: LowLevel::new()
        }
    }
}