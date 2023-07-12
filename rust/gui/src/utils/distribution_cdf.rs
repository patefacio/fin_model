////////////////////////////////////////////////////////////////////////////////////
// --- module uses ---
////////////////////////////////////////////////////////////////////////////////////
use plus_modeled::NormalSpec;

////////////////////////////////////////////////////////////////////////////////////
// --- traits ---
////////////////////////////////////////////////////////////////////////////////////
/// Function to provide points on the cdf of a normal.
pub trait DistributionCdf {
    /// Get a chart representing the *cdf* of the distribution.
    /// See [Normal Distribution](https://en.wikipedia.org/wiki/Normal_distribution)
    /// for function definition.
    ///
    ///   * **num_points** - Number of points to pull from the distribution
    ///   * _return_ - SVG image of distribution
    fn get_cdf_chart(&self, num_points: usize) -> String;

    /// Gets the cdf(x) value using an approximation
    ///
    ///   * **x** - x to get cdf value
    ///   * **sigma** - Standard deviation
    ///   * **mu** - Normal mean
    ///   * _return_ - Approximate value of the cdf
    fn sigmoid_approx(x: f64, sigma: f64, mu: f64) -> f64;
}

pub fn sigmoid_approx(x: f64, sigma: f64, mu: f64) -> f64 {
    // α <fn DistributionCdf::sigmoid_approx for NormalSpec>
    assert!(sigma != 0.0);
    let correction = 1.70175;
    let temp = (correction * (x - mu) / sigma).exp();
    return temp / (1.0 + temp);
    // ω <fn DistributionCdf::sigmoid_approx for NormalSpec>
}

////////////////////////////////////////////////////////////////////////////////////
// --- trait impls ---
////////////////////////////////////////////////////////////////////////////////////
impl DistributionCdf for NormalSpec {



    /// Get a chart representing the *cdf* of the distribution.
    /// See [Normal Distribution](https://en.wikipedia.org/wiki/Normal_distribution)
    /// for function definition.
    ///
    ///   * **num_points** - Number of points to pull from the distribution
    ///   * _return_ - SVG image of distribution
    fn get_cdf_chart(&self, num_points: usize) -> String {
        // α <fn DistributionCdf::get_cdf_chart for NormalSpec>

        use plotters::prelude::*;

        // Cap the number of points to range (32, 1024)
        let num_points = num_points.max(32).min(1024);

        // The more points the larger the number of std_devs to consider
        let (max_sigma, sigma_factor) = match num_points {
            num_points if num_points >= 1024 => (3.5, 0.85),
            _ => (3.5, 0.80),
        };

        let mut x_vec = vec![0.0; num_points];
        let mut y_vec = vec![0.0; num_points];


        let mut num_sigmas = max_sigma as f64;
        for i in 0..(num_points / 2) {
            let x_lhs = self.mean - self.std_dev * num_sigmas;
            let x_rhs = self.mean + self.std_dev * num_sigmas;
            let yr = sigmoid_approx(x_lhs, self.std_dev, self.mean);
            let yl = sigmoid_approx(x_rhs, self.std_dev, self.mean);
            let rhs = num_points - i - 1;

            x_vec[i] = x_lhs;
            x_vec[rhs] = x_rhs;
            y_vec[i] = yr;
            y_vec[rhs] = yl;

            num_sigmas *= sigma_factor;
        }

        tracing::debug!("X -> {x_vec:?}");
        tracing::debug!("Y -> {y_vec:?}");

        let mut plot_buff = String::with_capacity(2 ^ 11);
        {
            let text_style: TextStyle = ("sans-serif", 18).into();
            let root = SVGBackend::with_string(&mut plot_buff, (300, 275))
                .into_drawing_area()
                .titled("Cumulative Distribution", text_style.clone())
                .expect("");

            let mut chart = ChartBuilder::on(&root)
                .margin(4)
                .set_label_area_size(LabelAreaPosition::Left, 40)
                .set_label_area_size(LabelAreaPosition::Bottom, 40)
                .set_label_area_size(LabelAreaPosition::Right, 0)
                .build_cartesian_2d(
                    (x_vec[0])..(x_vec[num_points - 1]),
                    0f64..y_vec[num_points / 2] * 2.1,
                )
                .unwrap();

            chart
                .configure_mesh()
                .x_labels(10)
                .y_labels(5)
                .disable_mesh()
                .x_label_formatter(&|v| format!("{:.1}%", v))
                .y_label_formatter(&|v| format!("{:.1}", v))
                .draw()
                .unwrap();

            chart
                .draw_series(
                    LineSeries::new(x_vec.iter().enumerate().map(|(i, x)| (*x, y_vec[i])), &BLUE)
                        .point_size(1),
                )
                .unwrap()
                .label("CDF");

            root.present().expect("Should present");
        }

        plot_buff
        // ω <fn DistributionCdf::get_cdf_chart for NormalSpec>
    }

    fn sigmoid_approx(x: f64, sigma: f64, mu: f64) -> f64 {
        return sigmoid_approx(x, sigma, mu);
    }

}

/// Unit tests for `distribution_cdf`
#[cfg(test)]
pub mod unit_tests {

    /// Test trait distribution_cdf on NormalSpec
    pub mod test_distribution_cdf_on_normal_spec {
        ////////////////////////////////////////////////////////////////////////////////////
        // --- module uses ---
        ////////////////////////////////////////////////////////////////////////////////////
        use test_log::test;

        ////////////////////////////////////////////////////////////////////////////////////
        // --- functions ---
        ////////////////////////////////////////////////////////////////////////////////////
        #[test]
        fn test_get_cdf_chart() {
            // α <fn test DistributionCdf::get_cdf_chart on NormalSpec>
            println!(
                "Data\n{}",
                NormalSpec {
                    mean: 4.0,
                    std_dev: 1.0
                }
                .get_cdf_chart(200)
            );
            // ω <fn test DistributionCdf::get_cdf_chart on NormalSpec>
        }

        #[test]
        fn test_sigmoid_approx() {
            // α <fn test DistributionCdf::sigmoid_approx on NormalSpec>
            //todo!("Test sigmoid_approx")
            assert_eq!(0.5, sigmoid_approx(10.0, 1.0, 10.0) as f64);
            assert_eq!(1.0, sigmoid_approx(100.0, 1.0, 10.0) as f64);
            
            // ω <fn test DistributionCdf::sigmoid_approx on NormalSpec>
        }

        // α <mod-def test_distribution_cdf_on_normal_spec>
        use super::*;
        // ω <mod-def test_distribution_cdf_on_normal_spec>
    }

    // α <mod-def unit_tests>
    use super::*;
    // ω <mod-def unit_tests>
}

// α <mod-def distribution_cdf>
// ω <mod-def distribution_cdf>
