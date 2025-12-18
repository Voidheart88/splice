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

Maybe even better:
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


## Use cargo clippy
use `cargo clippy --all-targets --all-features -- -D warnings` after you created code since it prevents you
to use commonly used anti-patterns


