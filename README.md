# Splice - A blazingly fast Spice simulator

The Goal of splice is a better simulation experience.

## Features
  - Better error reporting! Your simulation fails? Splice tells you why. It shows you
  non convergent nodes and trys to use the best possible method to choose a solving
  strategy for your problems. A non converging simulation is considered a Bug!
  - Faster! Splice uses modern mechanisms like cache friendly memory layout, SIMD
  and ECS Evaluation. The Code is well benchmarked and tested!
  - Extensible! The interfaces between frontend, backend and output modules are
  well defined. New models, frontends and backends are easy to implement.
  - Minimal! Splice is runnable on small ARM PCs as well as its runnable on a HPC Rig

## Todos/Roadmap:
  - Frontends:
    - Build a JSON frontend
    - Build a yml frontend
    - Build a network frontend - Splice should be runnable inside a container without
    many dependencys
    - Improve the spice frontend
  - Backends:
    - Improve the nalgebra backend
    - Implement a sparse solver
    - Build a CUDA/OpenCL backend
    - Build a FPGA backend (should be a lot of fun)
  - Outputs:
    - Improve the CSV output
    - Build the network output
    - Build the plot output
    - Build the raw output
  - Models:
    - Build some capacitor models
    - Build some diode models
    - Build some inductor models
    - Implement magnetic simulations
    - Build additional (U,I,E,B) sources
  - Features:
    - Allow the annotation of Nodes and branches with physical quantities such as
    current density or revolutions per minute for a better Simulation of stuff like a
    electrical motor. Explicity helps a lot here!
    - Implement a dc simulation
    - Implement a ac simulaiton
    - Implement a transient simulation
    - Implement a intelligent solving strategy finder (maybe an AI thingy?)
    - Improve Error Messaging and UX Design
    - Implement some performance options for the user

## How to contribute:
  - Write tests for every Module, the frontends, the backends and the outputs.
  Test define the expected behavior.
  - Find failing simulations and how to make them runnable - We need Data to make
  splice rock solid. A failing simulation schould be considered as a bug since the reality does'nt fail!
  - Write benchmarks: Splice is a simulator - Simulation should be as fast as possible
  The room where your computer stands should'nt need any heating while you simulate
