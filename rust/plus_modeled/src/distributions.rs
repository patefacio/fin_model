///
/// Specifies various distributions of an instrument
#[derive(Serialize, Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DistributionSpec {
    ///
    /// Dividend yield attributed to *qualified* dividends.
    /// Quoted as rate curve. Default implies no qualified dividends.
    #[prost(message, optional, tag = "1")]
    pub qualified_dividend: ::core::option::Option<super::core::RateCurve>,
    ///
    /// Dividend yield attributed to *unqualified* dividends.
    /// Quoted as rate curve. Default implies no unqualified dividends.
    #[prost(message, optional, tag = "2")]
    pub unqualified_dividend: ::core::option::Option<super::core::RateCurve>,
    ///
    /// Capital gain distributions.
    /// Quoted as rate curve. Default implies no capital gain distributions.
    #[prost(message, optional, tag = "3")]
    pub capital_gain: ::core::option::Option<super::core::RateCurve>,
    ///
    /// Interest yield.
    /// Quoted as rate curve. Default implies no interest paid.
    #[prost(message, optional, tag = "4")]
    pub interest: ::core::option::Option<super::core::RateCurve>,
    ///
    /// If true distributions will made available for spending/investment.
    /// By default distributions are reinvested.
    #[prost(bool, tag = "5")]
    pub use_distributions: bool,
}
