/// Boltzmann constant J/K
pub const KB: f64 = 1.380649e-23f64;

/// Elementary Charge
pub const ELE_CHRG: f64 = 1.602176634e-19f64;

/// Room temperature in K
pub const TEMP: f64 = 293.15f64;

/// The default conductance for inductors in S
pub const DEFAULT_CONDUCTANCE: f64 = 1e24;

/// The dfault temperature voltage
pub const UT: f64 = KB * TEMP / ELE_CHRG;

pub const DIO_GUESS: f64 = 0.4;

/// Define tolerance and maximum number of iterations
pub const VECTOL: f64 = 1e-3;
pub const MAXITER: usize = 1000;
