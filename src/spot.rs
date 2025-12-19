use num::Complex;

pub type Numeric = f64;
pub type ComplexNumeric = Complex<Numeric>;

/// Ten
pub const NUMERIC_TEN: Numeric = 10.0;

/// TWO
pub const NUMERIC_TWO: Numeric = 10.0;

/// Boltzmann constant J/K
pub const KB: Numeric = 1.380_649e-23;

/// Elementary Charge
pub const ELE_CHRG: Numeric = 1.602_176_634e-19;

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

/// Constants for adaptive timestep control
pub(crate) const ADAPTIVE_MIN_TIMESTEP: Numeric = 1e-9;
pub(crate) const ADAPTIVE_MAX_TIMESTEP: Numeric = 1e-3;
pub(crate) const ADAPTIVE_INITIAL_TIMESTEP: Numeric = 1e-6;
pub(crate) const ADAPTIVE_TOLERANCE: Numeric = 1e-4;
pub(crate) const ADAPTIVE_SAFETY_FACTOR: Numeric = 0.9;
pub(crate) const ADAPTIVE_MAX_GROWTH_FACTOR: Numeric = 2.0;
pub(crate) const ADAPTIVE_MIN_GROWTH_FACTOR: Numeric = 0.5;

/// Constants for solver selection thresholds
/// Default threshold for switching between dense and sparse solvers
pub const DEFAULT_SOLVER_THRESHOLD: usize = 50;

/// Threshold for very small circuits where Nalgebra is fastest
pub const SMALL_CIRCUIT_THRESHOLD: usize = 10;

/// Threshold for medium circuits where FaerSparse is optimal
pub const MEDIUM_CIRCUIT_THRESHOLD: usize = 100;
