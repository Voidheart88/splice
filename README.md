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
  - Working V,I,R,L,C,D device models
  - Minimal diode model
  - Minimal mosfet model
  - Capacitors and inductors work in .ac simulation
  - Advanced transient simulation with adaptive time step control
  - Multiple integration methods (Backward Euler, Trapezoidal)
  - Network mode with MessagePack protocol - Run Splice as a server for remote simulations

## Todos/Roadmap:

### Frontends:
  - Build a KiCad frontend

### Solver:
  - Build a CUDA/OpenCL backend
  - Improve adaptive time step control with more sophisticated error estimation

### Outputs:
  - Improve the CSV output
  - Improve the plot output
  - Build the raw output

### Models:
  - Implement magnetic simulations

### Features:
  - Implement an intelligent solving strategy finder (maybe an AI thingy?)

### How to contribute:
  - Write tests for every module, the frontends, the backends, and the outputs. Tests define the expected behavior.
  - Find failing simulations and how to make them runnable - we need data to make Splice rock solid. A failing simulation should be considered a bug since reality doesn't fail!
  - Write benchmarks: Splice is a simulator - simulation should be as fast as possible. The room where your computer is should not need any heating while you simulate!
  - **Network mode testing**: Help test the new network functionality with different clients and scenarios
  - **Documentation**: Improve the network mode documentation and add more examples

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you shall be dual licensed as above, without any additional terms or conditions.
