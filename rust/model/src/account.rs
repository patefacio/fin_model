///
/// Attributes that specify a bond.
#[derive(Serialize, Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BondSpec {
    ///
    /// The face value of the bond.
    #[prost(double, tag = "1")]
    pub face_value: f64,
    ///
    /// Annual bond payment.
    /// Quoted as either dollar value or percent of `face_value` based on
    /// `is_percent_of_face`.
    #[prost(message, optional, tag = "2")]
    pub annual_coupon: ::core::option::Option<super::core::RateCurve>,
    ///
    /// Maturity date.
    #[prost(message, optional, tag = "3")]
    pub maturity: ::core::option::Option<super::core::Date>,
    ///
    /// If true the values of `annual_coupon` represent percentage of face.
    #[prost(bool, tag = "4")]
    pub is_percent_of_face: bool,
}
///
///
/// The holding of a given financial asset held in an account with enough relevant
/// data to model its growth and dividends in a forecast.
///
/// `valuation` is the most recent value of the holding on a `Date`.
#[derive(Serialize, Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Holding {
    ///
    /// An optional comment associated with the holding.
    #[prost(string, tag = "1")]
    pub comment: ::prost::alloc::string::String,
    ///
    ///
    /// Name of the instrument of the holding.
    ///
    /// This name becomes an index into a map of growths associated with instruments.
    /// The details of the instrument growth definitions are held in the `Dossier` and
    /// referenced here.
    #[prost(string, tag = "2")]
    pub instrument_name: ::prost::alloc::string::String,
    ///
    /// Quantity of holding.
    #[prost(double, tag = "3")]
    pub quantity: f64,
    ///
    /// Value of holding holding as of date.
    #[prost(message, optional, tag = "4")]
    pub unit_valuation: ::core::option::Option<super::core::YearCurrencyValue>,
    ///
    /// Cost basis for holding, assumed in currency of `unit_valuation`.
    #[prost(double, tag = "5")]
    pub cost_basis: f64,
    ///
    /// Models the sale date of the holding.
    #[prost(uint64, tag = "8")]
    pub sale_date: u64,
    ///
    /// Specifies any distributions.
    /// Distributions may be specified in one of three ways:
    /// - _DistributionSpec_: Provides breakdown of categorized (e.g.
    /// _qualified dividends_, _unqualified dividends_, ...) distribution
    /// - _BondSpec_: Provides coupon details for a bond.
    #[prost(oneof = "holding::DistributionPolicy", tags = "6, 7")]
    pub distribution_policy: ::core::option::Option<holding::DistributionPolicy>,
}
/// Nested message and enum types in `Holding`.
pub mod holding {
    ///
    /// Specifies any distributions.
    /// Distributions may be specified in one of three ways:
    /// - _DistributionSpec_: Provides breakdown of categorized (e.g.
    /// _qualified dividends_, _unqualified dividends_, ...) distribution
    /// - _BondSpec_: Provides coupon details for a bond.
    #[derive(Serialize, Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum DistributionPolicy {
        ///
        /// What was done with any distributions.
        #[prost(message, tag = "6")]
        DistributionSpec(super::super::distributions::DistributionSpec),
        ///
        /// Bond attributes if holding is a bond
        #[prost(message, tag = "7")]
        BondSpec(super::BondSpec),
    }
}
///
///
/// A store of holdings (ie financial assets) with single treatment in terms of gains.
///
/// The riskiness of holdings in an account can be specified on the holding directly,
/// or for convenience at the account level.
#[derive(Serialize, Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Account {
    ///
    /// Name of the account - *Do Not Use Account Numbers*.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    ///
    /// Type of account, important for determining tax treatment of gains.
    ///
    /// Note: Unlike _Growth Item_ categorization, this is not used to make
    /// _categorical_ growth assumptions.
    #[prost(enumeration = "super::core_enums::AccountType", tag = "2")]
    pub account_type: i32,
    ///
    /// Defines default growth for any holdings in the account without their own assumption.
    ///
    /// This can be used to assign a growth to the account by `growth_item_id`.
    /// For example, assign `us equity` type growth to the account.
    /// If the account has bonds or any holdings with different instruments it is not
    /// a problem since those can be modeled and will be honored.
    #[prost(message, optional, tag = "3")]
    pub default_item_growth: ::core::option::Option<super::growth::ItemGrowth>,
    ///
    /// Year associated with `value`.
    #[prost(message, repeated, tag = "4")]
    pub holdings: ::prost::alloc::vec::Vec<Holding>,
    ///
    /// Value associated with `year`.
    #[prost(double, tag = "5")]
    pub value: f64,
    ///
    /// Index into persons in dossier of the owner.
    #[prost(uint32, tag = "6")]
    pub owner_index: u32,
}
///
/// Details of _required minimum distribution_.
#[derive(Serialize, Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequiredMinimumDistribution {
    ///
    /// Penalty for *not* withdrawing enough.
    #[prost(double, tag = "1")]
    pub penalty: f64,
    ///
    /// Determines minimum distribution.
    #[prost(double, tag = "2")]
    pub distribution_divisor: f64,
}
///
/// Defines how holdings in the account are treated; updated annually
#[derive(Serialize, Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccountTreatment {
    ///
    /// Used to determine sales treatment.
    #[prost(enumeration = "super::core_enums::AccountType", tag = "1")]
    pub account_type: i32,
    ///
    /// `withdrawal_treatment` supports _one_of_:
    /// - EarlyWithdrawalPenalty
    /// - CollegeIrs529Penalty
    /// - HealthSavingsPenalty
    /// - RequiredMinimumDistribution
    #[prost(oneof = "account_treatment::WithdrawalTreatment", tags = "2, 3, 4, 5")]
    pub withdrawal_treatment: ::core::option::Option<
        account_treatment::WithdrawalTreatment,
    >,
}
/// Nested message and enum types in `AccountTreatment`.
pub mod account_treatment {
    ///
    /// `withdrawal_treatment` supports _one_of_:
    /// - EarlyWithdrawalPenalty
    /// - CollegeIrs529Penalty
    /// - HealthSavingsPenalty
    /// - RequiredMinimumDistribution
    #[derive(Serialize, Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum WithdrawalTreatment {
        ///
        /// Penalty for early withdrawal.
        #[prost(double, tag = "2")]
        EarlyWithdrawalPenalty(f64),
        ///
        /// Penalty for not using funds for education.
        #[prost(double, tag = "3")]
        CollegeIrs529Penalty(f64),
        ///
        /// Penalty for not using funds for health.
        #[prost(double, tag = "4")]
        HealthSavingsPenalty(f64),
        ///
        /// Penalty and divisor for _required minimum distribution_.
        #[prost(message, tag = "5")]
        RequiredMinimumDistribution(super::RequiredMinimumDistribution),
    }
}
