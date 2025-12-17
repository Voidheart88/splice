// Controlled sources module
// This module implements the four types of controlled sources:
// - VCVS (Voltage-Controlled Voltage Source) - E source
// - VCCS (Voltage-Controlled Current Source) - G source  
// - CCCS (Current-Controlled Current Source) - F source
// - CCVS (Current-Controlled Voltage Source) - H source

pub mod vcvrs;
pub mod vccs;
pub mod cccs;
pub mod ccvs;
pub mod serde;
pub mod spice;

pub use vcvrs::VCVSBundle;
pub use vccs::VCCSBundle;
pub use cccs::CCCSBundle;
pub use ccvs::CCVSBundle;