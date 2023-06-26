////////////////////////////////////////////////////////////////////////////////////
// --- module uses ---
////////////////////////////////////////////////////////////////////////////////////
use plus_modeled::NormalSpec;

////////////////////////////////////////////////////////////////////////////////////
// --- traits ---
////////////////////////////////////////////////////////////////////////////////////
/// Function to provide points on the pdf of a normal.
pub trait DistributionPdf {
    /// Get a chart representing the distribution.
    /// See [Normal Distribution](https://en.wikipedia.org/wiki/Normal_distribution)
    /// for function definition.
    ///
    ///   * **num_points** - Number of points to pull from the distribution
    ///   * _return_ - SVG image of distribution
    fn get_chart(&self, num_points: usize) -> String;
}

////////////////////////////////////////////////////////////////////////////////////
// --- trait impls ---
////////////////////////////////////////////////////////////////////////////////////
impl DistributionPdf for NormalSpec {
    /// Get a chart representing the distribution.
    /// See [Normal Distribution](https://en.wikipedia.org/wiki/Normal_distribution)
    /// for function definition.
    ///
    ///   * **num_points** - Number of points to pull from the distribution
    ///   * _return_ - SVG image of distribution
    fn get_chart(&self, num_points: usize) -> String {
        // α <fn DistributionPdf::get_chart for NormalSpec>

        use plotters::prelude::*;

        // Cap the number of points to range (32, 1024)
        let num_points = num_points.max(32).min(1024);

        // The more points the larger the number of std_devs to consider
        let (max_sigma, sigma_factor) = match num_points {
            num_points if num_points >= 1024 => (5, 0.75),
            _ => (6, 0.75),
        };

        let mut x_vec = vec![0.0; num_points];
        let mut y_vec = vec![0.0; num_points];

        let coefficient = 1.0 / (self.std_dev * (2.0 * std::f64::consts::PI).sqrt());
        let pdf = |z: f64| {
            debug_assert!(z <= self.mean);
            coefficient * (-0.5 * (z - self.mean) * (z - self.mean) / self.std_dev).exp()
        };

        let mut num_sigmas = max_sigma as f64;
        for i in 0..(num_points / 2) {
            let x_lhs = self.mean - self.std_dev * num_sigmas;
            let x_rhs = self.mean + self.std_dev * num_sigmas;
            let y = pdf(x_lhs);
            let rhs = num_points - i - 1;

            x_vec[i] = x_lhs;
            x_vec[rhs] = x_rhs;
            y_vec[i] = y;
            y_vec[rhs] = y;

            num_sigmas *= sigma_factor;
        }

        println!("X -> {x_vec:?}");
        println!("Y -> {y_vec:?}");

        let mut plot_buff = String::with_capacity(2 ^ 11);
        {
            let text_style: TextStyle = ("sans-serif", 18).into();
            let root = SVGBackend::with_string(&mut plot_buff, (300, 275))
                .into_drawing_area()
                .titled("Normal Distribution", text_style.clone())
                .expect("");

            let mut chart = ChartBuilder::on(&root)
                .margin(4)
                .set_label_area_size(LabelAreaPosition::Left, 40)
                .set_label_area_size(LabelAreaPosition::Bottom, 40)
                .set_label_area_size(LabelAreaPosition::Right, 0)
                .build_cartesian_2d(
                    (x_vec[0])..(x_vec[num_points - 1]),
                    0f64..y_vec[num_points / 2],
                )
                .unwrap();

            chart
                .configure_mesh()
                .x_labels(10)
                //.y_labels(5)
                .disable_mesh()
                .x_label_formatter(&|v| format!("{:.1}%", v))
                .y_label_formatter(&|v| String::default())
                .draw()
                .unwrap();

            chart
                .draw_series(LineSeries::new(
                    x_vec.iter().enumerate().map(|(i, x)| (*x, y_vec[i])), 
                    &RED))
                .unwrap()
                .label("PDF");

            root.present().expect("Should present");
        }

        plot_buff
        // ω <fn DistributionPdf::get_chart for NormalSpec>
    }
}

/// Unit tests for `distribution_pdf`
#[cfg(test)]
pub mod unit_tests {

    /// Test trait distribution_pdf on NormalSpec
    pub mod test_distribution_pdf_on_normal_spec {
        ////////////////////////////////////////////////////////////////////////////////////
        // --- module uses ---
        ////////////////////////////////////////////////////////////////////////////////////
        use test_log::test;

        ////////////////////////////////////////////////////////////////////////////////////
        // --- functions ---
        ////////////////////////////////////////////////////////////////////////////////////
        #[test]
        fn get_chart() {
            // α <fn test DistributionPdf::get_chart on NormalSpec>

            println!(
                "Data\n{}",
                NormalSpec {
                    mean: 4.0,
                    std_dev: 1.0
                }
                .get_chart(200)
            );

            // ω <fn test DistributionPdf::get_chart on NormalSpec>
        }

        // α <mod-def test_distribution_pdf_on_normal_spec>
        use super::*;
        // ω <mod-def test_distribution_pdf_on_normal_spec>
    }

    // α <mod-def unit_tests>
    use super::*;
    // ω <mod-def unit_tests>
}

// α <mod-def distribution_pdf>
// ω <mod-def distribution_pdf>
