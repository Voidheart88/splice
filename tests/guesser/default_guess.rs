// Tests for the Guesser implementation

#[cfg(test)]
mod tests {
    use splice::guesser::{new_guesser, Guesser};
    use splice::sim::Simulator;
    use splice::solver::RSparseSolver;
    use splice::frontends::Simulation;

    #[test]
    fn standard_guesser_matches_initial_guess() {
        // Minimal empty simulation (no elements, no variables)
        let sim_struct = Simulation {
            commands: vec![],
            options: vec![],
            elements: vec![],
            variables: vec![],
        };
        let mut sim: Simulator<RSparseSolver> = Simulator::from(sim_struct);

        // Use the Guesser factory (default options -> StandardGuesser)
        let guesser = new_guesser(sim.guesser_options.clone());
        let guess_vec = guesser.guess(&mut sim).expect("Guesser should succeed");
        let expected = sim.generate_initial_guess();
        assert_eq!(guess_vec, expected);
    }
}
