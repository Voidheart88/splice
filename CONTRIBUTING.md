# Contributing to Splice

Thank you for contributing to Splice! This document provides guidelines and information for contributors.

## Development Workflow

### Running Tests
```bash
# Run all tests
cargo test

# Run tests for a specific module
cargo test --lib models::resistor

# Run tests with output
cargo test -- --nocapture
```

### Running Benchmarks

Benchmarks use the Criterion framework. To run them:

```bash
# Run all benchmarks
cargo bench

# Run a specific benchmark
cargo bench --bench performance

# Run benchmarks with access to internal functions
cargo bench --features bench-internals
```

### The `bench-internals` Feature

Splice uses a `bench-internals` feature to control visibility of internal modules:

- **Default build**: Internal implementation details are `pub(crate)` only
- **With `bench-internals`**: Internal modules become accessible to benchmarks

This keeps the public API clean while allowing comprehensive benchmarking.

**For Benchmark Authors:**

When writing benchmarks that need access to internal functions:

1. The benchmark will automatically have access when run with `--features bench-internals`
2. Import internal types normally:
   ```rust
   use splice::sim::integration::BackwardEuler;  // Internal, but accessible with feature
   use splice::solver::FaerSolver;
   ```
3. Test your benchmark compiles:
   ```bash
   cargo bench --bench your_benchmark --features bench-internals --no-run
   ```

## Code Style

Please follow the style guide in `notes/style-guide.md`. Key points:

- Prefer functional style (transform over mutate)
- Keep functions small and focused
- Minimize cyclomatic complexity
- Use iterators over for/while loops where possible
- Follow DRY principle (including tests)

## Adding New Models

1. Create a new module in `src/models/your_model/`
2. Implement the `Element` trait
3. Add serde support in `serde.rs`
4. Add SPICE parser support in `spice.rs`
5. Add tests in `tests.rs`
6. Re-export from `src/models/mod.rs`

## Adding New Frontends/Backends

1. Create module in `src/frontends/` or `src/backends/`
2. Implement the respective trait
3. Add tests
4. Update documentation

## Reporting Bugs

Please include:
- SPICE netlist or circuit description
- Expected vs. actual behavior
- Error messages (Splice should provide clear diagnostics!)
- Splice version

## Performance Issues

If you encounter performance problems:

1. Run benchmarks to identify the bottleneck
2. Profile with `cargo flamegraph` or similar tools
3. Consider:
   - Matrix solver choice (faer, nalgebra, rsparse)
   - Integration method (Backward Euler, Trapezoidal, BDF)
   - Time step size in transient simulations

## Questions?

Feel free to open an issue for any questions or discussions about the codebase.
