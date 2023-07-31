///
/// Models the assets and liabilities (ie the balance sheet) of a dossier.
#[derive(Serialize, Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BalanceSheet {
    ///
    /// Assets and liabilities for the balance sheet.
    #[prost(message, repeated, tag = "1")]
    pub worths: ::prost::alloc::vec::Vec<super::worth::Worth>,
    ///
    /// Accounts for the balance sheet.
    #[prost(message, repeated, tag = "2")]
    pub accounts: ::prost::alloc::vec::Vec<super::account::Account>,
    ///
    /// Associates an instrument name (as defined by the user) with an `ItemGrowth`.
    #[prost(map = "string, message", tag = "3")]
    pub instrument_growth_mappings: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        super::growth::ItemGrowth,
    >,
}
