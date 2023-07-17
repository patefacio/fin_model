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

        let mut x_vec = historic_values
            .iter()
            .map(|hv| hv.risk_return.0)
            .collect::<Vec<_>>();
        let mut y_vec = historic_values
            .iter()
            .map(|hv| hv.risk_return.1)
            .collect::<Vec<_>>();
        String::default()
        // for each in HISTORIC_RISK_RETURN_SAMPLES{
        //     x_vec.push(risk_return.0);
        //     y_vec.push(risk_return.1);
        // }

        /*let mut vec_slice: Vec<(f64, f64)> = Vec::with_capacity(HISTORIC_RISK_RETURN_SAMPLES.len());
        let min_y = f64::MAX;
        let max_y = f64::MIN;

        for each in HistoricRiskReturn::risk_return.as_slice(){}

        let min_x = HISTORIC_RISK_RETURN_SAMPLES::HistoricRiskReturn::risk_return.0;
        let max_x = HistoricRiskReturn::risk_return.1;


        //This is all code for the writing and styling of the graph from the plotters library
        let mut plot_buff = String::with_capacity(2 ^ 11);
        {
            let text_style: TextStyle = ("sans-serif", 20).into();
            let root = SVGBackend::with_string(&mut plot_buff, (300, 275))
                .into_drawing_area()
                .titled("Historic Risk & Return", text_style.clone())
                .expect("");

            let mut chart = ChartBuilder::on(&root)
                .margin(4)
                .set_label_area_size(LabelAreaPosition::Left, 40)
                .set_label_area_size(LabelAreaPosition::Bottom, 40)
                .set_label_area_size(LabelAreaPosition::Right, 0)
                .caption("Historic Risk & Return", ("sans-serif", 30))
                .build_cartesian_2d((0.041..0.122/*min_x as f64..max_x as f64*/), (0.051..0.288/*min_y..max_y*/))
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

            chart
                .draw_series(LineSeries::new(vec_slice, &BLUE).point_size(2))
                .unwrap()
                .label("PDF");

            root.present().expect("Should show");
        }

        plot_buff*/

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

pub static HISTORIC_RISK_RETURN_SAMPLES: Lazy<Vec<HistoricRiskReturn>> = Lazy::new(|| {
    vec![
        // Samples pulled from here: https://www.visualcapitalist.com/historical-returns-by-asset-class/
        //x ->risk/ mean y -> return/ st. dev.
        HistoricRiskReturn {
            risk_return: (0.096, 0.161),
            label: "US Large Cap".into(),
        },
        HistoricRiskReturn {
            risk_return: (0.041, 0.051),
            label: "US Bonds".into(),
        },
        HistoricRiskReturn {
            risk_return: (0.085, 0.171),
            label: "REITS".into(),
        },
        HistoricRiskReturn {
            risk_return: (0.1212, 0.288),
            label: "Emerging Markets".into(),
        },
    ]
});

// ω <mod-def historic_risk_return>
