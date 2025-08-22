mod mos0;
/// The Vsource Module. As every module this module encapsulates exerything regarding a Vsource bundle
/// This includes parsing from various formats as well as the conductance-behaviour.
pub(crate) mod spice;

pub use mos0::Mos0Bundle;
