// Implement a runtime error & a compile time error

use core::fmt;

#[derive(Debug, Clone)]
pub struct ErrorBody {
    message: String,
    pos: i32
}

#[derive(Debug, Clone)]
pub enum Error {
    RuntimeError(ErrorBody),
    CompileTimeError(ErrorBody)
}

// Implement a runtime error & a compile time error
impl Error {
    pub fn runtime<T: Into<i32>>(message: String, start: T) -> Error {
        Error::RuntimeError(
            ErrorBody {
                message,
                pos: start.into()
            }
        )
    }

    pub fn compiler<T: Into<i32>>(message: String, start: T) -> Error {
        Error::CompileTimeError(
            ErrorBody {
                message,
                pos: start.into()
            }
        )
    }
}

// Implement a runtime error & a compile time error
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::RuntimeError(message) => {
                write!(f, "Runtime error: {}", message.message)
            },
            Error::CompileTimeError(message) => {
                write!(f, "Compiler error: {}", message.message)
            }
        }
    }
}

// Implement a runtime error & a compile time error
impl std::error::Error for Error {}
