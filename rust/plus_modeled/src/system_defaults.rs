///
///
/// Default values for the system which are required for running forecasts.
/// This is stored as an object in the database but is captured as an object
/// so tests are not tied to the database.
#[derive(Serialize, Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SystemDefaults {
    ///
    /// Default user - used in places where user is optional, like querying growth assumptions.
    #[prost(string, tag = "1")]
    pub default_user: ::prost::alloc::string::String,
    ///
    /// Default growth outlook - used when not specified by user.
    #[prost(uint32, tag = "2")]
    pub default_growth_outlook: u32,
    ///
    /// Value for inflation, used to PV future dollars to make sense of them.
    #[prost(message, optional, tag = "3")]
    pub inflation: ::core::option::Option<super::core::RateCurve>,
    ///
    /// Rate to charge any `remaining_shortfall` values.
    #[prost(message, optional, tag = "4")]
    pub cost_of_capital: ::core::option::Option<super::core::RateCurve>,
    ///
    /// System growth numbers for various item types.
    #[prost(message, optional, tag = "5")]
    pub growth_item_mappings: ::core::option::Option<super::growth::GrowthItemMappings>,
    ///
    /// Standard number of forecasts for M.C.
    #[prost(uint32, tag = "6")]
    pub forecast_count: u32,
    ///
    /// Maximum losses that can be used to reduce offset ordinary income.
    #[prost(double, tag = "7")]
    pub max_losses_offsetting_ordinary: f64,
    ///
    /// Maps the DbId numeric value of flow types (e.g. `CollegeExpense`) to account type (e.g. `CollegeIrs529)
    #[prost(map = "uint32, enumeration(super::core_enums::AccountType)", tag = "8")]
    pub auto_flow_account_links: ::std::collections::HashMap<u32, i32>,
    ///
    /// Maps the DbId numeric value of account type to corresponding contribution limits.
    #[prost(map = "uint32, message", tag = "9")]
    pub retirement_contribution_limits: ::std::collections::HashMap<
        u32,
        super::retirement::RetirementContributionLimit,
    >,
}
