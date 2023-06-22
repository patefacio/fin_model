///
///
/// Captures just enough about the `Person` to forecast.
/// Ability to determine age is important for determining timing of
/// events - like RMD requirements, statistics on costs such as healthcare,
/// etc.
#[derive(Serialize, Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Person {
    ///
    /// Identifier for the person (e.g. name or _nom de guerre_)
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    ///
    /// Type of person.
    #[prost(enumeration = "super::core_enums::PersonType", tag = "2")]
    pub person_type: i32,
    ///
    /// Year of birth.
    #[prost(uint32, tag = "3")]
    pub birth_year: u32,
    ///
    /// Important age data for the person.
    #[prost(message, optional, tag = "4")]
    pub age_assumptions: ::core::option::Option<super::age_assumptions::AgeAssumptions>,
}
