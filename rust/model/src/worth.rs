///
/// Represents some positive _asset_ or negative _liability_ worth that may grow (or
/// shrink) in forecast.
///
/// This is a `BalanceSheet` member and it is almost always an _asset_. We keep the
/// more general term `Worth`, though, because one could model _liabilities_ as
/// negative* worths.
#[derive(Serialize, Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Worth {
    ///
    /// Identifier of the worth.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    ///
    /// Defines growth of the `Worth`.
    #[prost(message, optional, tag = "2")]
    pub growth: ::core::option::Option<super::growth::ItemGrowth>,
    ///
    /// Dated value worth in currency.
    /// *Note** It is best to value at the start of the year. Most payments and markings
    /// are done at the end of the year after having applied the growth.
    #[prost(message, optional, tag = "3")]
    pub valuation: ::core::option::Option<super::core::YearCurrencyValue>,
    ///
    /// Cost basis of worth. Modeled separate from valuation because when purchased it might have
    /// been valued quite differently than any recent *valuation*.
    #[prost(double, tag = "4")]
    pub cost_basis: f64,
}
