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
  - Advanced transient simulation with adaptive time step control
  - Multiple integration methods (Backward Euler, Trapezoidal)
  - **Network mode with MessagePack protocol** - Run Splice as a server for remote simulations

## Known Issues and Limitations

### Transient Simulation

1. **Adaptive Time Step Control:**
   - The implementation now supports adaptive time step control for improved efficiency and stability
   - Automatic activation when small time steps (≤1µs) are specified
   - Dynamic adjustment based on error estimation with quadratic error control
   - Safety factors and growth limits ensure numerical stability
   - Backward compatible with fixed time step simulations

2. **Capacitor Charging:**
   - Capacitors charge correctly with improved stability using adaptive time steps
   - The adaptive control automatically adjusts Δt based on circuit dynamics
   - For circuits with varying time constants, adaptive stepping significantly improves efficiency

3. **Numerical Stability:**
   - Implicit Euler integration (default) with trapezoidal method available
   - Adaptive time stepping uses relative error estimation with tolerance of 1e-4
   - Timestep bounds: 1ns minimum, 1ms maximum
   - Growth factors limit rapid changes for smooth convergence

4. **Integration Methods:**
   - Backward Euler: Default method, stable for stiff systems
   - Trapezoidal: Available for improved accuracy in certain scenarios
   - Can be selected via `.tran` command options

### AC Analysis

1. **Frequency Range:**
   - The AC analysis works well for most frequency ranges
   - Very high frequencies may require more points for accuracy

2. **Complex Models:**
   - Some complex models may not be fully supported
   - The implementation focuses on basic R, L, C elements

### Network Mode

1. **Security:**
   - Current implementation uses plain TCP without encryption
   - For production use, consider adding TLS/SSL or running in a secure network

2. **Performance:**
   - Large circuits may require significant memory and computation time
   - Consider using `faer-sparse` solver for better performance with large circuits

3. **Error Handling:**
   - Network errors are handled gracefully, but complex error recovery is not yet implemented
   - Client should implement reconnection logic for production use

## Todos/Roadmap:

### Frontends:
  - ✅ **Build a network frontend** - Splice should be runnable inside a container without many dependencies
  - Build a KiCad frontend

### Solver:
  - Build a CUDA/OpenCL backend
  - Improve adaptive time step control with more sophisticated error estimation

### Outputs:
  - Improve the CSV output
  - ✅ **Build the network output** - MessagePack-based network backend implemented
  - Improve the plot output
  - Build the raw output

### Models:
  - Implement a BJT and MOSFET model
  - Implement magnetic simulations
  - Build controlled sources

### Features:
  - Allow the annotation of nodes and branches with physical quantities such as current density or revolutions per minute for a better simulation experience with things like an electrical motor. Explicity helps a lot here!
  - Enhance transient simulation with additional integration methods
  - Implement an intelligent solving strategy finder (maybe an AI thingy?)

### Network Mode Enhancements:
  - Add WebSocket support for real-time updates
  - Implement REST API with JSON support
  - Add authentication (JWT/OAuth)
  - Implement batch processing for multiple circuits
  - Add health check endpoint

### How to contribute:
  - Write tests for every module, the frontends, the backends, and the outputs. Tests define the expected behavior.
  - Find failing simulations and how to make them runnable - we need data to make Splice rock solid. A failing simulation should be considered a bug since reality doesn't fail!
  - Write benchmarks: Splice is a simulator - simulation should be as fast as possible. The room where your computer is should not need any heating while you simulate!
  - **Network mode testing**: Help test the new network functionality with different clients and scenarios
  - **Documentation**: Improve the network mode documentation and add more examples

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you shall be dual licensed as above, without any additional terms or conditions.

## Network Mode Quick Start

To run Splice in network mode:

```bash
# Start the network server
splice --frontend network --backend network --solver faer-sparse

# In another terminal, run a client (see notes/network_docs.md for examples)
```

For more information, see the complete network documentation in `notes/network_docs.md`.
