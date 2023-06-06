///
/// A modeled date
#[derive(Serialize, Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Date {
    ///
    /// Year.
    #[prost(uint32, tag = "1")]
    pub year: u32,
    ///
    /// Month.
    #[prost(uint32, tag = "2")]
    pub month: u32,
    ///
    /// Day.
    #[prost(uint32, tag = "3")]
    pub day: u32,
}
///
/// Pairs a <em>value</em> with a <em>year</em>, useful for storing _annual_ time series data.
#[derive(Serialize, Deserialize, Copy, PartialOrd)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct YearValue {
    ///
    /// Year associated with `value`.
    #[prost(uint32, tag = "1")]
    pub year: u32,
    ///
    /// Value associated with `year`.
    #[prost(double, tag = "2")]
    pub value: f64,
}
///
/// Pairs a <em>currency</em> with a <em>value</em>.
#[derive(Serialize, Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CurrencyValue {
    ///
    /// Currency of the value.
    #[prost(enumeration = "super::core_enums::Currency", tag = "1")]
    pub currency: i32,
    ///
    /// Value in denominated currency.
    #[prost(double, tag = "2")]
    pub value: f64,
}
///
/// A dated value in a specified currency.
#[derive(Serialize, Deserialize, Copy, PartialOrd)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct YearCurrencyValue {
    ///
    /// Year associated with `value`.
    #[prost(uint32, tag = "1")]
    pub year: u32,
    ///
    /// Currency for the value.
    #[prost(enumeration = "super::core_enums::Currency", tag = "2")]
    pub currency: i32,
    ///
    /// Value associated with `year`.
    #[prost(double, tag = "3")]
    pub value: f64,
}
///
/// Models a range of years [start, end).
#[derive(Serialize, Deserialize, Copy, Ord, PartialOrd, Eq)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct YearRange {
    ///
    /// Start year of range.
    #[prost(uint32, tag = "1")]
    pub start: u32,
    ///
    /// End year of range.
    #[prost(uint32, tag = "2")]
    pub end: u32,
}
///
/// Models a _Normal_ distribution.
#[derive(Serialize, Deserialize, Copy, PartialOrd)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NormalSpec {
    ///
    /// Mean of normally distributed random variable.
    #[prost(double, tag = "1")]
    pub mean: f64,
    ///
    /// Standard deviation of normally distributed random variable.
    #[prost(double, tag = "2")]
    pub std_dev: f64,
}
///
/// Defines a schedule of rates.
#[derive(Serialize, Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RateCurve {
    ///
    /// The curve data.
    #[prost(message, repeated, tag = "1")]
    pub curve: ::prost::alloc::vec::Vec<YearValue>,
}
///
/// Defines a series of `YearValue` instances - structurally same as `RateCurve` but different intent.
#[derive(Serialize, Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct YearValueSeries {
    ///
    /// The curve data.
    #[prost(message, repeated, tag = "1")]
    pub curve: ::prost::alloc::vec::Vec<YearValue>,
}
///
/// A database id combined with an interpretable string value for the item.
///
/// Used for providing both a numeric id and something a human can understand.
/// The name is optional. The idea is the client will need to pass identifiers to
/// the server and those are safest when always referring to and relying on
/// numeric values.
#[derive(Serialize, Deserialize, PartialOrd, Ord, Eq)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DbId {
    ///
    /// Identifier for the item.
    #[prost(uint32, tag = "1")]
    pub id: u32,
    ///
    /// Human readable name - *DO NOT TRUST - FOR DEBUG/CONVENIENCE*.
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
}
///
///
/// Identifies an account with a specific holding (optionally) by numeric indices.
///
/// This can be used to tie a specific set of expenses (e.g. a given child's college expenses)
/// to a specific account and/or holding within it. Similarly it can be used to assign non-reinvested
/// incomes to specific investments. An index where the account is specified but no holding is
/// specified implies *any* holding will suffice. See example usage in \[FlowSpec\].
#[derive(Serialize, Deserialize, Copy, Hash, Ord, PartialOrd, Eq)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DossierHoldingIndex {
    ///
    /// Index of the account.
    #[prost(uint32, tag = "1")]
    pub account_index: u32,
    ///
    /// Index of the holding.
    #[prost(uint32, optional, tag = "2")]
    pub holding_index: ::core::option::Option<u32>,
}
///
///
/// Wraps the \[ItemIndex\] in a proto _message_.
///
/// Can be used to identify specific items like `Worth`, `Holding` and `FlowSpec`
/// within a _dossier_.
///
#[derive(Serialize, Deserialize, Copy, Ord, PartialOrd, Eq)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DossierItemIndex {
    ///
    /// Provides an index into an item in the `Dossier` that may have growth characteristics.
    /// The item may be a `Worth`, `Holding` or `FlowSpec`.
    /// This definition is kept in _core_ because, while it is specific to a specific `dossier`
    /// it is simply an index. The dossier itself may store these indexes in some places to
    /// associate items with accounts and/or specific holdings.
    #[prost(oneof = "dossier_item_index::ItemIndex", tags = "1, 2, 3")]
    pub item_index: ::core::option::Option<dossier_item_index::ItemIndex>,
}
/// Nested message and enum types in `DossierItemIndex`.
pub mod dossier_item_index {
    ///
    /// Provides an index into an item in the `Dossier` that may have growth characteristics.
    /// The item may be a `Worth`, `Holding` or `FlowSpec`.
    /// This definition is kept in _core_ because, while it is specific to a specific `dossier`
    /// it is simply an index. The dossier itself may store these indexes in some places to
    /// associate items with accounts and/or specific holdings.
    #[derive(Serialize, Deserialize, Copy, Hash, Ord, PartialOrd, Eq)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum ItemIndex {
        ///
        /// Index into the `worths` of a `dossier`.
        #[prost(uint32, tag = "1")]
        WorthIndex(u32),
        ///
        /// Index into `accounts`/`holdings` of a `dossier`.
        #[prost(message, tag = "2")]
        HoldingIndex(super::DossierHoldingIndex),
        ///
        /// Index into the `FlowSpecs` of a `dossier`.
        #[prost(uint32, tag = "3")]
        FlowIndex(u32),
    }
}
///
/// An index into a 2D matrix of correlations
#[derive(Serialize, Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DossierCorrelationEntry {
    ///
    /// Row index of correlation
    #[prost(message, optional, tag = "1")]
    pub row_index: ::core::option::Option<DossierItemIndex>,
    ///
    /// Column index of correlation
    #[prost(message, optional, tag = "2")]
    pub column_index: ::core::option::Option<DossierItemIndex>,
    ///
    /// Correlation for pair of dossier items
    #[prost(double, tag = "3")]
    pub correlation: f64,
}
///
/// A 2D matrix indexed by a pair of \[ItemIndex\] pointing at the corresponding correlation.
#[derive(Serialize, Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DossierCorrelationMatrix {
    ///
    /// A correlation between i,j.
    #[prost(message, repeated, tag = "1")]
    pub mappings: ::prost::alloc::vec::Vec<DossierCorrelationEntry>,
}
///
///
/// A conceptually single value presented twice, first as a present value and
/// second as a future value. Both present and future values are `YearValue`
/// objects. A `ValueOverRange` where the present and future values are the
/// same may indicate a present valuing of a future value where the interest
/// rate of the PV is a 0 rate curve. Alternatively, it might mean the actual
/// growth in the value exactly offsets the effects of inflation.
#[derive(Serialize, Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ValueOverRange {
    ///
    /// The present (e.g.start) year value.
    #[prost(message, optional, tag = "1")]
    pub present_year_value: ::core::option::Option<YearValue>,
    ///
    /// The future (e.g. end) year value.
    #[prost(message, optional, tag = "2")]
    pub future_year_value: ::core::option::Option<YearValue>,
}
///
/// Balance change over a period
#[derive(Serialize, Deserialize, Copy, PartialOrd)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PeriodBalance {
    ///
    /// Balance at start of period.
    #[prost(double, tag = "1")]
    pub start_balance: f64,
    ///
    /// Balance at end of period.
    #[prost(double, tag = "2")]
    pub end_balance: f64,
}
