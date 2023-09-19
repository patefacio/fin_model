//! Support for plotting the _cumulative distribution function_ for a distribution

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

    /// Get _cdf(x)_
    ///
    ///   * **x** - Value to get _pdf(z)_
    ///   * _return_ - The point on the _pdf_ for x.
    fn cdf_sigmoid_approx(&self, x: f64) -> Option<f64>;
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

        use crate::scale_by;
        use crate::utils::constants::PLOT_TEXT_STYLE;
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
            let yr = self.cdf_sigmoid_approx(x_lhs);
            let yl = self.cdf_sigmoid_approx(x_rhs);
            let rhs = num_points - i - 1;

            x_vec[i] = x_lhs;
            x_vec[rhs] = x_rhs;
            y_vec[i] = yr.unwrap_or(0.0);
            y_vec[rhs] = yl.unwrap_or(0.0);

            num_sigmas *= sigma_factor;
        }

        tracing::debug!("X -> {x_vec:?}");
        tracing::debug!("Y -> {y_vec:?}");

        let mut plot_buff = String::with_capacity(2 ^ 11);
        {
            let root = SVGBackend::with_string(&mut plot_buff, (300, 275))
                .into_drawing_area()
                .titled(&format!("CDF {}", self), PLOT_TEXT_STYLE.clone())
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
                .x_label_formatter(&|v| format!("{:.1}%", scale_by(*v, 2)))
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

    /// Get _cdf(x)_
    ///
    ///   * **x** - Value to get _pdf(z)_
    ///   * _return_ - The point on the _pdf_ for x.
    #[inline]
    fn cdf_sigmoid_approx(&self, x: f64) -> Option<f64> {
        // α <fn DistributionCdf::cdf_sigmoid_approx for NormalSpec>

        if self.std_dev > 0.0 {
            let correction = 1.70175;
            let temp = (correction * (x - self.mean) / self.std_dev).exp();
            Some(temp / (1.0 + temp))
        } else {
            None
        }

        // ω <fn DistributionCdf::cdf_sigmoid_approx for NormalSpec>
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
        fn cdf_sigmoid_approx() {
            // α <fn test DistributionCdf::cdf_sigmoid_approx on NormalSpec>

            let normal_spec = NormalSpec {
                mean: 10.0,
                std_dev: 1.0,
            };
            assert_eq!(Some(0.5), normal_spec.cdf_sigmoid_approx(10.0));
            assert_eq!(Some(1.0), normal_spec.cdf_sigmoid_approx(100.0));
            assert_eq!(
                None,
                NormalSpec {
                    mean: 10.0,
                    std_dev: 0.0
                }
                .cdf_sigmoid_approx(10.0)
            );

            // ω <fn test DistributionCdf::cdf_sigmoid_approx on NormalSpec>
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
