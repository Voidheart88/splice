use std::path::{Path, PathBuf};

use super::Backend;
use crate::{
    models::{Unit, Variable},
    sim::simulation_result::{Sim, SimulationResults},
    BackendError,
};
use plotters::{
    backend::SVGBackend,
    prelude::*,
    style::full_palette::{GREY_400, GREY_800},
};

/// A struct for handling plot output of simulation results.
pub struct PlotBackend {
    pth: String,
}

impl Backend for PlotBackend {
    /// Outputs the simulation results as a plot.
    ///
    /// # Parameters
    ///
    /// - `res`: The `SimulationResult` to be output.
    ///
    /// # Returns
    ///
    /// A `Result` which is `Ok` if the output operation succeeds, or an `BackendError` if it fails.
    fn output(&self, res: SimulationResults) -> Result<(), BackendError> {
        for sim in res.0.iter() {
            self.select_output(sim)?;
        }

        Ok(())
    }
}

impl PlotBackend {
    /// Creates a new `PlotOutput` instance.
    ///
    /// # Parameters
    ///
    /// - `pth`: The file path where the plot will be saved.
    ///
    /// # Returns
    ///
    /// A new `PlotOutput` instance.
    pub fn new(pth: String) -> Self {
        Self { pth }
    }

    /// Selects the appropriate output method for the given simulation result.
    ///
    /// # Parameters
    ///
    /// - `sim`: The `Sim` result to be output.
    /// - `chart`: The chart to which the data will be added.
    ///
    /// # Returns
    ///
    /// A `Result` which is `Ok` if the output operation succeeds, or an `BackendError` if it fails.
    fn select_output(&self, sim: &Sim) -> Result<(), BackendError> {
        match sim {
            Sim::Op(data) => self.plot_op(data)?,
            Sim::Dc(data) => self.plot_dc(data)?,
        }
        Ok(())
    }

    /// Plots the DC simulation results.
    ///
    /// # Parameters
    ///
    /// - `data`: A reference to a vector of vectors of tuples where each tuple contains a `Variable` and a `f64`.
    ///
    /// # Returns
    ///
    /// A `Result` which is `Ok` if the plotting operation succeeds, or an `BackendError` if it fails.
    fn plot_dc(&self, data: &Vec<Vec<(Variable, f64)>>) -> Result<(), BackendError> {
        let mut path = PathBuf::from(&self.pth);
        path.set_extension("svg");

        // Add the suffix before the extension
        let stem = path.file_stem().and_then(|s| s.to_str()).unwrap_or("");
        let parent = path.parent().unwrap_or(Path::new(""));
        let new_file_name = format!("{}_dc.svg", stem);
        path = parent.join(new_file_name);

        let root = SVGBackend::new(&path, (1440, 900)).into_drawing_area();
        root.fill(&BLACK)?;

        let (max, min) = data
            .iter()
            .flat_map(|vec| vec.iter())
            .map(|(_, val)| val)
            .fold((None, None), |(max_acc, min_acc), &x| {
                (
                    Some(max_acc.map_or(x, |y| x.max(y))),
                    Some(min_acc.map_or(x, |y| x.min(y))),
                )
            });

        let (max, min) = match (max, min) {
            (None, None) => return Err(BackendError::CantFindMaxMin),
            (None, Some(v)) => (f64::MAX, v),
            (Some(v), None) => (v, f64::MIN),
            (Some(v1), Some(v2)) => (v1, v2),
        };

        let voltage_steps = data.len() as u32;

        let mut chart = ChartBuilder::on(&root)
            .x_label_area_size(35)
            .y_label_area_size(40)
            .margin(5)
            .caption(
                "DC Analysis Results",
                ("sans-serif", 50.0).into_font().color(&WHITE),
            )
            .build_cartesian_2d(0u32..voltage_steps, min..max)?;

        chart
            .configure_mesh()
            .x_labels(10)
            .y_labels(10)
            .x_desc("Voltage Steps")
            .x_label_style(("sans-serif", 15).into_font().color(&WHITE))
            .y_label_style(("sans-serif", 15).into_font().color(&WHITE))
            .bold_line_style(&GREY_400)
            .light_line_style(&GREY_800)
            .draw()?;

        // Create a series containing both voltage and current data
        let mut series: Vec<Vec<f64>> = Vec::new();
        let mut units: Vec<Unit> = Vec::new();
        let var_count = data[0].len();
        for _ in 0..var_count {
            series.push(Vec::new());
        }
        for var in 0..var_count {
            units.push(data[0][var].0.unit())
        }
        for var in 0..var_count {
            for step_data in data.iter() {
                series[var].push(step_data[var].1);
            }
        }

        for (idx, var) in series.iter().enumerate() {
            match units[idx] {
                Unit::Volt => {
                    chart
                        .draw_series(LineSeries::new(
                            var.iter().enumerate().map(|(x, &v)| (x as u32, v)),
                            &BLUE,
                        ))?
                        .label("Voltage")
                        .legend(|(x, y)| PathElement::new(vec![(x - 10, y), (x + 10, y)], &BLUE));
                }
                Unit::Ampere => {
                    chart
                        .draw_series(LineSeries::new(
                            var.iter().enumerate().map(|(x, &v)| (x as u32, -v)),
                            &RED,
                        ))?
                        .label("Current")
                        .legend(|(x, y)| PathElement::new(vec![(x - 10, y), (x + 10, y)], &BLUE));
                }
            };
        }

        // Configure and draw the legend
        chart
            .configure_series_labels()
            .background_style(&WHITE.mix(0.8))
            .border_style(&BLACK)
            .draw()?;

        root.present()?;
        Ok(())
    }

