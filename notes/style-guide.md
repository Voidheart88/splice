## Nesting

If you want to nest your loops and matches try to keep the nesting as lean as possible

Bad:
``` rust
for simulation in circuit.simulations {
    match simulation {
        SerdeSimulation::OP => {
            commands.push(SimulationCommand::Op);
        }
        SerdeSimulation::DC(dc) => {
            commands.push(SimulationCommand::Dc(
                Arc::from(dc.source()),
                dc.vstart(),
                dc.vstop(),
                dc.vstep(),
                None,
            ));
        }
        SerdeSimulation::AC(ac) => {
            commands.push(SimulationCommand::Ac(
                ac.fstart(),
                ac.fstop(),
                ac.fstep(),
                ACMode::default(),
            ));
        }
        SerdeSimulation::Tran(tran) => {
            commands.push(SimulationCommand::Tran(tran.tstep(), tran.tend()));
        }
    }
}
```

This is bad because the reader has to follow the nests 4 indendations deep.

Good:
```
for simulation in circuit.simulations {
    commands.push(simulation.into())
}
```

Even better:
```
let commands: Vec<SimulationCommand> = circuit.simulations
    .into_iter()
    .map(|simulation| simulation.into())
    .collect();
```

## Borrow Checker bracketing

consider the folloing pattern:
``` rust
while voltage <= *vstop {
    {
        let source = match &mut self.elements[vsource1_idx] {
            Element::VSource(ref mut vs) => vs,
            _ => unreachable!(),
        };
        source.set_voltage(voltage);
    }
    dc_results.push(self.find_op()?);
    voltage += vstep;
}
```

The borrow checker would be unhappy as source borrows a reference to vs. To fix this an additional block is 
needed. I consider this as an anti pattern, since this can be written as:

``` rust
fn set_voltage_on_source(elements: &mut [Element], vsource1_idx: usize, voltage: Numeric) {
    if let Element::VSource(ref mut vs) = elements[vsource1_idx] {
        vs.set_voltage(voltage);
    } else {
        unreachable!();
    }
}

while voltage <= *vstop {
    set_voltage_on_source(&mut self.elements, vsource1_idx, voltage);
    dc_results.push(self.find_op()?);
    voltage += vstep;
}
```
which is more readable and serves the same purpose.


## Use cargo clippy and cargo fmt
use `cargo clippy --all-targets --all-features -- -D warnings` after you created code since it prevents you
to use commonly used anti-patterns.
use `cargo fmt` after you created code since it formats the code to a known standard. 
use `cargo fmt` before you check for nesting depth.

## Use extra module files for tests

Test code should be placed in dedicated test files rather than inline test modules:
- `src/module/tests.rs` - Main test file for a module
- `src/module/tests/submodule.rs` - Organized test submodules when needed

This keeps production code focused and makes tests easier to navigate.


## TODO and FIXME Comments

**Avoid using TODO/FIXME comments in production code.** Instead:

1. **For bugs**: Fix them immediately or create a GitHub issue
2. **For improvements**: Track in project backlog (e.g., `notes/todo.md`)
3. **For incomplete features**: Either complete them or remove the comment if the code works as-is

Inline TODOs create technical debt and clutter the codebase. If something needs attention, make it trackable and actionable.


## Use `unreachable!()` vs `todo!()` Appropriately

Choose the right macro for unreachable code paths:

- **`unreachable!()`** - For code paths that should never be reached logically
  ```rust
  match result {
      Sim::Op(items) => items,
      Sim::Dc(_) => unreachable!("init_sim tests only run OP commands"),
  }
  ```

- **`todo!()`** - Only during active development, never commit with `todo!()`
  
Always add a descriptive message explaining why the path is unreachable.


## Prefer Iterator Patterns

Convert loops to iterator-based code where it improves clarity:

```rust
// Before
for (i, item) in items.iter().enumerate() {
    // use i and item
}

// After (when possible)
items.iter().enumerate().for_each(|(i, item)| {
    // use i and item
});
```

However, prioritize readability over cleverness. You might implement new iterators if they sevr.


## Limit Nesting Depth

Keep nesting to **maximum 3-4 levels**. Use early returns and guard clauses:

```rust
// Instead of deep nesting
if let Some(value) = opt {
    if let Some(inner) = value.inner {
        if inner.valid() {
            // do something
        }
    }
}

// Use early returns
let Some(value) = opt else { return; };
let Some(inner) = value.inner else { return; };
if !inner.valid() { return; }
// do something
```


## Centralize Constants

Move magic numbers and configuration values to appropriate central locations:
- Solver thresholds → `src/spot.rs`
- Simulation limits → `src/spot.rs`
- Model-specific constants → respective model modules
