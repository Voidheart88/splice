# Splice - A blazingly fast Spice simulator

The Goal of splice is a better simulation experience.

## Features
  - Better error reporting! Your simulation fails? Splice tells you why. It shows you
  non convergent nodes and trys to use the best possible method to choose a solving
  strategy for your problems. A valid but non converging simulation is considered a Bug!
  - Faster! Splice uses modern mechanisms like parralel Element Evaluation to be
  as fast as possible. The Code is well tested!
  - Extensible! The interfaces between frontend, backend and output modules are
  well defined. New models, frontends and backends are easy to implement.
  - Minimal! Splice is runnable on small ARM PCs as well as its runnable on your HPC Rig

## Todos/Roadmap:
  - Frontends:
    - Build a json frontend
    - Build a yml frontend
    - Build a network frontend - Splice should be runnable inside a container without
    many dependencys
    - Build a kicad frontend
    - Improve the spice frontend
  - Backends:
    - Build a CUDA/OpenCL backend
    - Build a FPGA backend (should be a lot of fun)
  - Outputs:
    - Improve the CSV output
    - Build the network output
    - Improve the plot output
    - Build the raw output
  - Models:
    - Implement an BJT and MOSFET model
    - Implement magnetic simulations
    - Build controlled sources
  - Features:
    - Allow the annotation of Nodes and branches with physical quantities such as
    current density or revolutions per minute for a better Simulation experience with
    stuff like an electrical motor. Explicity helps a lot here!
    - Implement a ac simulaiton
    - Implement a transient simulation
    - Implement a intelligent solving strategy finder (maybe an AI thingy?)
    - Improve Error Messaging and UX Design especially in the frontend

## How to contribute:
  - Write tests for every Module, the frontends, the backends and the outputs.
  Test define the expected behavior.
  - Find failing simulations and how to make them runnable - We need Data to make
  splice rock solid. A failing simulation schould be considered as a bug since the reality doesn't fail!
  - Write benchmarks: Splice is a simulator - Simulation should be as fast as possible
  The room where your computer is should'nt need any heating while you simulate!

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you shall be dual licensed as above, without any additional terms or conditions.