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
        use plus_modeled::RateCurve;
        use plus_modeled::YearValue;
        use std::slice::Windows;

        //Check to see if the curve is empty -> no values can be plotted if the user hasn't given any input
        if self.curve.is_empty() {
            return String::default();
        }

        let mut vec_slice: Vec<(f64, f64)> = Vec::with_capacity(self.curve.len());
        let mut min_y = f64::MAX;
        let mut max_y = f64::MIN;

        //windows returns a slice of the vector [left: (2005, 4.5), right: (2006, 2.6)]
        for slice in self.curve.as_slice().windows(2) {
            let (left, right) = (slice.first().unwrap(), slice.last().unwrap());
            vec_slice.push((left.year as f64, left.value));
            vec_slice.push((right.year as f64, left.value));
            min_y = left.value.min(min_y);
            max_y = left.value.max(max_y);
        }
        let min_x = self.curve.first().unwrap().year;
        let max_x = self.curve.last().unwrap().year;
        vec_slice.push(
            self.curve
                .last()
                .map(|year_value| (year_value.year as f64, year_value.value))
                .unwrap(),
        );

        //push the last y value in the curve to the graph since the vector could contain an odd number of
        //points, thereby failing to share the window of the next point
        let last_y = self.curve.last().unwrap().value;
        min_y = min_y.min(last_y);
        max_y = max_y.max(last_y);

        //This is all code for the writing and styling of the graph from the plotters library
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
                .build_cartesian_2d((min_x as f64..max_x as f64), (min_y..max_y))
                .unwrap();

            chart
                .configure_mesh()
                .x_labels(10)
                .y_labels(10)
                .disable_mesh()
                .x_label_formatter(&|year| format!("{year}"))
                .y_label_formatter(&|v| format!("{:.1}%", v * 100.0))
                .draw()
                .unwrap();

            chart
                .draw_series(LineSeries::new(vec_slice, &BLUE).point_size(2))
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
