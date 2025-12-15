use std::collections::HashSet;
use std::path::{Path, PathBuf};

use full_palette::{LIGHTBLUE, RED_500};
use plotters::style::full_palette::GREEN_500;
use plotters::{
    backend::SVGBackend,
    prelude::*,
    style::full_palette::{GREY_400, GREY_800},
};

use super::Backend;
use crate::models::{Unit, Variable};
use crate::sim::options::SimulationOption;
use crate::sim::simulation_result::{Sim, SimulationResults};
use crate::{backends::BackendError, spot::*};

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
        for sim in res.results.iter() {
            self.select_output(sim, res.options.clone())?;
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
    fn select_output(&self, sim: &Sim, options: Vec<SimulationOption>) -> Result<(), BackendError> {
        match sim {
            Sim::Op(data) => self.plot_op(data)?,
            Sim::Dc(data) => self.plot_dc(data, options)?,
            Sim::Ac(data) => self.plot_ac(data)?,
            Sim::Tran(data) => self.plot_tran(data)?,
        }
        Ok(())
    }

    /// Plots the DC simulation results.
    ///
    /// # Parameters
    ///
    /// - `data`: A reference to a vector of vectors of tuples where each tuple contains a `Variable` and a `Numeric`.
    ///
    /// # Returns
    ///
    /// A `Result` which is `Ok` if the plotting operation succeeds, or an `BackendError` if it fails.
    fn plot_dc(
        &self,
        data: &Vec<Vec<(Variable, Numeric)>>,
        options: Vec<SimulationOption>,
    ) -> Result<(), BackendError> {
        let mut path = PathBuf::from(&self.pth);
        path.set_extension("svg");

        // Add the suffix before the extension
        let stem = path.file_stem().and_then(|s| s.to_str()).unwrap_or("");
        let parent = path.parent().unwrap_or(Path::new(""));
        let new_file_name = format!("{stem}_dc.svg");
        path = parent.join(new_file_name);

        let root = SVGBackend::new(&path, (1440, 900)).into_drawing_area();
        root.fill(&BLACK)?;

        // Collect the variable names specified in the options.
        let mut filtered_headers = HashSet::new();
        for option in options {
            if let SimulationOption::Out(vars) = option {
                for var in vars {
                    filtered_headers.insert(var);
                }
            }
        }

        // If no filtering is specified, use all headers.
        if filtered_headers.is_empty() {
            for step_data in data {
                for (var, _) in step_data {
                    filtered_headers.insert(var.name());
                }
            }
        }

        let filtered_data: Vec<Vec<(Variable, Numeric)>> = data
            .iter()
            .map(|step_data| {
                step_data
                    .iter()
                    .filter(|(var, _)| filtered_headers.contains(&var.name()))
                    .cloned()
                    .collect()
            })
            .collect();

        let (max, min) = filtered_data
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
            (None, Some(v)) => (Numeric::MAX, v),
            (Some(v), None) => (v, Numeric::MIN),
            (Some(v1), Some(v2)) => (v1, v2),
        };

        let voltage_steps = filtered_data.len() as u32;

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
            .bold_line_style(GREY_400)
            .light_line_style(GREY_800)
            .draw()?;

        // Create a series containing both voltage and current data
        let mut series: Vec<Vec<Numeric>> = Vec::new();
        let mut units: Vec<Unit> = Vec::new();
        let var_count = filtered_data[0].len();
        for _ in 0..var_count {
            series.push(Vec::new());
        }
        for var in 0..var_count {
            units.push(filtered_data[0][var].0.unit())
        }
        for var in 0..var_count {
            for step_data in filtered_data.iter() {
                series[var].push(step_data[var].1);
            }
        }

        for (idx, var) in series.iter().enumerate() {
            match units[idx] {
                Unit::Volt => {
                    chart
                        .draw_series(LineSeries::new(
                            var.iter().enumerate().map(|(x, &v)| (x as u32, v)),
                            LIGHTBLUE,
                        ))?
                        .label("Voltage")
                        .legend(|(x, y)| PathElement::new(vec![(x - 10, y), (x + 10, y)], BLUE));
                }
                Unit::Ampere => {
                    chart
                        .draw_series(LineSeries::new(
                            var.iter().enumerate().map(|(x, &v)| (x as u32, -v)),
                            RED_500,
                        ))?
                        .label("Current")
                        .legend(|(x, y)| PathElement::new(vec![(x - 10, y), (x + 10, y)], BLUE));
                }
                Unit::None => {
                    chart
                        .draw_series(LineSeries::new(
                            var.iter().enumerate().map(|(x, &v)| (x as u32, -v)),
                            &GREEN_500,
                        ))?
                        .label(" ")
                        .legend(|(x, y)| PathElement::new(vec![(x - 10, y), (x + 10, y)], BLUE));
                }
            };
        }

        // Configure and draw the legend
        chart
            .configure_series_labels()
            .background_style(WHITE.mix(0.8))
            .border_style(BLACK)
            .draw()?;

        root.present()?;
        Ok(())
    }

    /// Plots the transient simulation results.
    ///
    /// # Parameters
    ///
    /// - `data`: A reference to a vector of tuples where each tuple contains a timestep and a vector of tuples with `Variable` and `Numeric`.
    ///
    /// # Returns
    ///
    /// A `Result` which is `Ok` if the plotting operation succeeds, or an `BackendError` if it fails.
    fn plot_tran(&self, data: &[(Numeric, Vec<(Variable, Numeric)>)]) -> Result<(), BackendError> {
        let mut path = PathBuf::from(&self.pth);
        path.set_extension("svg");
        let stem = path.file_stem().and_then(|s| s.to_str()).unwrap_or("");
        let parent = path.parent().unwrap_or(Path::new(""));
        let new_file_name = format!("{stem}_tran.svg");
        path = parent.join(new_file_name);

        let root = SVGBackend::new(&path, (1440, 900)).into_drawing_area();
        root.fill(&BLACK)?;

        let (min_y, max_y) = data
            .iter()
            .flat_map(|(_, vars)| vars.iter().map(|(_, val)| *val))
            .fold((None, None), |(min, max), val| {
                (
                    Some(min.map_or(val, |y: Numeric| y.min(val))),
                    Some(max.map_or(val, |y: Numeric| y.max(val))),
                )
            });

        let (min_y, max_y) = match (min_y, max_y) {
            (None, None) => return Err(BackendError::PlotError("Plot empty".into())),
            (None, Some(v)) => (Numeric::MIN, v),
            (Some(v), None) => (v, Numeric::MAX),
            (Some(v1), Some(v2)) => (v1, v2),
        };

        let (min_x, max_x) =
            data.iter()
                .map(|(time, _)| *time)
                .fold((None, None), |(min, max), val| {
                    (
                        Some(min.map_or(val, |y: Numeric| y.min(val))),
                        Some(max.map_or(val, |y: Numeric| y.max(val))),
                    )
                });

        let (min_x, max_x) = match (min_x, max_x) {
            (None, None) => return Err(BackendError::PlotError("Plot empty".into())),
            (None, Some(v)) => (Numeric::MIN, v),
            (Some(v), None) => (v, Numeric::MAX),
            (Some(v1), Some(v2)) => (v1, v2),
        };

        let mut chart = ChartBuilder::on(&root)
            .x_label_area_size(35)
            .y_label_area_size(40)
            .margin(5)
            .caption(
                "Transient Analysis Results",
                ("sans-serif", 50.0).into_font().color(&WHITE),
            )
            .build_cartesian_2d(min_x..max_x, min_y..max_y)?;

        chart
            .configure_mesh()
            .x_labels(10)
            .y_labels(10)
            .x_desc("Time")
            .y_desc("Value")
            .x_label_style(("sans-serif", 15).into_font().color(&WHITE))
            .y_label_style(("sans-serif", 15).into_font().color(&WHITE))
            .bold_line_style(GREY_400)
            .light_line_style(GREY_800)
            .draw()?;

        let var_count = data[0].1.len();
        for var_idx in 0..var_count {
            let var_name = data[0].1[var_idx].0.name().to_string();
            let unit = data[0].1[var_idx].0.unit();

            let points: Vec<(Numeric, Numeric)> = data
                .iter()
                .map(|(time, vars)| (*time, vars[var_idx].1))
                .collect();

            match unit {
                Unit::Volt => {
                    chart
                        .draw_series(LineSeries::new(points, BLUE))?
                        .label(format!("{} (Voltage)", var_name))
                        .legend(|(x, y)| PathElement::new(vec![(x - 10, y), (x + 10, y)], BLUE));
                }
                Unit::Ampere => {
                    chart
                        .draw_series(LineSeries::new(points, RED))?
                        .label(format!("{} (Current)", var_name))
                        .legend(|(x, y)| PathElement::new(vec![(x - 10, y), (x + 10, y)], RED));
                }
                Unit::None => {
                    chart
                        .draw_series(LineSeries::new(points, GREEN))?
                        .label(format!("{} (Value)", var_name))
                        .legend(|(x, y)| PathElement::new(vec![(x - 10, y), (x + 10, y)], GREEN));
                }
            };
        }

        chart
            .configure_series_labels()
            .background_style(WHITE.mix(0.8))
            .border_style(BLACK)
            .draw()?;

        root.present()?;
        Ok(())
    }

    fn plot_ac(
        &self,
        data: &[(Numeric, Vec<(Variable, ComplexNumeric)>)],
    ) -> Result<(), BackendError> {
        let mut path = PathBuf::from(&self.pth);
        path.set_extension("svg");

        // Add the suffix before the extension
        let stem = path.file_stem().and_then(|s| s.to_str()).unwrap_or("");
        let parent = path.parent().unwrap_or(Path::new(""));
        let new_file_name = format!("{stem}_ac.svg");
        path = parent.join(new_file_name);

        let root = SVGBackend::new(&path, (1440, 900)).into_drawing_area();
        root.fill(&BLACK)?;

        let (max_gain, min_gain, max_phase, min_phase) = data
            .iter()
            .flat_map(|(_, vec)| vec.iter())
            .map(|(_, val)| {
                let gain = 20.0 * val.norm().log10();
                let phase = val.arg().to_degrees();
                (gain, phase)
            })
            .fold(
                (None, None, None, None),
                |(max_g, min_g, max_p, min_p), (gain, phase)| {
                    (
                        Some(max_g.map_or(gain, |y| gain.max(y))),
                        Some(min_g.map_or(gain, |y| gain.min(y))),
                        Some(max_p.map_or(phase, |y| phase.max(y))),
                        Some(min_p.map_or(phase, |y| phase.min(y))),
                    )
                },
            );

        let (max_gain, min_gain) = match (max_gain, min_gain) {
            (None, None) => return Err(BackendError::PlotError("Plot empty".into())),
            (None, Some(v)) => (Numeric::MAX, v),
            (Some(v), None) => (v, Numeric::MIN),
            (Some(v1), Some(v2)) => (v1, v2),
        };

        let (max_phase, min_phase) = match (max_phase, min_phase) {
            (None, None) => return Err(BackendError::PlotError("Plot empty".into())),
            (None, Some(v)) => (Numeric::MAX, v),
            (Some(v), None) => (v, Numeric::MIN),
            (Some(v1), Some(v2)) => (v1, v2),
        };

        let fmin = data.iter().map(|(freq, _)| *freq as u32).min()
            .expect("No frequency data available for plotting. This indicates empty AC analysis results.");
        let fmax = data.iter().map(|(freq, _)| *freq as u32).max()
            .expect("No frequency data available for plotting. This indicates empty AC analysis results.");

        let (upper, lower) = root.split_vertically(450);

        let mut chart_gain = ChartBuilder::on(&upper)
            .x_label_area_size(35)
            .y_label_area_size(40)
            .margin(5)
            .caption(
                "AC Analysis Results (Gain)",
                ("sans-serif", 50.0).into_font().color(&WHITE),
            )
            .build_cartesian_2d(fmin..fmax, min_gain..max_gain)?;

        chart_gain
            .configure_mesh()
            .x_labels(10)
            .y_labels(10)
            .x_desc("Frequency Steps")
            .x_label_style(("sans-serif", 15).into_font().color(&WHITE))
            .y_label_style(("sans-serif", 15).into_font().color(&WHITE))
            .bold_line_style(GREY_400)
            .light_line_style(GREY_800)
            .draw()?;

        let mut chart_phase = ChartBuilder::on(&lower)
            .x_label_area_size(35)
            .y_label_area_size(40)
            .margin(5)
            .caption(
                "AC Analysis Results (Phase)",
                ("sans-serif", 50.0).into_font().color(&WHITE),
            )
            .build_cartesian_2d(fmin..fmax, min_phase..max_phase)?;

        chart_phase
            .configure_mesh()
            .x_labels(10)
            .y_labels(10)
            .x_desc("Frequency Steps")
            .x_label_style(("sans-serif", 15).into_font().color(&WHITE))
            .y_label_style(("sans-serif", 15).into_font().color(&WHITE))
            .bold_line_style(GREY_400)
            .light_line_style(GREY_800)
            .draw()?;

        // Create a series containing both gain and phase data
        let mut series_gain: Vec<Vec<Numeric>> = Vec::new();
        let mut series_phase: Vec<Vec<Numeric>> = Vec::new();
        let mut units: Vec<Unit> = Vec::new();
        let var_count = data[0].1.len();
        for _ in 0..var_count {
            series_gain.push(Vec::new());
            series_phase.push(Vec::new());
        }
        for var in 0..var_count {
            units.push(data[0].1[var].0.unit())
        }
        for var in 0..var_count {
            for (_, step_data) in data.iter() {
                let gain = 20.0 * step_data[var].1.norm().log10();
                let phase = step_data[var].1.arg().to_degrees();
                series_gain[var].push(gain);
                series_phase[var].push(phase);
            }
        }

        for (idx, var) in series_gain.iter().enumerate() {
            match units[idx] {
                Unit::Volt => {
                    chart_gain
                        .draw_series(LineSeries::new(
                            var.iter().enumerate().map(|(x, &v)| (x as u32, v)),
                            BLUE,
                        ))?
                        .label(format!("Voltage {}", idx + 1))
                        .legend(|(x, y)| PathElement::new(vec![(x - 10, y), (x + 10, y)], BLUE));
                }
                Unit::Ampere => {
                    chart_gain
                        .draw_series(LineSeries::new(
                            var.iter().enumerate().map(|(x, &v)| (x as u32, v)),
                            RED,
                        ))?
                        .label(format!("Current {}", idx + 1))
                        .legend(|(x, y)| PathElement::new(vec![(x - 10, y), (x + 10, y)], RED));
                }
                Unit::None => {
                    chart_gain
                        .draw_series(LineSeries::new(
                            var.iter().enumerate().map(|(x, &v)| (x as u32, v)),
                            GREEN,
                        ))?
                        .label(format!("Current {}", idx + 1))
                        .legend(|(x, y)| PathElement::new(vec![(x - 10, y), (x + 10, y)], RED));
                }
            };
        }

        for (idx, var) in series_phase.iter().enumerate() {
            match units[idx] {
                Unit::Volt => {
                    chart_phase
                        .draw_series(LineSeries::new(
                            var.iter().enumerate().map(|(x, &v)| (x as u32, v)),
                            BLUE,
                        ))?
                        .label(format!("Voltage {}", idx + 1))
                        .legend(|(x, y)| PathElement::new(vec![(x - 10, y), (x + 10, y)], BLUE));
                }
                Unit::Ampere => {
                    chart_phase
                        .draw_series(LineSeries::new(
                            var.iter().enumerate().map(|(x, &v)| (x as u32, v)),
                            RED,
                        ))?
                        .label(format!("Current {}", idx + 1))
                        .legend(|(x, y)| PathElement::new(vec![(x - 10, y), (x + 10, y)], RED));
                }
                Unit::None => {
                    chart_phase
                        .draw_series(LineSeries::new(
                            var.iter().enumerate().map(|(x, &v)| (x as u32, v)),
                            RED,
                        ))?
                        .label(format!("Value {}", idx + 1))
                        .legend(|(x, y)| PathElement::new(vec![(x - 10, y), (x + 10, y)], RED));
                }
            };
        }

        // Configure and draw the legend for both charts
        chart_gain
            .configure_series_labels()
            .background_style(WHITE.mix(0.8))
            .border_style(BLACK)
            .draw()?;

        chart_phase
            .configure_series_labels()
            .background_style(WHITE.mix(0.8))
            .border_style(BLACK)
            .draw()?;

        root.present()?;
        Ok(())
    }

    /// Plots the operational simulation results.
    ///
    /// # Parameters
    ///
    /// - `data`: A reference to a vector of tuples where each tuple contains a `Variable` and a `Numeric`.
    ///
    /// # Returns
    ///
    /// A `Result` which is `Ok` if the plotting operation succeeds, or an `BackendError` if it fails.
    fn plot_op(&self, data: &[(Variable, Numeric)]) -> Result<(), BackendError> {
        let mut path = PathBuf::from(&self.pth);

        if let Some(stem) = path.file_stem().and_then(|s| s.to_str()) {
            let parent = path.parent().unwrap_or_else(|| Path::new(""));
            let new_file_name = format!("{stem}_op.svg");
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
            (None, Some(v)) => (Numeric::MAX, v),
            (Some(v), None) => (v, Numeric::MIN),
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
            .bold_line_style(GREY_400)
            .light_line_style(GREY_800)
            .draw()?;

        let values = data
            .iter()
            .enumerate()
            .map(|(i, (_var, val))| (i as Numeric, *val))
            .map(|(x, y)| (x as u32, y));

        let histogram = Histogram::vertical(&chart)
            .style(BLUE.filled())
            .data(values);

        chart.draw_series(histogram)?;

        root.present()?;
        Ok(())
    }
}
