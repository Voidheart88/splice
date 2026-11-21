// Guesser module exposing the public API

// Guesser module exposing the public API

pub mod traits;
pub mod default;

pub use traits::{Guesser, GuesserOptions, new_guesser};
pub use default::StandardGuesser;

