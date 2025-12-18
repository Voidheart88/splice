/// The Vsource Module. As every module this module encapsulates exerything regarding a Vsource bundle
/// This includes parsing from various formats as well as the conductance-behaviour.
mod mos0;
pub(crate) mod serde;
pub(crate) mod spice;

#[cfg(test)]
mod tests;

pub use mos0::Mos0Bundle;
