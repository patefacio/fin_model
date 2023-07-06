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
        use std::slice::Windows;
        use plus_modeled::YearValue;

        //Extract the x and y coordinates out of RateCurve and Curve to collect in a vector
        // let mut x_vec = self
        //     .curve
        //     .iter()
        //     .map(|yv| yv.year as f64)
        //     .collect::<Vec<f64>>();
        // let mut y_vec = self.curve.iter().map(|yv| yv.value).collect::<Vec<f64>>();

        // let step_diagram = /*(x_vec: Vec<f64>, y_vec: Vec<f64>) -> RateCurve*/ {
        //     ChartBuilder::build_cartesian_2d(&mut self, x_vec[0], y_vec[0]);
        //     ChartBuilder::build_cartesian_2d(&mut self, x_vec[1], y_vec[0]);
        //     ChartBuilder::build_cartesian_2d(&mut self, x_vec[1], y_vec[1]);

        // };

        // for (first, second) in self.curve.iter().windows(2) {
        //     let vec_slice = self.curve;
        //     let iter = vec_slice.windows(2);
        //     vec_slice.push(self.curve)
        // }

        //check curve to see if it is empty
        // let mut check_empty_curve = || {
        //     if self.curve.is_empty() {
                
        //         //console_log(&format!("Curve is empty. Wait for the user to enter data"));
        //     }
        // };

        // check_empty_curve();

        if self.curve.is_empty() {
            return String::default();
        }


        let mut vec_slice: Vec<(f64, f64)> = Vec::with_capacity(self.curve.len());
        let mut min_y = f64::MAX;
        let mut max_y = f64::MIN;
        for slice in self.curve.as_slice().windows(2) {
            let (left, right) = (slice.first().unwrap(), slice.last().unwrap());
            vec_slice.push((left.year as f64, left.value));
            vec_slice.push((right.year as f64, left.value));
            min_y = left.value.min(min_y);
            max_y = left.value.max(max_y);
        }
        let min_x = self.curve.first().unwrap().year;
        let max_x = self.curve.last().unwrap().year;
        

        
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
                    (min_x as f64 ..max_x as f64 ),
                    (min_y..max_y),
                )
                .unwrap();

            chart
                .configure_mesh()
                .x_labels(10)
                .y_labels(10)
                .disable_mesh()
                .x_label_formatter(&|v| format!("{:.1}", v))
                .y_label_formatter(&|v| format!("{:.1}%", v * 100.0))
                .draw()
                .unwrap();

            
            //let foo = vec_slice.iter();
            chart
                .draw_series(
                    LineSeries::new(
                        vec_slice,
                        &BLUE,
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