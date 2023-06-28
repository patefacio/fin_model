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
        todo!("Implement `plot`")
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
