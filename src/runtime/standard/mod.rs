pub mod io;

use crate::runtime::Callable;

// Re-export output
pub use self::io::output::Print;
pub use self::io::output::PrintLine;
pub use self::io::input::ReadLn;

use std::collections::HashMap;
use std::thread::Scope;

use super::primitives::VariableStorage;
