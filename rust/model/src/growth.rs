///
///
/// Supplies growth assumptions for various items.
///
/// `GrowthAssumption` is used when forecasting values for items such as:
///
/// - Holdings
/// - Worths
/// - Inflation
/// - Incomes
/// - Expenses
///
/// Provides the characteristics of growth in the form of the distribution and also
/// allows for a specific realization of a growth curve, usually randomly generated
/// from the `NormalSpec`. The forecaster will use the `pinned_growth` curve
/// if provided or generate one from the `normal_spec`.
///
/// The presence of a non-null `pinned_growth` in the context of an M.C. simulation
/// is to freeze the growth of the particular item.
///
#[derive(Serialize, Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GrowthAssumption {
    ///
    /// Defines the characteristics of growth in terms of normal distribution.
    #[prost(message, optional, tag = "1")]
    pub normal_spec: ::core::option::Option<super::core::NormalSpec>,
    ///
    /// A fixed growth curve
    #[prost(message, optional, tag = "2")]
    pub pinned_growth: ::core::option::Option<super::core::RateCurve>,
}
///
/// Characterizes growth of the item. Items include specific concepts
/// with an associated `growth_item_type`(`Worth`, `Holding`, or `GrowingFlowSpec`).
///
/// Examples include:
///
/// - `residential_real_estate` growth item with a `growth_item_type`
/// of `worth`.
///
/// - `us_equity` a sample US security with a `growth_item_type` of
/// `instrument`.
///
/// - `us_equity_market` Well diversified instrument representing US equities,
/// including large, small, mid caps. An example instrument would be `SPY`.
///
/// The idea here is the user can leave the `growth_assumption` blank and the
/// corresponding lookups will resolve to an appropriate growth. Alternatively
/// the user may specify a specific growth. If no growth is specified the
/// process of resolving an assumption will go through a hierarchical search
/// for the best assumption (e.g. first look at the users overrides, then look
/// at the `primary_user` overrides, then the system defaults).
#[derive(Serialize, Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ItemGrowth {
    ///
    /// Identifies the item that is growing.
    #[prost(message, optional, tag = "1")]
    pub system_growth_id: ::core::option::Option<SystemGrowthId>,
    ///
    ///
    /// User specified `GrowthAssumption`.
    ///
    /// A forecast may be configured with `use_item_assumption` set to true,
    /// in which case any growth assumption here will be honored. Otherwise,
    /// the resolving process will provide a suitable growth based on the
    /// identified growth item.
    #[prost(message, optional, tag = "2")]
    pub growth_assumption: ::core::option::Option<GrowthAssumption>,
}
///
/// Uniquely identifies either an instrument or a `growth_item` as known
/// by the system. This sum type allows using instances of this type as
/// keys in associative arrays.
#[derive(Serialize, Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SystemGrowthId {
    ///
    /// `system_id` supports _one_of_:
    /// - InstrumentId
    /// - HoldingItemId
    /// - WorthItemId
    /// - FlowItemId
    #[prost(oneof = "system_growth_id::SystemId", tags = "1, 2, 3, 4")]
    pub system_id: ::core::option::Option<system_growth_id::SystemId>,
}
/// Nested message and enum types in `SystemGrowthId`.
pub mod system_growth_id {
    ///
    /// `system_id` supports _one_of_:
    /// - InstrumentId
    /// - HoldingItemId
    /// - WorthItemId
    /// - FlowItemId
    #[derive(Serialize, Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum SystemId {
        ///
        /// Identifies the instrument that is growing.
        #[prost(uint32, tag = "1")]
        InstrumentId(u32),
        ///
        /// Identifies the holding classification growth item (e.g. `US Equity Market`, `US Large Equity`, ...)
        #[prost(uint32, tag = "2")]
        HoldingItemId(u32),
        ///
        /// Identifies the worth growth item (e.g. `Real Estate`, `Yacht`, ...)
        #[prost(uint32, tag = "3")]
        WorthItemId(u32),
        ///
        /// Identifies the growing flow spec item (e.g. `Labor Income`, `College Expenses`, `Living Expenses`, ...)
        #[prost(uint32, tag = "4")]
        FlowItemId(u32),
    }
}
///
/// Mappings of `growth_item_id` to its `GrowthAssumption`.
#[derive(Serialize, Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GrowthItemMappings {
    ///
    /// Assigns a growth assumption to each `WorthItem` identified by its database id.
    #[prost(map = "uint32, message", tag = "1")]
    pub worth_item_mappings: ::std::collections::HashMap<u32, GrowthAssumption>,
    ///
    /// Assigns a growth assumption to each `HoldingItem` identified by its database id.
    #[prost(map = "uint32, message", tag = "2")]
    pub holding_item_mappings: ::std::collections::HashMap<u32, GrowthAssumption>,
    ///
    /// Assigns a growth assumption to each `FlowItem` identified by its database id.
    #[prost(map = "uint32, message", tag = "3")]
    pub flow_item_mappings: ::std::collections::HashMap<u32, GrowthAssumption>,
    ///
    /// Assigns a growth assumption to each `Instrument` identified by its database id.
    #[prost(map = "uint32, message", tag = "4")]
    pub instrument_mappings: ::std::collections::HashMap<u32, GrowthAssumption>,
    ///
    /// Inflation that should be used in summary reporting of a forecast
    /// that can pull future values to today's terms. Expenses are often
    /// modeled with individual inflation curves because different
    /// expenses may have differing inflation. For example, college
    /// expenses are notoriously more expensive than say overall living
    /// expense inflation. Any forecast may have loads of types of
    /// inflation impacting the final values - but it is still useful to
    /// see final numbers in today's terms.
    #[prost(message, optional, tag = "5")]
    pub report_inflation: ::core::option::Option<super::core::RateCurve>,
    ///
    /// Growth associated with charges on borrowing required to keep a
    /// forecast afloat. Forecasts can run out of money so something
    /// needs to be done about it. One option is to bail early on the
    /// forecast and call it a failure. Another is to carry it through
    /// with lots of negative net worths. Negative net worth is not free
    /// - there is a cost and this is a rate curve for that purpose.
    #[prost(message, optional, tag = "6")]
    pub cost_of_capital: ::core::option::Option<super::core::RateCurve>,
}
///
/// A single entry in a correlation matrix
#[derive(Serialize, Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SystemCorrelationEntry {
    ///
    /// Row index.
    #[prost(message, optional, tag = "1")]
    pub row_index: ::core::option::Option<SystemGrowthId>,
    ///
    /// Column index.
    #[prost(message, optional, tag = "2")]
    pub column_index: ::core::option::Option<SystemGrowthId>,
    ///
    /// Correlation for pair of growth items.
    #[prost(double, tag = "3")]
    pub correlation: f64,
}
///
/// A 2D matrix indexed by growth item ids pointing at the corresponding correlation.
///
/// Note*: This is stored as a list but represents a 2D matrix. So duplicate
/// entries should not exist in the map (and get folded if they do).
#[derive(Serialize, Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SystemCorrelationMatrix {
    ///
    /// A correlation between i,j
    #[prost(message, repeated, tag = "1")]
    pub mappings: ::prost::alloc::vec::Vec<SystemCorrelationEntry>,
}
///
/// Combines the growth assumptions with correlations that characterize the market.
#[derive(Serialize, Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MarketAssumptions {
    ///
    /// How the items grow.
    #[prost(message, optional, tag = "1")]
    pub growth_item_mappings: ::core::option::Option<GrowthItemMappings>,
    ///
    /// How the items are correlated.
    #[prost(message, optional, tag = "2")]
    pub system_correlation_matrix: ::core::option::Option<SystemCorrelationMatrix>,
}
///
/// Mapping of an outlook to corresponding `GrowthMappings`.
/// Note: Here the outlook id only appears in the key since it is not hard to remember id's of
/// 4 or so pre-defined outlooks.
#[derive(Serialize, Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OutlookMarketAssumptions {
    ///
    /// Maps the outlook to the assumptions by growth item type.
    #[prost(map = "uint32, message", tag = "1")]
    pub outlook_mappings: ::std::collections::HashMap<u32, MarketAssumptions>,
}
