////////////////////////////////////////////////////////////////////////////////////
// --- module uses ---
////////////////////////////////////////////////////////////////////////////////////
use plotters::prelude::RGBColor;
use plus_modeled::NormalSpec;

////////////////////////////////////////////////////////////////////////////////////
// --- traits ---
////////////////////////////////////////////////////////////////////////////////////
/// Function to plot historic risk returns together for context.
pub trait HistoricRiskReturnPlot {
    /// Get a chart showing the set of historic risk/return values,
    /// plotted together.
    ///
    ///   * **historic_values** - Set of historic risk return values.
    ///   * _return_ - SVG image of the points on a plot.
    fn get_historic_plot(&self, historic_values: &[HistoricRiskReturn]) -> String;
}

////////////////////////////////////////////////////////////////////////////////////
// --- structs ---
////////////////////////////////////////////////////////////////////////////////////
/// Contains a (risk, return) pair and a label
#[derive(Debug, Clone)]
pub struct HistoricRiskReturn {
    /// Historic risk/return value
    pub risk_return: (f64, f64),
    /// The label for the value (e.g. EquityMarket)
    pub label: String,
    /// Color in the legend
    pub color: RGBColor,
}

////////////////////////////////////////////////////////////////////////////////////
// --- trait impls ---
////////////////////////////////////////////////////////////////////////////////////
impl HistoricRiskReturnPlot for NormalSpec {
    /// Get a chart showing the set of historic risk/return values,
    /// plotted together.
    ///
    ///   * **historic_values** - Set of historic risk return values.
    ///   * _return_ - SVG image of the points on a plot.
    fn get_historic_plot(&self, historic_values: &[HistoricRiskReturn]) -> String {
        // α <fn HistoricRiskReturnPlot::get_historic_plot for NormalSpec>
        use crate::scale_by;
        use crate::utils::constants::plot_text_style;
        use plotters::prelude::*;

        let x_vec = historic_values
            .iter()
            .map(|hv| hv.risk_return.0)
            .collect::<Vec<_>>();

        let y_vec = historic_values
            .iter()
            .map(|hv| hv.risk_return.1)
            .collect::<Vec<_>>();

        let max_x = x_vec
            .iter()
            .max_by(|a, b| a.total_cmp(b))
            .unwrap_or_else(|| &f64::MIN);
        let max_y = y_vec
            .iter()
            .max_by(|a, b| a.total_cmp(b))
            .unwrap_or_else(|| &f64::MIN);

        let mut plot_buff = String::with_capacity(2 ^ 11);
        {
            let root = SVGBackend::with_string(&mut plot_buff, (470, 470))
                .into_drawing_area()
                .titled("Historic Risk Return", plot_text_style.clone())
                .expect("");

            let root = root.margin(10, 10, 10, 10);

            let mut chart = ChartBuilder::on(&root)
                .margin(4)
                .x_label_area_size(30)
                .y_label_area_size(60)
                .build_cartesian_2d(0f64..(max_x * 1.4), 0f64..(max_y * 1.1))
                .unwrap();

            chart
                .configure_mesh()
                .x_labels(5)
                .y_labels(5)
                .x_label_formatter(&|x| format!("{}%", scale_by(*x, 2).round()))
                .y_label_formatter(&|y| format!("{}%", scale_by(*y, 2).round()))
                .draw()
                .unwrap();

            let user_points = vec![(self.std_dev, self.mean)];

            for hrr in HISTORIC_RISK_RETURN_SAMPLES.iter() {
                chart
                .draw_series(PointSeries::of_element(
                    vec![hrr].iter(),
                    5,
                    &hrr.color,
                    &|c, s, st| {
                        return EmptyElement::at(c.risk_return)
                            + Circle::new((0, 0), s, st.filled())
                            + Text::new(
                                format!(
                                    "{}",
                                    NormalSpec {
                                        mean: c.risk_return.1,
                                        std_dev: c.risk_return.0,
                                    }
                                ),
                                (5, -5),
                                ("sans-serif", 9).into_font(),
                            );
                    },
                ))
                .unwrap();

            }
            // chart
            //     .draw_series(PointSeries::of_element(
            //         HISTORIC_RISK_RETURN_SAMPLES.iter(),
            //         5,
            //         &HISTORIC_RISK_RETURN_SAMPLES.color,
            //         &|c, s, st| {
            //             return EmptyElement::at(c.risk_return)
            //                 + Circle::new((0, 0), s, st.filled())
            //                 /*+ Text::new(
            //                     format!(
            //                         "{}\n{:?}",
            //                         c.label,
            //                         (scale_by(c.risk_return.0, 2), scale_by(c.risk_return.1, 2))
            //                     ),
            //                     (10, -5),
            //                     ("sans-serif", 10).into_font(),
            //                 );*/
            //         },
            //     ))
            //     .unwrap();

            chart
                .draw_series(PointSeries::of_element(
                    user_points.into_iter(),
                    5,
                    &BLUE,
                    &|c, s, st| {
                        return EmptyElement::at(c)
                            + Cross::new((0, 0), s, st.filled())
                            + Text::new(
                                format!("{:?}, {:?}", (c.0 ), (c.1) ),
                                (5, -5),
                                ("sans-serif", 9).into_font(),
                            );
                    },
                ))
                .unwrap();
            root.present().expect("Should present");
        }
        plot_buff

        // ω <fn HistoricRiskReturnPlot::get_historic_plot for NormalSpec>
    }
}

/// Unit tests for `historic_risk_return`
#[cfg(test)]
pub mod unit_tests {
    // α <mod-def unit_tests>
    // ω <mod-def unit_tests>
}

// α <mod-def historic_risk_return>

use once_cell::sync::Lazy;
use plotters::prelude::{GREEN, RED, YELLOW, BLUE, MAGENTA };

pub static HISTORIC_RISK_RETURN_SAMPLES: Lazy<Vec<HistoricRiskReturn>> = Lazy::new(|| {
    vec![
        // Samples pulled from here: https://www.bogleheads.org/wiki/Historical_and_expected_returns
        //x ->risk/ st. dev y -> return/ mean
        HistoricRiskReturn {
            risk_return: (0.202, 0.123),
            label: "US Large Cap".into(),
            color: GREEN
        },
        HistoricRiskReturn {
            risk_return: (0.329, 0.174),
            label: "US Small Cap".into(),
            color: MAGENTA
        },
        HistoricRiskReturn {
            risk_return: (0.057, 0.055),
            label: "US Bonds".into(),
            color: YELLOW
        },
        HistoricRiskReturn {
            risk_return: (0.171, 0.085),
            label: "REITS".into(),
            color: BLUE
        },
        HistoricRiskReturn {
            risk_return: (0.288, 0.1212),
            label: "Emerging Mkts".into(),
            color: RED
        },
    ]
});

// ω <mod-def historic_risk_return>
