///
/// Models investment in retirement accounts with contributions limited by system rules.
#[derive(Serialize, Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RetirementContribution {
    ///
    /// Index to the `holding` the contribution will go.
    #[prost(uint32, tag = "1")]
    pub account_index: u32,
    ///
    /// Index to the `FlowSpec` containing the earned income.
    #[prost(uint32, tag = "2")]
    pub earned_income_index: u32,
    ///
    /// Annual goal for investing contributions - if 0 maximum is the goal.
    #[prost(double, optional, tag = "3")]
    pub annual_investment_goal: ::core::option::Option<f64>,
    ///
    /// Growth rate for the annual investment goal, if `None` defaulted to inflation.
    #[prost(message, optional, tag = "4")]
    pub goal_growth_rate: ::core::option::Option<super::core::RateCurve>,
    ///
    /// Year range for the contributions.
    #[prost(message, optional, tag = "5")]
    pub year_range: ::core::option::Option<super::core::YearRange>,
    ///
    /// Employer match for the contributions.
    #[prost(message, optional, tag = "6")]
    pub employer_match_program: ::core::option::Option<EmployerMatchProgram>,
}
///
/// Models typical employer match programs.
#[derive(Serialize, Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EmployerMatchProgram {
    ///
    /// Percent of the employee contribution that is matched by the company.
    #[prost(double, tag = "1")]
    pub matching_percent: f64,
    ///
    /// Cap on total percent of salary that an employer will match.
    #[prost(double, tag = "2")]
    pub matching_percent_cap: f64,
}
///
/// Models annual limits on contributions, grown annually at inflation
#[derive(Serialize, Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RetirementContributionLimit {
    ///
    /// Contribution limit - dated so can be scaled in time.
    #[prost(message, optional, tag = "1")]
    pub limit: ::core::option::Option<super::core::YearValue>,
    ///
    /// Age, on or beyond which `catch_up_limit` _additional_ amount is allowed.
    #[prost(uint32, tag = "2")]
    pub catch_up_start_age: u32,
    ///
    /// Additional contribution amount allowed if older than `catch_up_start_age`.
    #[prost(message, optional, tag = "3")]
    pub catch_up_limit: ::core::option::Option<super::core::YearValue>,
}
