////////////////////////////////////////////////////////////////////////////////////
// --- module uses ---
////////////////////////////////////////////////////////////////////////////////////
use plus_modeled::RateCurve;

////////////////////////////////////////////////////////////////////////////////////
// --- traits ---
////////////////////////////////////////////////////////////////////////////////////
/// Create a plot for a dataset
pub trait PlotData {
    /// Create a plot (svg) for the data set.
    /// For example, implementation of this trait for a [RateCurve](plus_modeled::RateCurve)
    /// would return an SVG of the return step function.
    ///
    ///   * _return_ - An svg image of the plot
    fn plot(&self) -> String;
}

////////////////////////////////////////////////////////////////////////////////////
// --- trait impls ---
////////////////////////////////////////////////////////////////////////////////////
impl PlotData for RateCurve {
    /// Create a plot (svg) for the data set.
    /// For example, implementation of this trait for a [RateCurve](plus_modeled::RateCurve)
    /// would return an SVG of the return step function.
    ///
    ///   * _return_ - An svg image of the plot
    fn plot(&self) -> String {
        // α <fn PlotData::plot for RateCurve>

        use crate::Year;
        use plotters::prelude::*;

        //Extract the x and y coordinates out of RateCurve and Curve to collect in a vector
        let mut x_vec = self
            .curve
            .iter()
            .map(|yv| yv.year as f64)
            .collect::<Vec<f64>>();
        let mut y_vec = self.curve.iter().map(|yv| yv.value).collect::<Vec<f64>>();

        //check curve to see if it is empty
        let mut check_empty_curve = || {
            if self.curve.is_empty() {
                x_vec.push(0.0);
                y_vec.push(0.0);
                //console_log(&format!("Curve is empty. Wait for the user to enter data"));
            }
        };

        check_empty_curve();

        let mut plot_buff = String::with_capacity(2 ^ 11);
        {
            let text_style: TextStyle = ("sans-serif", 20).into();
            let root = SVGBackend::with_string(&mut plot_buff, (300, 275))
                .into_drawing_area()
                .titled("Rate Curve", text_style.clone())
                .expect("");

            let mut chart = ChartBuilder::on(&root)
                .margin(4)
                .set_label_area_size(LabelAreaPosition::Left, 40)
                .set_label_area_size(LabelAreaPosition::Bottom, 40)
                .set_label_area_size(LabelAreaPosition::Right, 0)
                .caption("Rate Curve", ("sans-serif", 30))
                .build_cartesian_2d(
                    (x_vec.first().cloned().unwrap() as f64)
                        ..(x_vec.last().cloned().unwrap() as f64),
                    (y_vec.first().cloned().unwrap())..(y_vec.last()).cloned().unwrap(),
                )
                .unwrap();

            chart
                .configure_mesh()
                .x_labels(10)
                .y_labels(10)
                .disable_mesh()
                .x_label_formatter(&|v| format!("{:.1}", v))
                .y_label_formatter(&|v| format!("{:.1}%", v))
                .draw()
                .unwrap();

            chart
                .draw_series(
                    LineSeries::new(
                        x_vec.iter().enumerate().map(|(i, x)| (*x, y_vec[i])),
                        &GREEN,
                    )
                    .point_size(2),
                )
                .unwrap()
                .label("PDF");

            root.present().expect("Should show");
        }

        plot_buff
        // ω <fn PlotData::plot for RateCurve>
    }
}

/// Unit tests for `plot_data`
#[cfg(test)]
pub mod unit_tests {

    /// Test trait plot_data on RateCurve
    pub mod test_plot_data_on_rate_curve {
        ////////////////////////////////////////////////////////////////////////////////////
        // --- module uses ---
        ////////////////////////////////////////////////////////////////////////////////////
        use test_log::test;

        ////////////////////////////////////////////////////////////////////////////////////
        // --- functions ---
        ////////////////////////////////////////////////////////////////////////////////////
        #[test]
        fn plot() {
            // α <fn test PlotData::plot on RateCurve>

            todo!("Test plot")
            // ω <fn test PlotData::plot on RateCurve>
        }

        // α <mod-def test_plot_data_on_rate_curve>
        // ω <mod-def test_plot_data_on_rate_curve>
    }

    // α <mod-def unit_tests>
    // ω <mod-def unit_tests>
}

// α <mod-def plot_data>
// ω <mod-def plot_data>
