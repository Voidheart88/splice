use num::Complex;

// Types and constants - public for benchmarks with bench-internals feature
#[cfg(not(feature = "bench-internals"))]
pub type Numeric = f64;
#[cfg(not(feature = "bench-internals"))]
pub type ComplexNumeric = Complex<Numeric>;

#[cfg(feature = "bench-internals")]
pub(crate) type Numeric = f64;
#[cfg(feature = "bench-internals")]
pub(crate) type ComplexNumeric = Complex<Numeric>;

/// Ten
#[cfg(not(feature = "bench-internals"))]
pub const NUMERIC_TEN: Numeric = 10.0;
#[cfg(feature = "bench-internals")]
pub(crate) const NUMERIC_TEN: Numeric = 10.0;

/// TWO
#[cfg(not(feature = "bench-internals"))]
pub const NUMERIC_TWO: Numeric = 10.0;
#[cfg(feature = "bench-internals")]
pub(crate) const NUMERIC_TWO: Numeric = 10.0;

/// Boltzmann constant J/K
#[cfg(not(feature = "bench-internals"))]
pub const KB: Numeric = 1.380_649e-23;
#[cfg(feature = "bench-internals")]
pub(crate) const KB: Numeric = 1.380_649e-23;

/// Elementary Charge
#[cfg(not(feature = "bench-internals"))]
pub const ELE_CHRG: Numeric = 1.602_176_634e-19;
#[cfg(feature = "bench-internals")]
pub(crate) const ELE_CHRG: Numeric = 1.602_176_634e-19;

/// Room temperature in K
#[cfg(not(feature = "bench-internals"))]
pub const TEMP: Numeric = 293.15;
#[cfg(feature = "bench-internals")]
pub(crate) const TEMP: Numeric = 293.15;

/// The default conductance for inductors in S
#[cfg(not(feature = "bench-internals"))]
pub const DEFAULT_CONDUCTANCE: Numeric = 1e24;
#[cfg(feature = "bench-internals")]
pub(crate) const DEFAULT_CONDUCTANCE: Numeric = 1e24;

/// The dfault temperature voltage
#[cfg(not(feature = "bench-internals"))]
pub const UT: Numeric = KB * TEMP / ELE_CHRG;
#[cfg(feature = "bench-internals")]
pub(crate) const UT: Numeric = KB * TEMP / ELE_CHRG;

#[cfg(not(feature = "bench-internals"))]
pub const DIO_GUESS: Numeric = 0.4;
#[cfg(feature = "bench-internals")]
pub(crate) const DIO_GUESS: Numeric = 0.4;

/// Define tolerance and maximum number of iterations
#[cfg(not(feature = "bench-internals"))]
pub const VECTOL: Numeric = 1e-3;
#[cfg(not(feature = "bench-internals"))]
pub const MAXITER: usize = 1000;

#[cfg(feature = "bench-internals")]
pub(crate) const VECTOL: Numeric = 1e-3;
#[cfg(feature = "bench-internals")]
pub(crate) const MAXITER: usize = 1000;

/// Constants for adaptive timestep control
#[cfg(not(feature = "bench-internals"))]
pub(crate) const ADAPTIVE_MIN_TIMESTEP: Numeric = 1e-9;
#[cfg(not(feature = "bench-internals"))]
pub(crate) const ADAPTIVE_MAX_TIMESTEP: Numeric = 1e-3;
#[cfg(not(feature = "bench-internals"))]
pub(crate) const ADAPTIVE_INITIAL_TIMESTEP: Numeric = 1e-6;
#[cfg(not(feature = "bench-internals"))]
pub(crate) const ADAPTIVE_TOLERANCE: Numeric = 1e-4;
#[cfg(not(feature = "bench-internals"))]
pub(crate) const ADAPTIVE_SAFETY_FACTOR: Numeric = 0.9;
#[cfg(not(feature = "bench-internals"))]
pub(crate) const ADAPTIVE_MAX_GROWTH_FACTOR: Numeric = 2.0;
#[cfg(not(feature = "bench-internals"))]
pub(crate) const ADAPTIVE_MIN_GROWTH_FACTOR: Numeric = 0.5;

#[cfg(feature = "bench-internals")]
pub(crate) const ADAPTIVE_MIN_TIMESTEP: Numeric = 1e-9;
#[cfg(feature = "bench-internals")]
pub(crate) const ADAPTIVE_MAX_TIMESTEP: Numeric = 1e-3;
#[cfg(feature = "bench-internals")]
pub(crate) const ADAPTIVE_INITIAL_TIMESTEP: Numeric = 1e-6;
#[cfg(feature = "bench-internals")]
pub(crate) const ADAPTIVE_TOLERANCE: Numeric = 1e-4;
#[cfg(feature = "bench-internals")]
pub(crate) const ADAPTIVE_SAFETY_FACTOR: Numeric = 0.9;
#[cfg(feature = "bench-internals")]
pub(crate) const ADAPTIVE_MAX_GROWTH_FACTOR: Numeric = 2.0;
#[cfg(feature = "bench-internals")]
pub(crate) const ADAPTIVE_MIN_GROWTH_FACTOR: Numeric = 0.5;

/// Default threshold for solver selection (number of variables)
#[cfg(not(feature = "bench-internals"))]
pub const SOLVER_SELECTION_THRESHOLD: usize = 50;
#[cfg(feature = "bench-internals")]
pub(crate) const SOLVER_SELECTION_THRESHOLD: usize = 50;
