use num::Complex;

pub type Numeric = f64;
pub type ComplexNumeric = Complex<Numeric>;

/// Boltzmann constant J/K
pub const KB: Numeric = 1.380649e-23f64;

/// Elementary Charge
pub const ELE_CHRG: Numeric = 1.602176634e-19f64;

/// Room temperature in K
pub const TEMP: Numeric = 293.15f64;

/// The default conductance for inductors in S
pub const DEFAULT_CONDUCTANCE: Numeric = 1e24;

/// The dfault temperature voltage
pub const UT: Numeric = KB * TEMP / ELE_CHRG;

pub const DIO_GUESS: Numeric = 0.4;

/// Define tolerance and maximum number of iterations
pub const VECTOL: Numeric = 1e-3;
pub const MAXITER: usize = 1000;
