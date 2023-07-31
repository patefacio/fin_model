///
/// Assumptions regarding the ages of a person _at retirement_, _at death_, etc.
#[derive(Serialize, Deserialize, Copy, Hash, Ord, PartialOrd, Eq)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AgeAssumptions {
    ///
    /// Age of retirement, generally when labor incomes are ended.
    #[prost(uint32, tag = "1")]
    pub retirement_age: u32,
    ///
    /// Age at death.
    #[prost(uint32, tag = "2")]
    pub death_age: u32,
}
