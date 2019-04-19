pub use self::languages::{is_supported, run};

mod languages;

// Language Runners
mod javascript;
mod golang;
mod python;
mod rust;