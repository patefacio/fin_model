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
        todo!("Implement `get_historic_plot`")
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

pub static HISTORIC_RISK_RETURN_SAMPLES: Lazy<Vec<HistoricRiskReturn>> = Lazy::new(||  vec! {
    // Samples pulled from here: https://www.visualcapitalist.com/historical-returns-by-asset-class/
    HistoricRiskReturn{
        risk_return: (0.096, 0.161),
        label: "US Large Cap".into()
    },
    HistoricRiskReturn{
        risk_return: (0.041, 0.051),
        label: "US Bonds".into()
    },
    HistoricRiskReturn{
        risk_return: (0.085, 0.171),
        label: "REITS".into()
    },
    HistoricRiskReturn{
        risk_return: (0.1212, 0.288),
        label: "Emerging Markets".into()
    },
});

// ω <mod-def historic_risk_return>