    /// Plots the operational simulation results.
    ///
    /// # Parameters
    ///
    /// - `data`: A reference to a vector of tuples where each tuple contains a `Variable` and a `f64`.
    ///
    /// # Returns
    ///
    /// A `Result` which is `Ok` if the plotting operation succeeds, or an `BackendError` if it fails.
    fn plot_op(&self, data: &Vec<(Variable, f64)>) -> Result<(), BackendError> {
        let mut path = PathBuf::from(&self.pth);

        if let Some(stem) = path.file_stem().and_then(|s| s.to_str()) {
            let parent = path.parent().unwrap_or_else(|| Path::new(""));
            let new_file_name = format!("{}_op.svg", stem);
            path = parent.join(new_file_name);
        }

        let root = SVGBackend::new(&path, (1440, 900)).into_drawing_area();
        root.fill(&BLACK)?;

        let (max, min) =
            data.iter()
                .map(|(_, val)| val)
                .fold((None, None), |(max_acc, min_acc), &x| {
                    (
                        Some(max_acc.map_or(x, |y| x.max(y))),
                        Some(min_acc.map_or(x, |y| x.min(y))),
                    )
                });

        let (max, min) = match (max, min) {
            (None, None) => return Err(BackendError::CantFindMaxMin),
            (None, Some(v)) => (f64::MAX, v),
            (Some(v), None) => (v, f64::MIN),
            (Some(v1), Some(v2)) => (v1, v2),
        };

        let mut chart = ChartBuilder::on(&root)
            .x_label_area_size(35)
            .y_label_area_size(40)
            .margin(5)
            .caption(
                "Simulation Results",
                ("sans-serif", 50.0).into_font().color(&WHITE),
            )
            .build_cartesian_2d((0u32..10u32).into_segmented(), min..max)?;

        chart
            .configure_mesh()
            .x_labels(10)
            .y_labels(10)
            .x_desc("Variables")
            .x_label_style(("sans-serif", 15).into_font().color(&WHITE))
            .y_label_style(("sans-serif", 15).into_font().color(&WHITE))
            .bold_line_style(&GREY_400)
            .light_line_style(&GREY_800)
            .draw()?;

        let values = data
            .iter()
            .enumerate()
            .map(|(i, (_var, val))| (i as f64, *val))
            .map(|(x, y)| (x as u32, y));

        let histogram = Histogram::vertical(&chart)
            .style(BLUE.filled())
            .data(values);

        chart.draw_series(histogram)?;

        root.present()?;
        Ok(())
    }
}
