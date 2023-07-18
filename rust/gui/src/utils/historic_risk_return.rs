////////////////////////////////////////////////////////////////////////////////////
// --- module uses ---
////////////////////////////////////////////////////////////////////////////////////
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

#[derive(Debug)]
pub struct HistoricRiskReturn {
    /// Historic risk/return value
    pub risk_return: (f64, f64),
    /// The label for the value (e.g. EquityMarket)
    pub label: String,
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
        use plotters::prelude::*;

        //let user_point =

        // let mut x_vec = historic_values
        //     .iter()
        //     .map(|hv| hv.risk_return.0)
        //     .collect::<Vec<_>>();

        // let mut y_vec = historic_values
        //     .iter()
        //     .map(|hv| hv.risk_return.1)
        //     .collect::<Vec<_>>();

        let mut plot_buff = String::with_capacity(2 ^ 11);
        {
            let root = SVGBackend::with_string(&mut plot_buff, (400, 375)).into_drawing_area();
            root.fill(&WHITE);
            let root = root.margin(10, 10, 10, 10);

            let mut chart = ChartBuilder::on(&root)
                .margin(4)
                .caption("Historic Risk Return", ("sans-serif", 40).into_font())
                .x_label_area_size(30)
                .y_label_area_size(60)
                .build_cartesian_2d(0f64..50f64, 0f64..20f64)
                .unwrap();

            chart
                .configure_mesh()
                .x_labels(5)
                .y_labels(5)
                .y_label_formatter(&|x| format!("{:.3}", x))
                .draw()
                .unwrap();

            // chart
            //     .draw_series(LineSeries::new(

            //         HISTORIC_RISK_RETURN_SAMPLES.iter().map(|hv| hv.risk_return),
            //         &RED,
            //     ))
            //     .unwrap();

            chart
                .draw_series(PointSeries::of_element(
                    //HISTORIC_RISK_RETURN_SAMPLES.iter().map(|hv| hv.risk_return)
                    HISTORIC_RISK_RETURN_SAMPLES.iter(),
                    5,
                    &RED,
                    &|c, s, st| {
                        return EmptyElement::at(c.risk_return)
                            + Circle::new((0, 0), s, st.filled())
                            + Text::new(
                                format!("{}\n{:?}", c.label, c.risk_return),
                                (15, -10),
                                ("sans-serif", 10).into_font(),
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

    /// Test trait historic_risk_return_plot on NormalSpec
    pub mod test_historic_risk_return_plot_on_normal_spec {
        ////////////////////////////////////////////////////////////////////////////////////
        // --- module uses ---
        ////////////////////////////////////////////////////////////////////////////////////
        use test_log::test;

        ////////////////////////////////////////////////////////////////////////////////////
        // --- functions ---
        ////////////////////////////////////////////////////////////////////////////////////
        #[test]
        fn get_historic_plot() {
            // α <fn test HistoricRiskReturnPlot::get_historic_plot on NormalSpec>
            todo!("Test get_historic_plot")
            // ω <fn test HistoricRiskReturnPlot::get_historic_plot on NormalSpec>
        }

        // α <mod-def test_historic_risk_return_plot_on_normal_spec>
        // ω <mod-def test_historic_risk_return_plot_on_normal_spec>
    }

    // α <mod-def unit_tests>
    // ω <mod-def unit_tests>
}

// α <mod-def historic_risk_return>

use once_cell::sync::Lazy;

use crate::utils::historic_risk_return;

pub static HISTORIC_RISK_RETURN_SAMPLES: Lazy<Vec<HistoricRiskReturn>> = Lazy::new(|| {
    vec![
        // Samples pulled from here: https://www.visualcapitalist.com/historical-returns-by-asset-class/
        //x ->risk/ st. dev y -> return/ mean
        HistoricRiskReturn {
            risk_return: (16.1, 9.6),
            label: "US Large Cap".into(),
        },
        HistoricRiskReturn {
            risk_return: (5.1, 4.1),
            label: "US Bonds".into(),
        },
        HistoricRiskReturn {
            risk_return: (17.1, 8.5),
            label: "REITS".into(),
        },
        HistoricRiskReturn {
            risk_return: (28.8, 12.12),
            label: "Emerging Mkts".into(),
        },
    ]
});

// ω <mod-def historic_risk_return>
