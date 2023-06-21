///
///
/// Defines a *fixed* set of cash flows (i.e. a collection of YearValues). *Fixed*
/// here in the sense that they are not subject to random variance - but the values
/// may differ. Since they are modeled with a time series the values can be any set
/// of payments.
#[derive(Serialize, Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ValueFlowSpec {
    ///
    /// Set of cash flows.
    #[prost(message, repeated, tag = "1")]
    pub year_values: ::prost::alloc::vec::Vec<super::core::YearValue>,
    ///
    /// Currency of the flows.
    #[prost(enumeration = "super::core_enums::Currency", tag = "2")]
    pub currency: i32,
    ///
    /// Specifies the type of flow - the link to direction tax implications.
    #[prost(enumeration = "super::core_enums::FlowType", tag = "3")]
    pub flow_type: i32,
    ///
    /// Differentiate inflows vs outflows.
    #[prost(enumeration = "super::core_enums::FlowDirection", tag = "4")]
    pub flow_direction: i32,
    ///
    /// If true every year in `year_range` gets a value.
    /// By default only years specified in `year_values` will get a flow. However, if `is_dense`
    /// is set every year in the forecast (or in the optionally specified `year_range`) will get
    /// a value. In this case the `yield_values` behave like a _piece-wise_ selector.
    ///
    /// Example: [ (2022, 1000.0), (2025, 2000.0) ]
    ///
    /// - Case 1: `is_dense` is false (default) - Two payments will hit the books: `[(2022, 1000.0), (2025, 2000.0)]`
    /// - Case 2: `is_dense` is true - These payments will hit the books: `[(2022, 1000.0), (2023, 1000.0), (2024, 1000.0),
    /// (2025, 2000.0), (2026, 2000.0), ...]`
    #[prost(bool, tag = "5")]
    pub is_dense: bool,
    ///
    ///
    /// Range of years over which the cash flows are defined. This is *optional* and if
    /// not set flows will be applied to all years of the forecast period.
    #[prost(message, optional, tag = "6")]
    pub year_range: ::core::option::Option<super::core::YearRange>,
}
///
///
/// Defines a set of flows that have an `initial_value` and grow with a rate defined
/// by a _Normal Distribution_. This mirrors the approach taken with `Worth`s and
/// `Holding`s.
#[derive(Serialize, Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GrowingFlowSpec {
    ///
    /// Specifies the growth.
    #[prost(message, optional, tag = "1")]
    pub growth: ::core::option::Option<super::growth::ItemGrowth>,
    ///
    ///
    /// Range of years over which the cash flows are defined. **This is *required**.
    #[prost(message, optional, tag = "2")]
    pub year_range: ::core::option::Option<super::core::YearRange>,
    ///
    /// Initial value of cash flows.
    /// *Note** It is best to value at the start of the year. Flows are assumed
    /// paid/received at the end of the year after having applied the growth. So an
    /// initial flow of 100K with 3 percent that starts in _current year_ or
    /// _first year_ will have the rate included when hitting the books at end of year.
    /// Given the rate is included and paid/received at end of year best to value at start of year.
    #[prost(message, optional, tag = "3")]
    pub initial_value: ::core::option::Option<super::core::YearCurrencyValue>,
    ///
    /// By default growing specs grow every year, whether in `year_range` or not.
    /// Set this to true to have growth only occur during `year_range`. An application
    /// might be a set of flows that do not even exist until the `year_range` becomes
    /// current, for example a future business.
    #[prost(bool, tag = "4")]
    pub grow_during_range_only: bool,
}
///
/// A collection of `DossierHoldingIndex` representing funding sources or investment targets.
/// This enables _out flows_ to be linked to funding sources and _in flows_ to target
/// specific investment holdings.
#[derive(Serialize, Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HoldingLinks {
    ///
    /// Optionally assign one or more account/holdings to fund the flows.
    /// Useful for linking college expenses to certain college funds
    /// or medical expense to medical funds. Those holdings are used to pay
    /// the expenses until exhausted and then the remaining expense obligations
    /// are pooled with the others.
    #[prost(message, repeated, tag = "1")]
    pub account_holdings: ::prost::alloc::vec::Vec<super::core::DossierHoldingIndex>,
}
///
///
/// Defines characteristics of a set of evenly spaced cash flows.
///
/// Modeled cash flows arise in the following cases:
///
/// - Incomes: For example, modeling your labor income or a stream of annuity payments
///
/// - Expenses: For example, general expenses or mortgage payments
///
/// Similar to `Worth` and `Holding` items, the `FlowSpec` supports M.C. analysis by supporting
/// `growth_assumption`. Expenses, like living expenses or college expenses and
/// incomes, like your annual labor income, do not stand still; they tend to grow.
/// And just like asset returns there is a random/stochastic nature to that
/// growth.
#[derive(Serialize, Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FlowSpec {
    ///
    /// Identifier for the flow spec - e.g. `College Expense`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    ///
    /// Optionally assign account/holdings to invest the flows.
    #[prost(message, repeated, tag = "4")]
    pub investment_target: ::prost::alloc::vec::Vec<super::core::DossierHoldingIndex>,
    ///
    /// `one_of_flow_spec` supports _one_of_:
    /// - ValueSpec
    /// - GrowingSpec
    #[prost(oneof = "flow_spec::OneOfFlowSpec", tags = "2, 3")]
    pub one_of_flow_spec: ::core::option::Option<flow_spec::OneOfFlowSpec>,
    ///
    /// Holding links are a way to associate flows with specific {account/holdings}.
    /// The \[DossierHoldingIndex\](crate::DossierHoldingIndex) is the tie to the account/holdings.
    /// The `holding_index` in `DossierHoldingIndex` is _optional_ specifically to allow a
    /// single account as a source, rather than requiring the user to specify the actual funds.
    /// A single flow spec is either an _in_flow_ or _out_flow_ and if one of these is set
    /// it will dictate the direction. Specified _funding sources_ implies an _out_flow_
    /// requiring funding. Specified _investment_targets_ implies an _in_flow_ to be invested.
    /// It is possible that a growth category could be specified that contradicts the direction
    /// implied here. The GUI should ensure they are consistent and the server will as well.
    #[prost(oneof = "flow_spec::OneOfHoldingLinks", tags = "5, 6")]
    pub one_of_holding_links: ::core::option::Option<flow_spec::OneOfHoldingLinks>,
}
/// Nested message and enum types in `FlowSpec`.
pub mod flow_spec {
    ///
    /// `one_of_flow_spec` supports _one_of_:
    /// - ValueSpec
    /// - GrowingSpec
    #[derive(Serialize, Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum OneOfFlowSpec {
        ///
        /// Values of the curve
        #[prost(message, tag = "2")]
        ValueSpec(super::ValueFlowSpec),
        ///
        /// The growth spec
        #[prost(message, tag = "3")]
        GrowingSpec(super::GrowingFlowSpec),
    }
    ///
    /// Holding links are a way to associate flows with specific {account/holdings}.
    /// The \[DossierHoldingIndex\](crate::DossierHoldingIndex) is the tie to the account/holdings.
    /// The `holding_index` in `DossierHoldingIndex` is _optional_ specifically to allow a
    /// single account as a source, rather than requiring the user to specify the actual funds.
    /// A single flow spec is either an _in_flow_ or _out_flow_ and if one of these is set
    /// it will dictate the direction. Specified _funding sources_ implies an _out_flow_
    /// requiring funding. Specified _investment_targets_ implies an _in_flow_ to be invested.
    /// It is possible that a growth category could be specified that contradicts the direction
    /// implied here. The GUI should ensure they are consistent and the server will as well.
    #[derive(Serialize, Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum OneOfHoldingLinks {
        ///
        /// Optionally assign one or more account/holdings to fund the flows.
        /// Useful for linking college expenses to certain college funds
        /// or medical expense to medical funds. Those holdings are used to pay
        /// the expenses until exhausted and then the remaining expense obligations
        /// are pooled with the others.
        #[prost(message, tag = "5")]
        FundingSources(super::HoldingLinks),
        ///
        /// Optionally assign account/holdings to invest these flows.
        /// Useful for defining 401K investments to be put in a 401K fund, or funds
        /// targeting a (possibly capped) emergency fund.
        #[prost(message, tag = "6")]
        InvestmentTargets(super::HoldingLinks),
    }
}
