///
/// Enumerate the currencies supported.
#[derive(Serialize, Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Currency {
    ///
    /// United States Dollar
    Usd = 0,
    ///
    /// Euro
    Eur = 1,
    ///
    /// Japanese Yen
    Jpy = 2,
    ///
    /// United Kingdom Pound
    Gbp = 3,
    ///
    /// Australian Dollar
    Aud = 4,
    ///
    /// Canadian Dollar
    Cad = 5,
    ///
    /// Swiss Franc
    Chf = 6,
    ///
    /// China Yuan Renminbi
    Cny = 7,
    ///
    /// Hong Kong Dollar
    Hkd = 8,
    ///
    /// New Zealand Dollar
    Nzd = 9,
    ///
    /// Costa Rica Colon
    Crc = 10,
    ///
    /// Russian Ruble
    Rub = 11,
    ///
    /// South Korean Won
    Krw = 12,
    ///
    /// Swedish Krona
    Sek = 13,
}
impl Currency {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Currency::Usd => "USD",
            Currency::Eur => "EUR",
            Currency::Jpy => "JPY",
            Currency::Gbp => "GBP",
            Currency::Aud => "AUD",
            Currency::Cad => "CAD",
            Currency::Chf => "CHF",
            Currency::Cny => "CNY",
            Currency::Hkd => "HKD",
            Currency::Nzd => "NZD",
            Currency::Crc => "CRC",
            Currency::Rub => "RUB",
            Currency::Krw => "KRW",
            Currency::Sek => "SEK",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "USD" => Some(Self::Usd),
            "EUR" => Some(Self::Eur),
            "JPY" => Some(Self::Jpy),
            "GBP" => Some(Self::Gbp),
            "AUD" => Some(Self::Aud),
            "CAD" => Some(Self::Cad),
            "CHF" => Some(Self::Chf),
            "CNY" => Some(Self::Cny),
            "HKD" => Some(Self::Hkd),
            "NZD" => Some(Self::Nzd),
            "CRC" => Some(Self::Crc),
            "RUB" => Some(Self::Rub),
            "KRW" => Some(Self::Krw),
            "SEK" => Some(Self::Sek),
            _ => None,
        }
    }
}
///
/// Used to determine treatment of gains and in some cases (e.g. 529) how funds should be used.
#[derive(Serialize, Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum AccountType {
    ///
    /// An account not covered by other types.
    OtherAccountType = 0,
    ///
    /// A tax sheltered account with funds whose taxes have been paid.
    RothIrs401K = 1,
    ///
    /// A tax sheltered account for funds taxed in retirement.
    TraditionalIrs401K = 2,
    ///
    /// A tax sheltered account for qualified education expenses.
    CollegeIrs529 = 3,
    ///
    /// A tax sheltered account with funds taxed in retirement
    TraditionalIra = 4,
    ///
    /// An account whose investment returns are taxable.
    Taxable = 5,
    ///
    /// An account that allow a relatively quick access to funds, including checking, savings.
    Demand = 6,
    ///
    /// A tax sheltered account for qualified medical expenses.
    HealthSavingsAccount = 7,
}
impl AccountType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            AccountType::OtherAccountType => "OTHER_ACCOUNT_TYPE",
            AccountType::RothIrs401K => "ROTH_IRS_401_K",
            AccountType::TraditionalIrs401K => "TRADITIONAL_IRS_401_K",
            AccountType::CollegeIrs529 => "COLLEGE_IRS_529",
            AccountType::TraditionalIra => "TRADITIONAL_IRA",
            AccountType::Taxable => "TAXABLE",
            AccountType::Demand => "DEMAND",
            AccountType::HealthSavingsAccount => "HEALTH_SAVINGS_ACCOUNT",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "OTHER_ACCOUNT_TYPE" => Some(Self::OtherAccountType),
            "ROTH_IRS_401_K" => Some(Self::RothIrs401K),
            "TRADITIONAL_IRS_401_K" => Some(Self::TraditionalIrs401K),
            "COLLEGE_IRS_529" => Some(Self::CollegeIrs529),
            "TRADITIONAL_IRA" => Some(Self::TraditionalIra),
            "TAXABLE" => Some(Self::Taxable),
            "DEMAND" => Some(Self::Demand),
            "HEALTH_SAVINGS_ACCOUNT" => Some(Self::HealthSavingsAccount),
            _ => None,
        }
    }
}
///
/// To identify holdings as one of the basic allocaiton types.
#[derive(Serialize, Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum BasicAllocationType {
    ///
    /// None of the above.
    OtherBasicAllocationType = 0,
    ///
    /// Instrument is stock like.
    Stock = 1,
    ///
    /// Instrument is bond like.
    Bond = 2,
    ///
    /// Instrument is cash like.
    Cash = 3,
}
impl BasicAllocationType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            BasicAllocationType::OtherBasicAllocationType => {
                "OTHER_BASIC_ALLOCATION_TYPE"
            }
            BasicAllocationType::Stock => "STOCK",
            BasicAllocationType::Bond => "BOND",
            BasicAllocationType::Cash => "CASH",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "OTHER_BASIC_ALLOCATION_TYPE" => Some(Self::OtherBasicAllocationType),
            "STOCK" => Some(Self::Stock),
            "BOND" => Some(Self::Bond),
            "CASH" => Some(Self::Cash),
            _ => None,
        }
    }
}
///
/// Identifies holding as capable of generating distributions.
#[derive(Serialize, Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum DistributionInstrument {
    ///
    /// A single stock that may pay dividends.
    Equity = 0,
    ///
    /// A instrument (e.g. bond) that pays interest.
    InterestBearing = 1,
    ///
    /// An instrument that may *distribute* dividends, interest, and/or capital gains
    MutualFund = 2,
}
impl DistributionInstrument {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            DistributionInstrument::Equity => "EQUITY",
            DistributionInstrument::InterestBearing => "INTEREST_BEARING",
            DistributionInstrument::MutualFund => "MUTUAL_FUND",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "EQUITY" => Some(Self::Equity),
            "INTEREST_BEARING" => Some(Self::InterestBearing),
            "MUTUAL_FUND" => Some(Self::MutualFund),
            _ => None,
        }
    }
}
///
/// Specifies the type of distribution
#[derive(Serialize, Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum DistributionInstrumentType {
    ///
    /// Distribution type not specified
    DistUnknown = 0,
    ///
    /// Distribution for stock - which pays dividends
    DistEquity = 1,
    ///
    /// Distribution type for a bond which may pay interest
    DistBond = 2,
    ///
    /// Distribution type for a mutual fund which may _distribute_
    /// dividends, interest and/or capital gains
    DistMutualFund = 3,
}
impl DistributionInstrumentType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            DistributionInstrumentType::DistUnknown => "DIST_UNKNOWN",
            DistributionInstrumentType::DistEquity => "DIST_EQUITY",
            DistributionInstrumentType::DistBond => "DIST_BOND",
            DistributionInstrumentType::DistMutualFund => "DIST_MUTUAL_FUND",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "DIST_UNKNOWN" => Some(Self::DistUnknown),
            "DIST_EQUITY" => Some(Self::DistEquity),
            "DIST_BOND" => Some(Self::DistBond),
            "DIST_MUTUAL_FUND" => Some(Self::DistMutualFund),
            _ => None,
        }
    }
}
///
///
/// Dictates how the forecaster will treat accounts in terms of taxes.
///
/// The default is `as_modeled` so all taxation is based on account types.
/// If `as_tax_deferred` the forecaster ignores all account type designations
/// and treats all accounts as deferred. Similarly `as_taxable` treats all
/// accounts as taxable and `as_tax_exempt` treats all accounts as tax exempt.
/// The purpose of these is to get a quick view of how taxes are impacting
/// the forecast.
#[derive(Serialize, Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ForecastTaxTreatment {
    ///
    /// Treat taxes as modeled by accounts.
    AsModeled = 0,
    ///
    /// Treat all accounts as if `Taxable`
    AsTaxable = 1,
    ///
    /// Treat all accounts as if `TaxDeferred`.
    AsTaxDeferred = 2,
    ///
    /// Treat all accounts as if `TaxExempt`
    AsTaxExempt = 3,
}
impl ForecastTaxTreatment {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ForecastTaxTreatment::AsModeled => "AS_MODELED",
            ForecastTaxTreatment::AsTaxable => "AS_TAXABLE",
            ForecastTaxTreatment::AsTaxDeferred => "AS_TAX_DEFERRED",
            ForecastTaxTreatment::AsTaxExempt => "AS_TAX_EXEMPT",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "AS_MODELED" => Some(Self::AsModeled),
            "AS_TAXABLE" => Some(Self::AsTaxable),
            "AS_TAX_DEFERRED" => Some(Self::AsTaxDeferred),
            "AS_TAX_EXEMPT" => Some(Self::AsTaxExempt),
            _ => None,
        }
    }
}
///
/// Direction of flow from perspective of balance sheet.
#[derive(Serialize, Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum FlowDirection {
    ///
    /// Indicates flow direction depends on sign.
    UnspecifiedFlow = 0,
    ///
    /// Indicates flows are incoming.
    InFlow = 1,
    ///
    /// Indicates flows are outgoing.
    OutFlow = 2,
    ///
    /// Indicates flows are leaving one place and entering another.
    InternalFlow = 3,
}
impl FlowDirection {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            FlowDirection::UnspecifiedFlow => "UNSPECIFIED_FLOW",
            FlowDirection::InFlow => "IN_FLOW",
            FlowDirection::OutFlow => "OUT_FLOW",
            FlowDirection::InternalFlow => "INTERNAL_FLOW",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "UNSPECIFIED_FLOW" => Some(Self::UnspecifiedFlow),
            "IN_FLOW" => Some(Self::InFlow),
            "OUT_FLOW" => Some(Self::OutFlow),
            "INTERNAL_FLOW" => Some(Self::InternalFlow),
            _ => None,
        }
    }
}
///
/// Used to categorize people covered by `Dossier`.
#[derive(Serialize, Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PersonType {
    ///
    /// Primary owner of the dossier.
    PrimaryOwner = 0,
    ///
    /// Secondary owner of the dossier.
    SecondaryOwner = 1,
    ///
    /// Dependent to primary owner.
    Dependent = 2,
}
impl PersonType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            PersonType::PrimaryOwner => "PRIMARY_OWNER",
            PersonType::SecondaryOwner => "SECONDARY_OWNER",
            PersonType::Dependent => "DEPENDENT",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "PRIMARY_OWNER" => Some(Self::PrimaryOwner),
            "SECONDARY_OWNER" => Some(Self::SecondaryOwner),
            "DEPENDENT" => Some(Self::Dependent),
            _ => None,
        }
    }
}
///
/// Identifies type of year market.
#[derive(Serialize, Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ForecastYearMarkerType {
    ///
    /// Indicates the user has inadequate funds.
    FirstInsolvency = 0,
    ///
    /// Indicates user retirement start.
    RetirementStart = 1,
    ///
    /// Indicates year of death of user.
    Death = 2,
}
impl ForecastYearMarkerType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ForecastYearMarkerType::FirstInsolvency => "FIRST_INSOLVENCY",
            ForecastYearMarkerType::RetirementStart => "RETIREMENT_START",
            ForecastYearMarkerType::Death => "DEATH",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "FIRST_INSOLVENCY" => Some(Self::FirstInsolvency),
            "RETIREMENT_START" => Some(Self::RetirementStart),
            "DEATH" => Some(Self::Death),
            _ => None,
        }
    }
}
///
/// Filing status for taxes.
#[derive(Serialize, Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum TaxUsFilingStatus {
    ///
    /// Married filing jointly.
    MarriedJoint = 0,
    ///
    /// Married filing separately.
    MarriedSeparate = 1,
    ///
    /// Single filing status.
    Single = 2,
    ///
    /// Filing as head of household.
    HeadOfHousehold = 3,
}
impl TaxUsFilingStatus {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            TaxUsFilingStatus::MarriedJoint => "MARRIED_JOINT",
            TaxUsFilingStatus::MarriedSeparate => "MARRIED_SEPARATE",
            TaxUsFilingStatus::Single => "SINGLE",
            TaxUsFilingStatus::HeadOfHousehold => "HEAD_OF_HOUSEHOLD",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "MARRIED_JOINT" => Some(Self::MarriedJoint),
            "MARRIED_SEPARATE" => Some(Self::MarriedSeparate),
            "SINGLE" => Some(Self::Single),
            "HEAD_OF_HOUSEHOLD" => Some(Self::HeadOfHousehold),
            _ => None,
        }
    }
}
///
/// Categorizes taxes according to the schedules that might be used.
#[derive(Serialize, Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum TaxUsCategory {
    ///
    /// Social security taxes.
    UsSocialSecurity = 0,
    ///
    /// Medicare taxes.
    UsMedicare = 1,
    ///
    /// Qualified dividend taxes.
    UsQualifiedDividend = 2,
    ///
    /// Long term capital gain taxes.
    UsLongTermCapitalGain = 3,
    ///
    /// Passive income taxes.
    UsPassiveIncome = 4,
    ///
    /// A type of ordinary income from labor
    UsEarnedIncome = 5,
    ///
    /// Ordinary income taxes.
    UsOrdinaryIncome = 6,
}
impl TaxUsCategory {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            TaxUsCategory::UsSocialSecurity => "US_SOCIAL_SECURITY",
            TaxUsCategory::UsMedicare => "US_MEDICARE",
            TaxUsCategory::UsQualifiedDividend => "US_QUALIFIED_DIVIDEND",
            TaxUsCategory::UsLongTermCapitalGain => "US_LONG_TERM_CAPITAL_GAIN",
            TaxUsCategory::UsPassiveIncome => "US_PASSIVE_INCOME",
            TaxUsCategory::UsEarnedIncome => "US_EARNED_INCOME",
            TaxUsCategory::UsOrdinaryIncome => "US_ORDINARY_INCOME",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "US_SOCIAL_SECURITY" => Some(Self::UsSocialSecurity),
            "US_MEDICARE" => Some(Self::UsMedicare),
            "US_QUALIFIED_DIVIDEND" => Some(Self::UsQualifiedDividend),
            "US_LONG_TERM_CAPITAL_GAIN" => Some(Self::UsLongTermCapitalGain),
            "US_PASSIVE_INCOME" => Some(Self::UsPassiveIncome),
            "US_EARNED_INCOME" => Some(Self::UsEarnedIncome),
            "US_ORDINARY_INCOME" => Some(Self::UsOrdinaryIncome),
            _ => None,
        }
    }
}
///
/// Classification of account with respect to taxation.
#[derive(Serialize, Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum TaxTreatment {
    ///
    /// An account whose income is taxable.
    TaxableAccount = 0,
    ///
    /// An account with income taxed on withdrawal.
    TaxDeferredAccount = 1,
    ///
    /// An account where income is not taxed, usually because it is special purpose like college 529.
    TaxExemptAccount = 2,
}
impl TaxTreatment {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            TaxTreatment::TaxableAccount => "TAXABLE_ACCOUNT",
            TaxTreatment::TaxDeferredAccount => "TAX_DEFERRED_ACCOUNT",
            TaxTreatment::TaxExemptAccount => "TAX_EXEMPT_ACCOUNT",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "TAXABLE_ACCOUNT" => Some(Self::TaxableAccount),
            "TAX_DEFERRED_ACCOUNT" => Some(Self::TaxDeferredAccount),
            "TAX_EXEMPT_ACCOUNT" => Some(Self::TaxExemptAccount),
            _ => None,
        }
    }
}
///
/// Enumerate the sort criteria for forecasts in a simulation.
#[derive(Serialize, Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ForecastSortCriteria {
    ///
    /// Sort forecasts by `final end balance`.
    ByFinalEndBalance = 0,
}
impl ForecastSortCriteria {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ForecastSortCriteria::ByFinalEndBalance => "BY_FINAL_END_BALANCE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "BY_FINAL_END_BALANCE" => Some(Self::ByFinalEndBalance),
            _ => None,
        }
    }
}
///
/// Country supported by the system.
#[derive(Serialize, Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Country {
    ///
    /// United States Of America
    UnitedStates = 0,
    ///
    /// France
    France = 1,
    ///
    /// United Kingdom
    UnitedKingdom = 2,
}
impl Country {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Country::UnitedStates => "UNITED_STATES",
            Country::France => "FRANCE",
            Country::UnitedKingdom => "UNITED_KINGDOM",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "UNITED_STATES" => Some(Self::UnitedStates),
            "FRANCE" => Some(Self::France),
            "UNITED_KINGDOM" => Some(Self::UnitedKingdom),
            _ => None,
        }
    }
}
///
/// State of residence
#[derive(Serialize, Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum StateOfResidence {
    ///
    /// Unspecified state.
    UnspecifiedState = 0,
    ///
    /// State of `AL`
    Al = 1,
    ///
    /// State of `AK`
    Ak = 2,
    ///
    /// State of `AZ`
    Az = 3,
    ///
    /// State of `AR`
    Ar = 4,
    ///
    /// State of `CA`
    Ca = 5,
    ///
    /// State of `CO`
    Co = 6,
    ///
    /// State of `CT`
    Ct = 7,
    ///
    /// State of `DE`
    De = 8,
    ///
    /// State of `FL`
    Fl = 9,
    ///
    /// State of `GA`
    Ga = 10,
    ///
    /// State of `HI`
    Hi = 11,
    ///
    /// State of `ID`
    Id = 12,
    ///
    /// State of `IL`
    Il = 13,
    ///
    /// State of `IN`
    In = 14,
    ///
    /// State of `IA`
    Ia = 15,
    ///
    /// State of `KS`
    Ks = 16,
    ///
    /// State of `KY`
    Ky = 17,
    ///
    /// State of `LA`
    La = 18,
    ///
    /// State of `ME`
    Me = 19,
    ///
    /// State of `MD`
    Md = 20,
    ///
    /// State of `MA`
    Ma = 21,
    ///
    /// State of `MI`
    Mi = 22,
    ///
    /// State of `MN`
    Mn = 23,
    ///
    /// State of `MS`
    Ms = 24,
    ///
    /// State of `MO`
    Mo = 25,
    ///
    /// State of `MT`
    Mt = 26,
    ///
    /// State of `NE`
    Ne = 27,
    ///
    /// State of `NV`
    Nv = 28,
    ///
    /// State of `NH`
    Nh = 29,
    ///
    /// State of `NJ`
    Nj = 30,
    ///
    /// State of `NM`
    Nm = 31,
    ///
    /// State of `NY`
    Ny = 32,
    ///
    /// State of `NC`
    Nc = 33,
    ///
    /// State of `ND`
    Nd = 34,
    ///
    /// State of `OH`
    Oh = 35,
    ///
    /// State of `OK`
    Ok = 36,
    ///
    /// State of `OR`
    Or = 37,
    ///
    /// State of `PA`
    Pa = 38,
    ///
    /// State of `RI`
    Ri = 39,
    ///
    /// State of `SC`
    Sc = 40,
    ///
    /// State of `SD`
    Sd = 41,
    ///
    /// State of `TN`
    Tn = 42,
    ///
    /// State of `TX`
    Tx = 43,
    ///
    /// State of `UT`
    Ut = 44,
    ///
    /// State of `VT`
    Vt = 45,
    ///
    /// State of `VA`
    Va = 46,
    ///
    /// State of `WA`
    Wa = 47,
    ///
    /// State of `WV`
    Wv = 48,
    ///
    /// State of `WI`
    Wi = 49,
    ///
    /// State of `WY`
    Wy = 50,
}
impl StateOfResidence {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            StateOfResidence::UnspecifiedState => "UNSPECIFIED_STATE",
            StateOfResidence::Al => "AL",
            StateOfResidence::Ak => "AK",
            StateOfResidence::Az => "AZ",
            StateOfResidence::Ar => "AR",
            StateOfResidence::Ca => "CA",
            StateOfResidence::Co => "CO",
            StateOfResidence::Ct => "CT",
            StateOfResidence::De => "DE",
            StateOfResidence::Fl => "FL",
            StateOfResidence::Ga => "GA",
            StateOfResidence::Hi => "HI",
            StateOfResidence::Id => "ID",
            StateOfResidence::Il => "IL",
            StateOfResidence::In => "IN",
            StateOfResidence::Ia => "IA",
            StateOfResidence::Ks => "KS",
            StateOfResidence::Ky => "KY",
            StateOfResidence::La => "LA",
            StateOfResidence::Me => "ME",
            StateOfResidence::Md => "MD",
            StateOfResidence::Ma => "MA",
            StateOfResidence::Mi => "MI",
            StateOfResidence::Mn => "MN",
            StateOfResidence::Ms => "MS",
            StateOfResidence::Mo => "MO",
            StateOfResidence::Mt => "MT",
            StateOfResidence::Ne => "NE",
            StateOfResidence::Nv => "NV",
            StateOfResidence::Nh => "NH",
            StateOfResidence::Nj => "NJ",
            StateOfResidence::Nm => "NM",
            StateOfResidence::Ny => "NY",
            StateOfResidence::Nc => "NC",
            StateOfResidence::Nd => "ND",
            StateOfResidence::Oh => "OH",
            StateOfResidence::Ok => "OK",
            StateOfResidence::Or => "OR",
            StateOfResidence::Pa => "PA",
            StateOfResidence::Ri => "RI",
            StateOfResidence::Sc => "SC",
            StateOfResidence::Sd => "SD",
            StateOfResidence::Tn => "TN",
            StateOfResidence::Tx => "TX",
            StateOfResidence::Ut => "UT",
            StateOfResidence::Vt => "VT",
            StateOfResidence::Va => "VA",
            StateOfResidence::Wa => "WA",
            StateOfResidence::Wv => "WV",
            StateOfResidence::Wi => "WI",
            StateOfResidence::Wy => "WY",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "UNSPECIFIED_STATE" => Some(Self::UnspecifiedState),
            "AL" => Some(Self::Al),
            "AK" => Some(Self::Ak),
            "AZ" => Some(Self::Az),
            "AR" => Some(Self::Ar),
            "CA" => Some(Self::Ca),
            "CO" => Some(Self::Co),
            "CT" => Some(Self::Ct),
            "DE" => Some(Self::De),
            "FL" => Some(Self::Fl),
            "GA" => Some(Self::Ga),
            "HI" => Some(Self::Hi),
            "ID" => Some(Self::Id),
            "IL" => Some(Self::Il),
            "IN" => Some(Self::In),
            "IA" => Some(Self::Ia),
            "KS" => Some(Self::Ks),
            "KY" => Some(Self::Ky),
            "LA" => Some(Self::La),
            "ME" => Some(Self::Me),
            "MD" => Some(Self::Md),
            "MA" => Some(Self::Ma),
            "MI" => Some(Self::Mi),
            "MN" => Some(Self::Mn),
            "MS" => Some(Self::Ms),
            "MO" => Some(Self::Mo),
            "MT" => Some(Self::Mt),
            "NE" => Some(Self::Ne),
            "NV" => Some(Self::Nv),
            "NH" => Some(Self::Nh),
            "NJ" => Some(Self::Nj),
            "NM" => Some(Self::Nm),
            "NY" => Some(Self::Ny),
            "NC" => Some(Self::Nc),
            "ND" => Some(Self::Nd),
            "OH" => Some(Self::Oh),
            "OK" => Some(Self::Ok),
            "OR" => Some(Self::Or),
            "PA" => Some(Self::Pa),
            "RI" => Some(Self::Ri),
            "SC" => Some(Self::Sc),
            "SD" => Some(Self::Sd),
            "TN" => Some(Self::Tn),
            "TX" => Some(Self::Tx),
            "UT" => Some(Self::Ut),
            "VT" => Some(Self::Vt),
            "VA" => Some(Self::Va),
            "WA" => Some(Self::Wa),
            "WV" => Some(Self::Wv),
            "WI" => Some(Self::Wi),
            "WY" => Some(Self::Wy),
            _ => None,
        }
    }
}
///
/// A small set of commonly identifiable rate curves
#[derive(Serialize, Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum NamedRateCurve {
    ///
    /// Identifies the curve with no growth.
    NoGrowthCurve = 0,
    ///
    /// Identifies the established inflation rate to be used.
    /// Inflation of specific items is often modeled with a specific type of inflation.
    /// For example, the cost of college is growing faster than general inflation.
    /// So, if all or almost all `flows` grow at their own individual rates there really
    /// is no single _inflation_.
    ///
    /// However, there are times where it is useful to have a single standard inflation to discount values.
    /// When a forecast provides a single final net worth it is in future dollars and for it to be more
    /// digestible it is useful to show in present value terms. Since that net worth is a combination of
    /// lots of expenses that have potentially many different built in inflation numbers - having one to
    /// curve to provide a general discount mechanism is useful.
    ReportInflation = 1,
    ///
    /// A discount curve that can be used to charge borrowing done by a user to stay afloat.
    /// There are times when a forecast takes the person under water and it looks like game over.
    /// However, future inflows might save the day and rather than just immediately throw in the
    /// towel, that under water amount can grow at a _cost of capital_ in hopes of forecast
    /// recovery.
    CostOfCapital = 2,
}
impl NamedRateCurve {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            NamedRateCurve::NoGrowthCurve => "NO_GROWTH_CURVE",
            NamedRateCurve::ReportInflation => "REPORT_INFLATION",
            NamedRateCurve::CostOfCapital => "COST_OF_CAPITAL",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "NO_GROWTH_CURVE" => Some(Self::NoGrowthCurve),
            "REPORT_INFLATION" => Some(Self::ReportInflation),
            "COST_OF_CAPITAL" => Some(Self::CostOfCapital),
            _ => None,
        }
    }
}
///
/// List of plusauri worth types.
#[derive(Serialize, Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum WorthType {
    ///
    /// An _atypical_ worth not, represented by other categories.
    OtherWorth = 0,
    ///
    /// Any form of residential real estate.
    ResidentialRealEstate = 1,
    ///
    /// Any form of commercial real estate.
    CommercialRealEstate = 2,
    ///
    /// Family Farm
    FamilyFarm = 3,
    ///
    /// A car, truck, etc
    Automobile = 4,
    ///
    /// A car that is 20 years or older.
    ClassicCar = 5,
    ///
    /// A car that is 45 years or older.
    AntiqueCar = 6,
    ///
    /// A car built between 1919 and 1930.
    VintageCar = 7,
}
impl WorthType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            WorthType::OtherWorth => "OTHER_WORTH",
            WorthType::ResidentialRealEstate => "RESIDENTIAL_REAL_ESTATE",
            WorthType::CommercialRealEstate => "COMMERCIAL_REAL_ESTATE",
            WorthType::FamilyFarm => "FAMILY_FARM",
            WorthType::Automobile => "AUTOMOBILE",
            WorthType::ClassicCar => "CLASSIC_CAR",
            WorthType::AntiqueCar => "ANTIQUE_CAR",
            WorthType::VintageCar => "VINTAGE_CAR",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "OTHER_WORTH" => Some(Self::OtherWorth),
            "RESIDENTIAL_REAL_ESTATE" => Some(Self::ResidentialRealEstate),
            "COMMERCIAL_REAL_ESTATE" => Some(Self::CommercialRealEstate),
            "FAMILY_FARM" => Some(Self::FamilyFarm),
            "AUTOMOBILE" => Some(Self::Automobile),
            "CLASSIC_CAR" => Some(Self::ClassicCar),
            "ANTIQUE_CAR" => Some(Self::AntiqueCar),
            "VINTAGE_CAR" => Some(Self::VintageCar),
            _ => None,
        }
    }
}
///
/// List of plusauri holding types.
#[derive(Serialize, Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum HoldingType {
    ///
    /// An instrument not represented by other categories.
    OtherInstrument = 0,
    ///
    /// A sample instrument from the US equity market.
    UsEquitySample = 1,
    ///
    /// Well diversified instrument representing US equities, including large, small, mid caps.
    UsEquityMarket = 2,
    ///
    /// US Large cap equity market
    UsLargeEquityMarket = 3,
    ///
    /// Individual equity from us large cap equity market.
    UsLargeEquitySample = 4,
    ///
    /// Sample small cap US equity
    UsSmallEquityMarket = 5,
    ///
    /// Sample from US small cap equities.
    UsSmallEquitySample = 6,
    ///
    /// ETF or similar diversified instrument representing US mid-caps (e.g. `Vanguard Mid-Cap - VO`).
    UsMidEquityMarket = 7,
    ///
    /// Sample from US midcaps.
    UsMidEquitySample = 8,
    ///
    /// ETF or similar diversified instrument representing emerging markets (e.g. iShares MSCI Emerging Markets - `EEM`)
    EmergingEquityMarket = 9,
    ///
    /// A single instrument from the emerging markets category of stocks.
    EmergingEquityMarketSample = 10,
    ///
    /// Gold as an investment, physical or ETF
    Gold = 11,
    ///
    /// Treasury Inflation Protected Securities
    Tips = 12,
    ///
    /// Investments in real estate.
    RealEstate = 13,
    ///
    /// Developed Markets - proxied by `VTMGX`
    /// Vanguard Developed Markets Index Fund seeks to track the performance of the FTSE Developed All Cap ex
    /// US Index, a market-capitalization-weighted index that is made up of approximately 3,700 common stocks
    /// of large-, mid-, and small-cap companies located in Canada and the major markets of Europe and the
    /// Pacific region. The fund attempts to replicate the target index by investing all, or substantially
    /// all, of its assets in the stocks that make up the index, holding each stock in approximately the same
    /// proportion as its weighting in the index. Vanguard’s Equity Index Group uses proprietary software to
    /// implement trading decisions that accommodate cash flows and maintain close correlation with index
    /// characteristics. Vanguard’s refined indexing process, combined with low management fees and efficient
    /// trading, has provided tight tracking net of expenses.
    DevelopedMarkets = 14,
    ///
    /// An instrument in the _developed markets_ domain.
    DevelopedMarketsSample = 15,
    ///
    /// US High Yield Bonds
    UsHighYieldBonds = 16,
    ///
    /// Commodities
    Commodities = 17,
    ///
    /// Long term treasury average maturity of 15–30 years - proxied by `VUSUX`.
    UsLongTermTreasury = 18,
    ///
    /// Intermediate term treasury average maturity of 1-10 years - proxied by `VSIGX`.
    UsIntermediateTermTreasury = 19,
    ///
    /// Short term treasury average maturity of 1-4 years - proxied by `VFIRX`.
    UsShortTermTreasury = 20,
    ///
    /// Short duration corporate bonds - proxied by `VCSH`
    UsShortTermCorporateBond = 21,
    ///
    /// Long term corporate bonds - proxied by `VCLT`
    UsLongTermCorporateBond = 22,
    ///
    /// Money market fund - proxied by `VMFXX`
    UsMoneyMarket = 23,
    ///
    /// International bonds - proxied by `VTABX`.
    InternationalBonds = 24,
    ///
    /// Basic CD - typical cash instrument
    CertificateOfDeposit = 25,
}
impl HoldingType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            HoldingType::OtherInstrument => "OTHER_INSTRUMENT",
            HoldingType::UsEquitySample => "US_EQUITY_SAMPLE",
            HoldingType::UsEquityMarket => "US_EQUITY_MARKET",
            HoldingType::UsLargeEquityMarket => "US_LARGE_EQUITY_MARKET",
            HoldingType::UsLargeEquitySample => "US_LARGE_EQUITY_SAMPLE",
            HoldingType::UsSmallEquityMarket => "US_SMALL_EQUITY_MARKET",
            HoldingType::UsSmallEquitySample => "US_SMALL_EQUITY_SAMPLE",
            HoldingType::UsMidEquityMarket => "US_MID_EQUITY_MARKET",
            HoldingType::UsMidEquitySample => "US_MID_EQUITY_SAMPLE",
            HoldingType::EmergingEquityMarket => "EMERGING_EQUITY_MARKET",
            HoldingType::EmergingEquityMarketSample => "EMERGING_EQUITY_MARKET_SAMPLE",
            HoldingType::Gold => "GOLD",
            HoldingType::Tips => "TIPS",
            HoldingType::RealEstate => "REAL_ESTATE",
            HoldingType::DevelopedMarkets => "DEVELOPED_MARKETS",
            HoldingType::DevelopedMarketsSample => "DEVELOPED_MARKETS_SAMPLE",
            HoldingType::UsHighYieldBonds => "US_HIGH_YIELD_BONDS",
            HoldingType::Commodities => "COMMODITIES",
            HoldingType::UsLongTermTreasury => "US_LONG_TERM_TREASURY",
            HoldingType::UsIntermediateTermTreasury => "US_INTERMEDIATE_TERM_TREASURY",
            HoldingType::UsShortTermTreasury => "US_SHORT_TERM_TREASURY",
            HoldingType::UsShortTermCorporateBond => "US_SHORT_TERM_CORPORATE_BOND",
            HoldingType::UsLongTermCorporateBond => "US_LONG_TERM_CORPORATE_BOND",
            HoldingType::UsMoneyMarket => "US_MONEY_MARKET",
            HoldingType::InternationalBonds => "INTERNATIONAL_BONDS",
            HoldingType::CertificateOfDeposit => "CERTIFICATE_OF_DEPOSIT",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "OTHER_INSTRUMENT" => Some(Self::OtherInstrument),
            "US_EQUITY_SAMPLE" => Some(Self::UsEquitySample),
            "US_EQUITY_MARKET" => Some(Self::UsEquityMarket),
            "US_LARGE_EQUITY_MARKET" => Some(Self::UsLargeEquityMarket),
            "US_LARGE_EQUITY_SAMPLE" => Some(Self::UsLargeEquitySample),
            "US_SMALL_EQUITY_MARKET" => Some(Self::UsSmallEquityMarket),
            "US_SMALL_EQUITY_SAMPLE" => Some(Self::UsSmallEquitySample),
            "US_MID_EQUITY_MARKET" => Some(Self::UsMidEquityMarket),
            "US_MID_EQUITY_SAMPLE" => Some(Self::UsMidEquitySample),
            "EMERGING_EQUITY_MARKET" => Some(Self::EmergingEquityMarket),
            "EMERGING_EQUITY_MARKET_SAMPLE" => Some(Self::EmergingEquityMarketSample),
            "GOLD" => Some(Self::Gold),
            "TIPS" => Some(Self::Tips),
            "REAL_ESTATE" => Some(Self::RealEstate),
            "DEVELOPED_MARKETS" => Some(Self::DevelopedMarkets),
            "DEVELOPED_MARKETS_SAMPLE" => Some(Self::DevelopedMarketsSample),
            "US_HIGH_YIELD_BONDS" => Some(Self::UsHighYieldBonds),
            "COMMODITIES" => Some(Self::Commodities),
            "US_LONG_TERM_TREASURY" => Some(Self::UsLongTermTreasury),
            "US_INTERMEDIATE_TERM_TREASURY" => Some(Self::UsIntermediateTermTreasury),
            "US_SHORT_TERM_TREASURY" => Some(Self::UsShortTermTreasury),
            "US_SHORT_TERM_CORPORATE_BOND" => Some(Self::UsShortTermCorporateBond),
            "US_LONG_TERM_CORPORATE_BOND" => Some(Self::UsLongTermCorporateBond),
            "US_MONEY_MARKET" => Some(Self::UsMoneyMarket),
            "INTERNATIONAL_BONDS" => Some(Self::InternationalBonds),
            "CERTIFICATE_OF_DEPOSIT" => Some(Self::CertificateOfDeposit),
            _ => None,
        }
    }
}
///
/// List of plusauri flow types.
#[derive(Serialize, Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum FlowType {
    ///
    /// An _atypical_ in flow not represented by other categories
    OtherInFlow = 0,
    ///
    /// An _atypical_ out flow not represented by other categories
    OtherOutFlow = 1,
    ///
    /// Money earned from a job - wages, salaries, tips etc
    EarnedIncome = 2,
    ///
    /// Money from pension
    PensionIncome = 3,
    ///
    /// Money from social security
    SocialSecurityIncome = 4,
    ///
    /// Money from rental properties - passive income
    RentalIncome = 5,
    ///
    /// Money from royalties such as music, books, manuscripts, computer software, or a patent
    RoyaltyIncome = 6,
    ///
    /// Money from internet advertising - passive income
    InternetAdvertisingIncome = 7,
    ///
    /// Money from business you are not actively involved in - passive income
    PassiveBusinessIncome = 8,
    ///
    /// Money classified as ordinary income for tax purposes
    OrdinaryIncome = 9,
    ///
    /// General cost of living
    LivingExpense = 10,
    ///
    /// Costs associated with health care
    HealthCareExpense = 11,
    ///
    /// Cost of college which historically has grown faster than inflation
    CollegeExpense = 12,
    ///
    /// Property taxes, which may be deductible.
    /// Property taxes may be deductible if you itemize, but a limit comes into play.
    ///
    /// Under a massive tax overhaul that was signed into law in 2017, deductible state and local income taxes
    /// (SALT), including property taxes, are capped at $10,000.
    ///
    /// The limit is scheduled to last through the 2025 tax year, unless Congress extends it.
    PropertyTaxes = 13,
    ///
    /// Mortgage interest, which may be deductible.
    /// From [_Forbe's_ article](<https://www.forbes.com/advisor/taxes/tax-write-offs-you-can-claim-on-your-taxes/>)
    /// The interest you pay for your mortgage can be deducted from your taxes. The write-off is limited to interest
    /// on up to $750,000 ($375,000 for married-filing-separately taxpayers) of mortgage debt incurred
    /// after Dec. 15, 2017.
    MortgageInterest = 14,
    ///
    /// State taxes paid, which may be deductible.
    /// you can deduct state income taxes that are paid, but the write-off is limited to up to $10,000,
    /// which includes all deductible state and local taxes.
    StateTaxesPaid = 15,
    ///
    /// Charitable donations, which may be deductible.
    /// From [_Forbe's_ article](<https://www.forbes.com/advisor/taxes/tax-write-offs-you-can-claim-on-your-taxes/>)
    /// Generally, you can deduct charitable contributions of cash totaling up to 60% of your adjusted gross income,
    /// or AGI. Donations of items or property also are considered deductible charitable contributions.
    CharitableDonations = 16,
    ///
    /// Medical expenses, which may be deductible.
    /// From [_Forbe's_ article](<https://www.forbes.com/advisor/taxes/tax-write-offs-you-can-claim-on-your-taxes/>)
    /// Medical and dental expenses qualify for a tax deduction, though you can deduct only the costs that exceed
    /// 7.5% of your AGI.
    /// To claim medical-related expenses on your 2022 tax return next year, they must have been paid in 2022, unless
    /// they were charged to a credit card. In those cases, you can deduct the expenses in the year you charged the
    /// card, not necessarily the year in which you repaid them.
    ///
    /// Trips to your doctor’s office or hospital appointments qualify for medical mileage. For 2022, you can deduct
    /// 18 cents a mile for travel you made for medical purposes through June 2022. The amount has increased to 22
    /// cents a mile from July 1, 2022, through the end of the year.
    MedicalExpenses = 17,
    ///
    /// Retirement credits, which may be deductible.
    /// From [_Forbe's_ article](<https://www.forbes.com/advisor/taxes/tax-write-offs-you-can-claim-on-your-taxes/>)
    /// The contributions you make to a retirement plan such as a 401(k) or a traditional or Roth IRA give you a tax
    /// credit of 50%, 20% or 10%, depending on your adjusted gross income that you report on Form 1040. Any rollover
    /// contributions do not qualify for the credit.
    ///
    /// The maximum contribution amount that qualifies for the credit is $2,000 ($4,000 if married filing
    /// jointly), making the maximum possible credit $1,000 ($2,000 if married filing jointly). The IRS has
    /// a chart to help calculate your credit.
    RetirementCredits = 18,
    ///
    /// IRA contributions, which may be deductible.
    /// From [_Forbe's_ article](<https://www.forbes.com/advisor/taxes/tax-write-offs-you-can-claim-on-your-taxes/>)
    /// The maximum contribution for 2022 in a traditional or Roth IRA is $6,000, plus another $1,000 for
    /// people who are 50 years old or more. Your contributions to a traditional IRA are tax-deductible.
    IraContributions = 19,
}
impl FlowType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            FlowType::OtherInFlow => "OTHER_IN_FLOW",
            FlowType::OtherOutFlow => "OTHER_OUT_FLOW",
            FlowType::EarnedIncome => "EARNED_INCOME",
            FlowType::PensionIncome => "PENSION_INCOME",
            FlowType::SocialSecurityIncome => "SOCIAL_SECURITY_INCOME",
            FlowType::RentalIncome => "RENTAL_INCOME",
            FlowType::RoyaltyIncome => "ROYALTY_INCOME",
            FlowType::InternetAdvertisingIncome => "INTERNET_ADVERTISING_INCOME",
            FlowType::PassiveBusinessIncome => "PASSIVE_BUSINESS_INCOME",
            FlowType::OrdinaryIncome => "ORDINARY_INCOME",
            FlowType::LivingExpense => "LIVING_EXPENSE",
            FlowType::HealthCareExpense => "HEALTH_CARE_EXPENSE",
            FlowType::CollegeExpense => "COLLEGE_EXPENSE",
            FlowType::PropertyTaxes => "PROPERTY_TAXES",
            FlowType::MortgageInterest => "MORTGAGE_INTEREST",
            FlowType::StateTaxesPaid => "STATE_TAXES_PAID",
            FlowType::CharitableDonations => "CHARITABLE_DONATIONS",
            FlowType::MedicalExpenses => "MEDICAL_EXPENSES",
            FlowType::RetirementCredits => "RETIREMENT_CREDITS",
            FlowType::IraContributions => "IRA_CONTRIBUTIONS",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "OTHER_IN_FLOW" => Some(Self::OtherInFlow),
            "OTHER_OUT_FLOW" => Some(Self::OtherOutFlow),
            "EARNED_INCOME" => Some(Self::EarnedIncome),
            "PENSION_INCOME" => Some(Self::PensionIncome),
            "SOCIAL_SECURITY_INCOME" => Some(Self::SocialSecurityIncome),
            "RENTAL_INCOME" => Some(Self::RentalIncome),
            "ROYALTY_INCOME" => Some(Self::RoyaltyIncome),
            "INTERNET_ADVERTISING_INCOME" => Some(Self::InternetAdvertisingIncome),
            "PASSIVE_BUSINESS_INCOME" => Some(Self::PassiveBusinessIncome),
            "ORDINARY_INCOME" => Some(Self::OrdinaryIncome),
            "LIVING_EXPENSE" => Some(Self::LivingExpense),
            "HEALTH_CARE_EXPENSE" => Some(Self::HealthCareExpense),
            "COLLEGE_EXPENSE" => Some(Self::CollegeExpense),
            "PROPERTY_TAXES" => Some(Self::PropertyTaxes),
            "MORTGAGE_INTEREST" => Some(Self::MortgageInterest),
            "STATE_TAXES_PAID" => Some(Self::StateTaxesPaid),
            "CHARITABLE_DONATIONS" => Some(Self::CharitableDonations),
            "MEDICAL_EXPENSES" => Some(Self::MedicalExpenses),
            "RETIREMENT_CREDITS" => Some(Self::RetirementCredits),
            "IRA_CONTRIBUTIONS" => Some(Self::IraContributions),
            _ => None,
        }
    }
}
