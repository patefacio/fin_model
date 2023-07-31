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
    fn get_pdf_chart(&self, num_points: usize) -> String;

    /// Get _pdf(z)_
    ///
    ///   * **z** - Value to get _pdf(z)_
    ///   * _return_ - The point on the _pdf_ for z.
    fn pdf(&self, z: f64) -> f64;
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
    fn get_pdf_chart(&self, num_points: usize) -> String {
        // α <fn DistributionPdf::get_pdf_chart for NormalSpec>
        use crate::scale_by;
        use crate::utils::constants::plot_text_style;
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
            let y = self.pdf(x_lhs);
            let rhs = num_points - i - 1;

            x_vec[i] = x_lhs;
            x_vec[rhs] = x_rhs;
            y_vec[i] = y;
            y_vec[rhs] = y;

            num_sigmas *= sigma_factor;
        }

        tracing::debug!("{self:?} -> X -> {x_vec:?}");
        tracing::debug!("Y -> {y_vec:?}");

        leptos::log!("Creating pdf for `{self:?}`");

        let mut plot_buff = String::with_capacity(2 ^ 11);
        {
            let root = SVGBackend::with_string(&mut plot_buff, (300, 275))
                .into_drawing_area()
                .titled(&format!("PDF {}", self), plot_text_style.clone())
                .expect("");

            let mut chart = ChartBuilder::on(&root)
                .margin(4)
                .set_label_area_size(LabelAreaPosition::Left, 40)
                .set_label_area_size(LabelAreaPosition::Bottom, 40)
                .set_label_area_size(LabelAreaPosition::Right, 0)
                .build_cartesian_2d(
                    (x_vec[0])..(x_vec[num_points - 1]),
                    0f64..y_vec[num_points / 2] * 1.1,
                )
                .unwrap();

            chart
                .configure_mesh()
                .x_labels(10)
                //.y_labels(5)
                .disable_mesh()
                .x_label_formatter(&|v| format!("{:.1}%", scale_by(*v, 2)))
                .y_label_formatter(&|_v| String::default())
                .draw()
                .unwrap();

            chart
                .draw_series(
                    LineSeries::new(x_vec.iter().enumerate().map(|(i, x)| (*x, y_vec[i])), &RED)
                        .point_size(1),
                )
                .unwrap()
                .label("PDF");

            root.present().expect("Should present");
        }

        plot_buff

        // ω <fn DistributionPdf::get_pdf_chart for NormalSpec>
    }

    /// Get _pdf(z)_
    ///
    ///   * **z** - Value to get _pdf(z)_
    ///   * _return_ - The point on the _pdf_ for z.
    #[inline]
    fn pdf(&self, z: f64) -> f64 {
        // α <fn DistributionPdf::pdf for NormalSpec>
        debug_assert!(z <= self.mean);
        let coefficient = 1.0 / (self.std_dev * (2.0 * std::f64::consts::PI).sqrt());
        coefficient
            * (-0.5 * (z - self.mean) * (z - self.mean) / (self.std_dev * self.std_dev)).exp()
        // ω <fn DistributionPdf::pdf for NormalSpec>
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
        fn pdf() {
            // α <fn test DistributionPdf::pdf on NormalSpec>

            /*  Test motivated by this bard interaction:

            > I have a normal distribution with mu=0.1 and standard deviation of 0.2. What
            > is the probability distribution function value for 0.05?

            ---

            > Sure, I can help you with that.

            > The probability distribution function value for 0.05 in a normal distribution
            > with mu=0.1 and standard deviation of 0.2 is 1.9333405840142461.

            > To calculate this, we can use the following formula:

            ```
            pdf = 1 / math.sqrt(2 * math.pi * sigma**2) * math.exp(-(x - mu)**2 / (2 * sigma**2))
            ```

            where:

            - pdf is the probability distribution function value
            - x is the value to evaluate the PDF at (in this case, 0.05)
            - mu is the mean of the distribution (in this case, 0.1)
            - sigma is the standard deviation of the distribution (in this case, 0.2)

            Plugging these values into the formula, we get:

            ```
            pdf = 1 / math.sqrt(2 * math.pi * 0.2**2) * math.exp(-(0.05 - 0.1)**2 / (2 * 0.2**2))
            ```

            ```
            pdf = 1.9333405840142461
            ```

            Therefore, the probability distribution function value for 0.05 in a normal
            distribution with mu=0.1 and standard deviation of 0.2 is 1.9333405840142461.

             */

            let normal_spec = NormalSpec {
                mean: 0.1,
                std_dev: 0.2,
            };
            let pdf = normal_spec.pdf(0.05);
            assert_eq!(pdf, 1.9333405840142461);
            // ω <fn test DistributionPdf::pdf on NormalSpec>
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
