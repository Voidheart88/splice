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
  - **Network mode with MessagePack protocol** - Run Splice as a server for remote simulations

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
  - Implement adaptive time step control for better stability

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
  - Implement a transient simulation
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
