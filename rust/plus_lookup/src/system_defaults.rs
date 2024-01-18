//! Contains defaults for values such as display inflation, cost of capital, etc

////////////////////////////////////////////////////////////////////////////////////
// --- module uses ---
////////////////////////////////////////////////////////////////////////////////////
use once_cell::sync::Lazy;
use plus_modeled::RateCurve;

////////////////////////////////////////////////////////////////////////////////////
// --- lazy inits ---
////////////////////////////////////////////////////////////////////////////////////
/// Basic system defaults
pub static SYSTEM_DEFAULTS: Lazy<SystemDefaults> = Lazy::new(|| {
    {
        // α <lazy init for system_defaults>
        todo!("Write system_defaults initializer")
        // ω <lazy init for system_defaults>
    }
});

////////////////////////////////////////////////////////////////////////////////////
// --- structs ---
////////////////////////////////////////////////////////////////////////////////////
/// Defaults for various constant-like data
#[derive(Debug, Clone)]
pub struct SystemDefaults {
    /// Inflation used for display purposes
    pub display_inflation: RateCurve,
    /// Inflation sometimes used as a default when user does not supply
    pub generic_inflation: RateCurve,
    /// Cost of capital used to charge deficits in a forecast
    pub cost_of_capital: RateCurve,
    /// Default number of runs in a MC forecast
    pub forecast_count: usize,
}

// α <mod-def system_defaults>
// ω <mod-def system_defaults>
