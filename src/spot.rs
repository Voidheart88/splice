use num::Complex;

pub type Numeric = f64;
pub type ComplexNumeric = Complex<Numeric>;

/// Ten
pub const NUMERIC_TEN: Numeric = 10.0;

/// TWO
pub const NUMERIC_TWO: Numeric = 10.0;

/// Boltzmann constant J/K
pub const KB: Numeric = 1.380649e-23;

/// Elementary Charge
pub const ELE_CHRG: Numeric = 1.602176634e-19;

/// Room temperature in K
pub const TEMP: Numeric = 293.15;

/// The default conductance for inductors in S
pub const DEFAULT_CONDUCTANCE: Numeric = 1e24;

/// The dfault temperature voltage
pub const UT: Numeric = KB * TEMP / ELE_CHRG;

pub const DIO_GUESS: Numeric = 0.4;

/// Define tolerance and maximum number of iterations
pub const VECTOL: Numeric = 1e-3;
pub const MAXITER: usize = 1000;
