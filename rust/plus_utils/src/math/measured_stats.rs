////////////////////////////////////////////////////////////////////////////////////
// --- module uses ---
////////////////////////////////////////////////////////////////////////////////////
use ::core::fmt::Display;
use ::core::fmt::Formatter;

////////////////////////////////////////////////////////////////////////////////////
// --- structs ---
////////////////////////////////////////////////////////////////////////////////////
/// Bundled statistics measures (count, min, max, mean, std_dev).
#[derive(Debug, Default, Copy, Clone, Serialize)]
pub struct MeasuredStats {
    /// Count of data points included.
    pub count: usize,
    /// Minimum encountered.
    pub min: Option<f64>,
    /// Maximum encountered.
    pub max: Option<f64>,
    /// Arithmetic mean
    pub mean: Option<f64>,
    /// Median value
    pub median: Option<f64>,
    /// Standard deviation.
    pub std_dev: Option<f64>,
}

////////////////////////////////////////////////////////////////////////////////////
// --- trait impls ---
////////////////////////////////////////////////////////////////////////////////////
impl Display for MeasuredStats {
    /// Format the instance.
    ///
    ///   * **f** - Formatter to push formatted item to.
    ///   * _return_ - Formatted instance
    fn fmt(&self, #[allow(unused)] f: &mut Formatter<'_>) -> ::core::fmt::Result {
        // α <fn Display::fmt for MeasuredStats>

        let prec = 5;
        let format_item = |value: &Option<f64>| -> String {
            value
                .map(|value| format!("{:.prec$}", value))
                .unwrap_or_else(|| String::from("_"))
        };

        write!(
            f,
            "(N={}, min={}, max={},{} mean={}, SD={})",
            self.count,
            format_item(&self.min),
            format_item(&self.max),
            if let Some(median) = self.median {
                format!(" med={:.prec$}", median)
            } else {
                String::default()
            },
            format_item(&self.mean),
            format_item(&self.std_dev)
        )

        // ω <fn Display::fmt for MeasuredStats>
    }
}

// α <mod-def measured_stats>
// ω <mod-def measured_stats>
