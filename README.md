# Splice - A blazingly fast Spice simulator
The goal of Splice is to provide a better simulation experience.

## Goals
  - Better error reporting! Your simulation fails? Splice tells you why. It shows you non-convergent nodes and tries to use the best possible method to choose a solving strategy for your problems. A valid but non-converging simulation is considered a bug!
  - Faster! Splice uses modern mechanisms like parallel element evaluation to be as fast as possible. The code is   well-tested!
  - Extensible! The interfaces between frontend, backend, and output modules are well-defined. New models, frontends,  and backends are easy to implement.
  - Minimal! Splice is runnable on small ARM PCs as well as on your HPC rig.
  - Useful error-messages!

## Features
  - Working .dc simulation
  - Working .op simulation
  - Working .ac simulation
  - Working V,I,R device models
  - Minimal diode model
  - Minimal mosfet model
  - Capacitors and inductors work in .ac simulation
  - Basic transient simulation with fixed time step

## Technical Details

### Capacitor Implementation

The capacitor model implements proper transient behavior using the following approach:

**1. State Storage:**
```rust
pub struct CapacitorBundle {
    pub name: Arc<str>,
    pub node0: Option<Variable>,
    pub node1: Option<Variable>,
    pub value: Numeric,              // Capacitance value
    pub previous_voltage: Numeric,   // Voltage from previous time step
}
```

**2. Matrix Contribution (triples method):**
- Calculates the equivalent conductance: G = C/Δt
- Returns the matrix entries for the MNA system
- Implements the left-hand side of the integration formula

**3. Right-Hand Side (pairs method):**
- Calculates the current due to stored charge: I = C/Δt * V_prev
- Returns the vector entries for the MNA system
- Implements the right-hand side of the integration formula

**4. Time Integration:**
The complete integration formula is:
```
[ C/Δt   -C/Δt ] [V0]   [ I0 ]   [ C/Δt * V_prev ]
[ -C/Δt  C/Δt ] [V1] = [ I1 ] + [ -C/Δt * V_prev ]
```

### Transient Simulation

The transient simulation uses implicit Euler integration with the following approach:

1. **Matrix Formulation**: For each time step, the system is formulated as:
   ```
   A * x(t) = b(t)
   ```
   where A contains the conductance matrix and b contains the source terms.

2. **Time Integration**: Capacitors are integrated using:
   ```
   I(t) = C * (V(t) - V(t-Δt)) / Δt
   ```
   This is implemented using the `triples()` method for the matrix A and the `pairs()` method for the right-hand side vector b.

3. **Nonlinear Elements**: Diodes and MOSFETs use Newton-Raphson iteration within each time step.

### Adaptive Time Step Control (Future Work)

For improved stability and efficiency, an adaptive time step control could be implemented:

```rust
fn adaptive_time_step(&mut self, t: &mut Numeric, tstop: &Numeric) -> Result<(), SimulatorError> {
    // 1. Estimate system time constants
    let tau = self.estimate_time_constants();
    
    // 2. Calculate optimal step size (e.g., Δt = τ/100)
    let delta_t = (tau / 100.0).clamp(1e-9, 1e-3);
    
    // 3. Perform time step
    self.single_step(delta_t)?;
    
    // 4. Estimate error and adjust step size
    let error = self.estimate_error();
    self.adjust_step_size(error);
    
    *t += delta_t;
    Ok(())
}
```

**Benefits of Adaptive Time Stepping:**
- Automatic stability: Small steps for fast transients, large steps for steady state
- Efficiency: Optimal computation time for each simulation
- Accuracy: Error control ensures precise results

## Known Issues and Limitations

### Transient Simulation

1. **Fixed Time Step Limitations:**
   - The current implementation uses fixed time steps
   - For small capacitors (e.g., 100nF), the time step must be very small (e.g., 1µs) for stability
   - Larger time steps can cause oscillations or divergence

2. **Capacitor Charging:**
   - Capacitors charge correctly but may require very small time steps
   - The integration is stable for Δt < τ/100 where τ = R·C
   - Example: For R=1kΩ, C=1µF (τ=1ms), use Δt < 10µs

3. **Numerical Stability:**
   - The implicit Euler integration is conditionally stable
   - For better stability, consider implementing adaptive time stepping

### AC Analysis

1. **Frequency Range:**
   - The AC analysis works well for most frequency ranges
   - Very high frequencies may require more points for accuracy

2. **Complex Models:**
   - Some complex models may not be fully supported
   - The implementation focuses on basic R, L, C elements

## Todos/Roadmap:

### Frontends:
  - Build a network frontend - Splice should be runnable inside a container without many dependencies
  - Build a KiCad frontend

### Solver:
  - Build a CUDA/OpenCL backend
  - Implement adaptive time step control for better stability

### Outputs:
  - Improve the CSV output
  - Build the network output
  - Improve the plot output
  - Build the raw output

### Models:
  - Implement a BJT and MOSFET model
  - Implement magnetic simulations
  - Build controlled sources

### Features:
  - Allow the annotation of nodes and branches with physical quantities such as current density or revolutions per minute for a better simulation experience with things like an electrical motor. Explicity helps a lot here!
  - Implement a transient simulation
  - Implement an intelligent solving strategy finder (maybe an AI thingy?)

### How to contribute:
  - Write tests for every module, the frontends, the backends, and the outputs. Tests define the expected behavior.
  - Find failing simulations and how to make them runnable - we need data to make Splice rock solid. A failing simulation should be considered a bug since reality doesn't fail!
  - Write benchmarks: Splice is a simulator - simulation should be as fast as possible. The room where your computer is should not need any heating while you simulate!

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you shall be dual licensed as above, without any additional terms or conditions.
