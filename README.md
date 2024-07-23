# Splice - A blazingly fast Spice simulator
The goal of Splice is to provide a better simulation experience.

## Goals
  - Better error reporting! Your simulation fails? Splice tells you why. It shows you non-convergent nodes and tries to use the best possible method to choose a solving strategy for your problems. A valid but non-converging simulation is considered a bug!
  - Faster! Splice uses modern mechanisms like parallel element evaluation to be as fast as possible. The code is   well-tested!
  - Extensible! The interfaces between frontend, backend, and output modules are well-defined. New models, frontends,  and backends are easy to implement.
  - Minimal! Splice is runnable on small ARM PCs as well as on your HPC rig.

## Features
  - Working .dc simulation
  - Working .op simulation
  - Working .ac simulation
  - Working V,I,R device models
  - Minimal diode model
  - Minimal mosfet model
  - Capacitors and inductors work in .ac simulation

## Todos/Roadmap:
### Frontends:
  - Build a JSON frontend
  - Build a YAML frontend
  - Build a network frontend - Splice should be runnable inside a container without many dependencies
  - Build a KiCad frontend
  - Improve the Spice frontend

### Solver:
  - Build a CUDA/OpenCL backend
  - Build an FPGA backend (should be a lot of fun)

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
  - Allow the annotation of nodes and branches with physical quantities such as current density or revolutions per minute for a better simulation experience with things like an electrical motor. Explicitly helps a lot here!
  - Implement an AC simulation
  - Implement a transient simulation
  - Implement an intelligent solving strategy finder (maybe an AI thingy?)
  - Improve error messaging and UX design, especially in the frontend

### How to contribute:
  - Write tests for every module, the frontends, the backends, and the outputs. Tests define the expected behavior.
  - Find failing simulations and how to make them runnable - we need data to make Splice rock solid. A failing simulation should be considered a bug since reality doesn't fail!
  - Write benchmarks: Splice is a simulator - simulation should be as fast as possible. The room where your computer is should not need any heating while you simulate!

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you shall be dual licensed as above, without any additional terms or conditions.