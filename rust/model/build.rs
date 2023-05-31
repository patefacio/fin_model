//! Build proto files.

////////////////////////////////////////////////////////////////////////////////////
// --- functions ---
////////////////////////////////////////////////////////////////////////////////////
/// Incorporate protobuf to rust transformation into the build step.
/// 
///   * _return_ - Nothing or any errors
pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    
    tonic_build::configure()
        .build_server(false)
        .out_dir("src/")
        .type_attribute("BondSpec", "#[derive(Serialize, Deserialize)]")
        .type_attribute("Holding", "#[derive(Serialize, Deserialize)]")
        .type_attribute("Account", "#[derive(Serialize, Deserialize)]")
        .type_attribute("RequiredMinimumDistribution", "#[derive(Serialize, Deserialize)]")
        .type_attribute("AccountTreatment", "#[derive(Serialize, Deserialize)]")
        .type_attribute("AgeAssumptions", "#[derive(Serialize, Deserialize)]")
        .type_attribute("BalanceSheet", "#[derive(Serialize, Deserialize)]")
        .type_attribute("Date", "#[derive(Serialize, Deserialize)]")
        .type_attribute("YearValue", "#[derive(Serialize, Deserialize)]")
        .type_attribute("CurrencyValue", "#[derive(Serialize, Deserialize)]")
        .type_attribute("YearCurrencyValue", "#[derive(Serialize, Deserialize)]")
        .type_attribute("YearRange", "#[derive(Serialize, Deserialize)]")
        .type_attribute("NormalSpec", "#[derive(Serialize, Deserialize)]")
        .type_attribute("RateCurve", "#[derive(Serialize, Deserialize)]")
        .type_attribute("YearValueSeries", "#[derive(Serialize, Deserialize)]")
        .type_attribute("DbId", "#[derive(Serialize, Deserialize)]")
        .type_attribute("DossierHoldingIndex", "#[derive(Serialize, Deserialize)]")
        .type_attribute("DossierItemIndex", "#[derive(Serialize, Deserialize)]")
        .type_attribute("DossierCorrelationEntry", "#[derive(Serialize, Deserialize)]")
        .type_attribute("DossierCorrelationMatrix", "#[derive(Serialize, Deserialize)]")
        .type_attribute("ValueOverRange", "#[derive(Serialize, Deserialize)]")
        .type_attribute("PeriodBalance", "#[derive(Serialize, Deserialize)]")
        .type_attribute("GrowthAssumption", "#[derive(Serialize, Deserialize)]")
        .type_attribute("ItemGrowth", "#[derive(Serialize, Deserialize)]")
        .type_attribute("SystemGrowthId", "#[derive(Serialize, Deserialize)]")
        .type_attribute("GrowthItemMappings", "#[derive(Serialize, Deserialize)]")
        .type_attribute("SystemCorrelationEntry", "#[derive(Serialize, Deserialize)]")
        .type_attribute("SystemCorrelationMatrix", "#[derive(Serialize, Deserialize)]")
        .type_attribute("MarketAssumptions", "#[derive(Serialize, Deserialize)]")
        .type_attribute("OutlookMarketAssumptions", "#[derive(Serialize, Deserialize)]")
        .type_attribute("DistributionSpec", "#[derive(Serialize, Deserialize)]")
        .type_attribute("Worth", "#[derive(Serialize, Deserialize)]")
        .type_attribute("Currency", "#[derive(Serialize, Deserialize, EnumVariantNames)]")
        .type_attribute("AccountType", "#[derive(Serialize, Deserialize, EnumVariantNames)]")
        .type_attribute("BasicAllocationType", "#[derive(Serialize, Deserialize, EnumVariantNames)]")
        .type_attribute("DistributionInstrument", "#[derive(Serialize, Deserialize, EnumVariantNames)]")
        .type_attribute("DistributionInstrumentType", "#[derive(Serialize, Deserialize, EnumVariantNames)]")
        .type_attribute("ForecastTaxTreatment", "#[derive(Serialize, Deserialize, EnumVariantNames)]")
        .type_attribute("FlowDirection", "#[derive(Serialize, Deserialize, EnumVariantNames)]")
        .type_attribute("PersonType", "#[derive(Serialize, Deserialize, EnumVariantNames)]")
        .type_attribute("ForecastYearMarkerType", "#[derive(Serialize, Deserialize, EnumVariantNames)]")
        .type_attribute("TaxUsFilingStatus", "#[derive(Serialize, Deserialize, EnumVariantNames)]")
        .type_attribute("TaxUsCategory", "#[derive(Serialize, Deserialize, EnumVariantNames)]")
        .type_attribute("TaxTreatment", "#[derive(Serialize, Deserialize, EnumVariantNames)]")
        .type_attribute("ForecastSortCriteria", "#[derive(Serialize, Deserialize, EnumVariantNames)]")
        .type_attribute("Country", "#[derive(Serialize, Deserialize, EnumVariantNames)]")
        .type_attribute("StateOfResidence", "#[derive(Serialize, Deserialize, EnumVariantNames)]")
        .type_attribute("NamedRateCurve", "#[derive(Serialize, Deserialize, EnumVariantNames)]")
        .type_attribute("WorthType", "#[derive(Serialize, Deserialize, EnumVariantNames)]")
        .type_attribute("HoldingType", "#[derive(Serialize, Deserialize, EnumVariantNames)]")
        .type_attribute("FlowType", "#[derive(Serialize, Deserialize, EnumVariantNames)]")
        .type_attribute("distribution_policy", "#[derive(Serialize, Deserialize)]")
        .type_attribute("withdrawal_treatment", "#[derive(Serialize, Deserialize)]")
        .type_attribute("item_index", "#[derive(Serialize, Deserialize)]")
        .type_attribute("system_id", "#[derive(Serialize, Deserialize)]") 
        .compile(&[ 
          "account.proto",
          "age_assumptions.proto",
          "balance_sheet.proto",
          "core.proto",
          "growth.proto",
          "distributions.proto",
          "core_enums.proto",
          "worth.proto"
        ], &["../../protobuf"])?;
    Ok(())
    
}

// α <mod-def build>
// ω <mod-def build>