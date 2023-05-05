pub mod io;
pub mod net;

// Re-export output
pub use self::io::output::Print;
pub use self::io::output::PrintLine;
pub use self::io::input::ReadLn;
