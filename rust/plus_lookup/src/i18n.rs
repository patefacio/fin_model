//! All the i18n text

////////////////////////////////////////////////////////////////////////////////////
// --- module uses ---
////////////////////////////////////////////////////////////////////////////////////
use crate::lang_selector_to_language_id;
use crate::LangSelector;
use crate::LOCALES;
use fluent_templates::Loader;

/// Functions for i18n strings in AccountComponent
pub mod account_component {
    ////////////////////////////////////////////////////////////////////////////////////
    // --- module uses ---
    ////////////////////////////////////////////////////////////////////////////////////
    use super::*;

    ////////////////////////////////////////////////////////////////////////////////////
    // --- functions ---
    ////////////////////////////////////////////////////////////////////////////////////
    /// I18n for account
    ///
    ///   * **lang_selector** - Language selector
    ///   * _return_ - The string for language
    pub fn i18n_account(lang_selector: LangSelector) -> String {
        LOCALES
            .lookup(
                lang_selector_to_language_id(&lang_selector),
                "account_component.account",
            )
            .unwrap_or_default()
    }

    /// I18n for name
    ///
    ///   * **lang_selector** - Language selector
    ///   * _return_ - The string for language
    pub fn i18n_name(lang_selector: LangSelector) -> String {
        LOCALES
            .lookup(
                lang_selector_to_language_id(&lang_selector),
                "account_component.name",
            )
            .unwrap_or_default()
    }

    /// I18n for account_type
    ///
    ///   * **lang_selector** - Language selector
    ///   * _return_ - The string for language
    pub fn i18n_account_type(lang_selector: LangSelector) -> String {
        LOCALES
            .lookup(
                lang_selector_to_language_id(&lang_selector),
                "account_component.account_type",
            )
            .unwrap_or_default()
    }

    // α <mod-def account_component>
    // ω <mod-def account_component>
}

/// Functions for i18n strings in AccountsGrid
pub mod accounts_grid {
    ////////////////////////////////////////////////////////////////////////////////////
    // --- module uses ---
    ////////////////////////////////////////////////////////////////////////////////////
    use super::*;

    ////////////////////////////////////////////////////////////////////////////////////
    // --- functions ---
    ////////////////////////////////////////////////////////////////////////////////////
    /// I18n for name
    ///
    ///   * **lang_selector** - Language selector
    ///   * _return_ - The string for language
    pub fn i18n_name(lang_selector: LangSelector) -> String {
        LOCALES
            .lookup(
                lang_selector_to_language_id(&lang_selector),
                "accounts_grid.name",
            )
            .unwrap_or_default()
    }

    /// I18n for accounts
    ///
    ///   * **lang_selector** - Language selector
    ///   * _return_ - The string for language
    pub fn i18n_accounts(lang_selector: LangSelector) -> String {
        LOCALES
            .lookup(
                lang_selector_to_language_id(&lang_selector),
                "accounts_grid.accounts",
            )
            .unwrap_or_default()
    }

    /// I18n for mv
    ///
    ///   * **lang_selector** - Language selector
    ///   * _return_ - The string for language
    pub fn i18n_mv(lang_selector: LangSelector) -> String {
        LOCALES
            .lookup(
                lang_selector_to_language_id(&lang_selector),
                "accounts_grid.mv",
            )
            .unwrap_or_default()
    }

    /// I18n for type
    ///
    ///   * **lang_selector** - Language selector
    ///   * _return_ - The string for language
    pub fn i18n_grid_type(lang_selector: LangSelector) -> String {
        LOCALES
            .lookup(
                lang_selector_to_language_id(&lang_selector),
                "accounts_grid.type",
            )
            .unwrap_or_default()
    }

    /// I18n for new_account
    ///
    ///   * **lang_selector** - Language selector
    ///   * _return_ - The string for language
    pub fn i18n_new_account(lang_selector: LangSelector) -> String {
        LOCALES
            .lookup(
                lang_selector_to_language_id(&lang_selector),
                "accounts_grid.new_account",
            )
            .unwrap_or_default()
    }

    /// I18n for grid_help
    ///
    ///   * **lang_selector** - Language selector
    ///   * _return_ - The string for language
    pub fn i18n_grid_help(lang_selector: LangSelector) -> String {
        LOCALES
            .lookup(
                lang_selector_to_language_id(&lang_selector),
                "accounts_grid.grid_help",
            )
            .unwrap_or_default()
    }

    // α <mod-def accounts_grid>
    // ω <mod-def accounts_grid>
}

/// Functions for i18n strings in AgeAssumptionsComponent
pub mod age_assumptions_component {
    ////////////////////////////////////////////////////////////////////////////////////
    // --- module uses ---
    ////////////////////////////////////////////////////////////////////////////////////
    use super::*;

    ////////////////////////////////////////////////////////////////////////////////////
    // --- functions ---
    ////////////////////////////////////////////////////////////////////////////////////
    /// I18n for retirement_age
    ///
    ///   * **lang_selector** - Language selector
    ///   * _return_ - The string for language
    pub fn i18n_retirement_age(lang_selector: LangSelector) -> String {
        LOCALES
            .lookup(
                lang_selector_to_language_id(&lang_selector),
                "age_assumptions_component.retirement_age",
            )
            .unwrap_or_default()
    }

    /// I18n for death_age
    ///
    ///   * **lang_selector** - Language selector
    ///   * _return_ - The string for language
    pub fn i18n_death_age(lang_selector: LangSelector) -> String {
        LOCALES
            .lookup(
                lang_selector_to_language_id(&lang_selector),
                "age_assumptions_component.death_age",
            )
            .unwrap_or_default()
    }

    // α <mod-def age_assumptions_component>
    // ω <mod-def age_assumptions_component>
}

/// Functions for i18n strings in AssetLiabilityTimeline
pub mod asset_liability_timeline {
    ////////////////////////////////////////////////////////////////////////////////////
    // --- module uses ---
    ////////////////////////////////////////////////////////////////////////////////////
    use super::*;

    ////////////////////////////////////////////////////////////////////////////////////
    // --- functions ---
    ////////////////////////////////////////////////////////////////////////////////////
    /// I18n for asset_liability_timeline
    ///
    ///   * **lang_selector** - Language selector
    ///   * _return_ - The string for language
    pub fn i18n_asset_liability_timeline(lang_selector: LangSelector) -> String {
        LOCALES
            .lookup(
                lang_selector_to_language_id(&lang_selector),
                "asset_liability_timeline.asset_liability_timeline",
            )
            .unwrap_or_default()
    }

    // α <mod-def asset_liability_timeline>
    // ω <mod-def asset_liability_timeline>
}

/// Functions for i18n strings in BalanceSheetStatement
pub mod balance_sheet_statement {
    ////////////////////////////////////////////////////////////////////////////////////
    // --- module uses ---
    ////////////////////////////////////////////////////////////////////////////////////
    use super::*;

    ////////////////////////////////////////////////////////////////////////////////////
    // --- functions ---
    ////////////////////////////////////////////////////////////////////////////////////
    /// I18n for balance_sheet
    ///
    ///   * **lang_selector** - Language selector
    ///   * _return_ - The string for language
    pub fn i18n_balance_sheet(lang_selector: LangSelector) -> String {
        LOCALES
            .lookup(
                lang_selector_to_language_id(&lang_selector),
                "balance_sheet_statement.balance_sheet",
            )
            .unwrap_or_default()
    }

    /// I18n for balance_sheet_statement
    ///
    ///   * **lang_selector** - Language selector
    ///   * _return_ - The string for language
    pub fn i18n_balance_sheet_statement(lang_selector: LangSelector) -> String {
        LOCALES
            .lookup(
                lang_selector_to_language_id(&lang_selector),
                "balance_sheet_statement.balance_sheet_statement",
            )
            .unwrap_or_default()
    }

    /// I18n for start_value
    ///
    ///   * **lang_selector** - Language selector
    ///   * _return_ - The string for language
    pub fn i18n_start_value(lang_selector: LangSelector) -> String {
        LOCALES
            .lookup(
                lang_selector_to_language_id(&lang_selector),
                "balance_sheet_statement.start_value",
            )
            .unwrap_or_default()
    }

    /// I18n for end_value
    ///
    ///   * **lang_selector** - Language selector
    ///   * _return_ - The string for language
    pub fn i18n_end_value(lang_selector: LangSelector) -> String {
        LOCALES
            .lookup(
                lang_selector_to_language_id(&lang_selector),
                "balance_sheet_statement.end_value",
            )
            .unwrap_or_default()
    }

    /// I18n for appreciation_abbrev
    ///
    ///   * **lang_selector** - Language selector
    ///   * _return_ - The string for language
    pub fn i18n_appreciation_abbrev(lang_selector: LangSelector) -> String {
        LOCALES
            .lookup(
                lang_selector_to_language_id(&lang_selector),
                "balance_sheet_statement.appreciation_abbrev",
            )
            .unwrap_or_default()
    }

    /// I18n for appreciation_abbrev_pct
    ///
    ///   * **lang_selector** - Language selector
    ///   * _return_ - The string for language
    pub fn i18n_appreciation_abbrev_pct(lang_selector: LangSelector) -> String {
        LOCALES
            .lookup(
                lang_selector_to_language_id(&lang_selector),
                "balance_sheet_statement.appreciation_abbrev_pct",
            )
            .unwrap_or_default()
    }

    /// I18n for spendable_distributions
    ///
    ///   * **lang_selector** - Language selector
    ///   * _return_ - The string for language
    pub fn i18n_spendable_distributions(lang_selector: LangSelector) -> String {
        LOCALES
            .lookup(
                lang_selector_to_language_id(&lang_selector),
                "balance_sheet_statement.spendable_distributions",
            )
            .unwrap_or_default()
    }

    /// I18n for buy_sell
    ///
    ///   * **lang_selector** - Language selector
    ///   * _return_ - The string for language
    pub fn i18n_buy_sell(lang_selector: LangSelector) -> String {
        LOCALES
            .lookup(
                lang_selector_to_language_id(&lang_selector),
                "balance_sheet_statement.buy_sell",
            )
            .unwrap_or_default()
    }

    // α <mod-def balance_sheet_statement>
    // ω <mod-def balance_sheet_statement>
}

/// Functions for i18n strings in BondSpecComponent
pub mod bond_spec_component {
    ////////////////////////////////////////////////////////////////////////////////////
    // --- module uses ---
    ////////////////////////////////////////////////////////////////////////////////////
    use super::*;

    ////////////////////////////////////////////////////////////////////////////////////
    // --- functions ---
    ////////////////////////////////////////////////////////////////////////////////////
    /// I18n for annual_coupon
    ///
    ///   * **lang_selector** - Language selector
    ///   * _return_ - The string for language
    pub fn i18n_annual_coupon(lang_selector: LangSelector) -> String {
        LOCALES
            .lookup(
                lang_selector_to_language_id(&lang_selector),
                "bond_spec_component.annual_coupon",
            )
            .unwrap_or_default()
    }

    /// I18n for maturity_year
    ///
    ///   * **lang_selector** - Language selector
    ///   * _return_ - The string for language
    pub fn i18n_maturity_year(lang_selector: LangSelector) -> String {
        LOCALES
            .lookup(
                lang_selector_to_language_id(&lang_selector),
                "bond_spec_component.maturity_year",
            )
            .unwrap_or_default()
    }

    // α <mod-def bond_spec_component>
    // ω <mod-def bond_spec_component>
}

/// Functions for i18n strings in CashFlowStatement
pub mod cash_flow_statement {
    ////////////////////////////////////////////////////////////////////////////////////
    // --- module uses ---
    ////////////////////////////////////////////////////////////////////////////////////
    use super::*;

    ////////////////////////////////////////////////////////////////////////////////////
    // --- functions ---
    ////////////////////////////////////////////////////////////////////////////////////
    /// I18n for cash_flow_statement
    ///
    ///   * **lang_selector** - Language selector
    ///   * _return_ - The string for language
    pub fn i18n_cash_flow_statement(lang_selector: LangSelector) -> String {
        LOCALES
            .lookup(
                lang_selector_to_language_id(&lang_selector),
                "cash_flow_statement.cash_flow_statement",
            )
            .unwrap_or_default()
    }

    /// I18n for cash_flows
    ///
    ///   * **lang_selector** - Language selector
    ///   * _return_ - The string for language
    pub fn i18n_cash_flows(lang_selector: LangSelector) -> String {
        LOCALES
            .lookup(
                lang_selector_to_language_id(&lang_selector),
                "cash_flow_statement.cash_flows",
            )
            .unwrap_or_default()
    }

    /// I18n for value
    ///
    ///   * **lang_selector** - Language selector
    ///   * _return_ - The string for language
    pub fn i18n_value(lang_selector: LangSelector) -> String {
        LOCALES
            .lookup(
                lang_selector_to_language_id(&lang_selector),
                "cash_flow_statement.value",
            )
            .unwrap_or_default()
    }

    // α <mod-def cash_flow_statement>
    // ω <mod-def cash_flow_statement>
}

/// Functions for i18n strings in CashFlowTimeline
pub mod cash_flow_timeline {
    ////////////////////////////////////////////////////////////////////////////////////
    // --- module uses ---
    ////////////////////////////////////////////////////////////////////////////////////
    use super::*;

    ////////////////////////////////////////////////////////////////////////////////////
    // --- functions ---
    ////////////////////////////////////////////////////////////////////////////////////
    /// I18n for cash_flow_timeline
    ///
    ///   * **lang_selector** - Language selector
    ///   * _return_ - The string for language
    pub fn i18n_cash_flow_timeline(lang_selector: LangSelector) -> String {
        LOCALES
            .lookup(
                lang_selector_to_language_id(&lang_selector),
                "cash_flow_timeline.cash_flow_timeline",
            )
            .unwrap_or_default()
    }

    /// I18n for zoom
    ///
    ///   * **lang_selector** - Language selector
    ///   * _return_ - The string for language
    pub fn i18n_zoom(lang_selector: LangSelector) -> String {
        LOCALES
            .lookup(
                lang_selector_to_language_id(&lang_selector),
                "cash_flow_timeline.zoom",
            )
            .unwrap_or_default()
    }

    /// I18n for balanced
    ///
    ///   * **lang_selector** - Language selector
    ///   * _return_ - The string for language
    pub fn i18n_balanced(lang_selector: LangSelector) -> String {
        LOCALES
            .lookup(
                lang_selector_to_language_id(&lang_selector),
                "cash_flow_timeline.balanced",
            )
            .unwrap_or_default()
    }

    // α <mod-def cash_flow_timeline>
    // ω <mod-def cash_flow_timeline>
}

/// Functions for i18n strings in DistributionPolicyComponent
pub mod distribution_policy_component {
    ////////////////////////////////////////////////////////////////////////////////////
    // --- module uses ---
    ////////////////////////////////////////////////////////////////////////////////////
    use super::*;

    ////////////////////////////////////////////////////////////////////////////////////
    // --- functions ---
    ////////////////////////////////////////////////////////////////////////////////////
    /// I18n for no_distributions
    ///
    ///   * **lang_selector** - Language selector
    ///   * _return_ - The string for language
    pub fn i18n_no_distributions(lang_selector: LangSelector) -> String {
        LOCALES
            .lookup(
                lang_selector_to_language_id(&lang_selector),
                "distribution_policy_component.no_distributions",
            )
            .unwrap_or_default()
    }

    /// I18n for distributions
    ///
    ///   * **lang_selector** - Language selector
    ///   * _return_ - The string for language
    pub fn i18n_distributions(lang_selector: LangSelector) -> String {
        LOCALES
            .lookup(
                lang_selector_to_language_id(&lang_selector),
                "distribution_policy_component.distributions",
            )
            .unwrap_or_default()
    }

    /// I18n for bond
    ///
    ///   * **lang_selector** - Language selector
    ///   * _return_ - The string for language
    pub fn i18n_bond(lang_selector: LangSelector) -> String {
        LOCALES
            .lookup(
                lang_selector_to_language_id(&lang_selector),
                "distribution_policy_component.bond",
            )
            .unwrap_or_default()
    }

    // α <mod-def distribution_policy_component>
    // ω <mod-def distribution_policy_component>
}

/// Functions for i18n strings in DistributionSpecComponent
pub mod distribution_spec_component {
    ////////////////////////////////////////////////////////////////////////////////////
    // --- module uses ---
    ////////////////////////////////////////////////////////////////////////////////////
    use super::*;

    ////////////////////////////////////////////////////////////////////////////////////
    // --- functions ---
    ////////////////////////////////////////////////////////////////////////////////////
    /// I18n for annual_pct_placeholder
    ///
    ///   * **lang_selector** - Language selector
    ///   * _return_ - The string for language
    pub fn i18n_annual_pct_placeholder(lang_selector: LangSelector) -> String {
        LOCALES
            .lookup(
                lang_selector_to_language_id(&lang_selector),
                "distribution_spec_component.annual_pct_placeholder",
            )
            .unwrap_or_default()
    }

    /// I18n for qualified_div
    ///
    ///   * **lang_selector** - Language selector
    ///   * _return_ - The string for language
    pub fn i18n_qualified_div(lang_selector: LangSelector) -> String {
        LOCALES
            .lookup(
                lang_selector_to_language_id(&lang_selector),
                "distribution_spec_component.qualified_div",
            )
            .unwrap_or_default()
    }

    /// I18n for unqualified_div
    ///
    ///   * **lang_selector** - Language selector
    ///   * _return_ - The string for language
    pub fn i18n_unqualified_div(lang_selector: LangSelector) -> String {
        LOCALES
            .lookup(
                lang_selector_to_language_id(&lang_selector),
                "distribution_spec_component.unqualified_div",
            )
            .unwrap_or_default()
    }

    /// I18n for capital_gain
    ///
    ///   * **lang_selector** - Language selector
    ///   * _return_ - The string for language
    pub fn i18n_capital_gain(lang_selector: LangSelector) -> String {
        LOCALES
            .lookup(
                lang_selector_to_language_id(&lang_selector),
                "distribution_spec_component.capital_gain",
            )
            .unwrap_or_default()
    }

    /// I18n for interest
    ///
    ///   * **lang_selector** - Language selector
    ///   * _return_ - The string for language
    pub fn i18n_interest(lang_selector: LangSelector) -> String {
        LOCALES
            .lookup(
                lang_selector_to_language_id(&lang_selector),
                "distribution_spec_component.interest",
            )
            .unwrap_or_default()
    }

    // α <mod-def distribution_spec_component>
    // ω <mod-def distribution_spec_component>
}

/// Functions for i18n strings in DossierEditor
pub mod dossier_editor {
    ////////////////////////////////////////////////////////////////////////////////////
    // --- module uses ---
    ////////////////////////////////////////////////////////////////////////////////////
    use super::*;

    ////////////////////////////////////////////////////////////////////////////////////
    // --- functions ---
    ////////////////////////////////////////////////////////////////////////////////////
    /// I18n for dossier_name
    ///
    ///   * **lang_selector** - Language selector
    ///   * _return_ - The string for language
    pub fn i18n_dossier_name(lang_selector: LangSelector) -> String {
        LOCALES
            .lookup(
                lang_selector_to_language_id(&lang_selector),
                "dossier_editor.dossier_name",
            )
            .unwrap_or_default()
    }

    /// I18n for dossier_editor
    ///
    ///   * **lang_selector** - Language selector
    ///   * _return_ - The string for language
    pub fn i18n_dossier_editor(lang_selector: LangSelector) -> String {
        LOCALES
            .lookup(
                lang_selector_to_language_id(&lang_selector),
                "dossier_editor.dossier_editor",
            )
            .unwrap_or_default()
    }

    /// I18n for people
    ///
    ///   * **lang_selector** - Language selector
    ///   * _return_ - The string for language
    pub fn i18n_people(lang_selector: LangSelector) -> String {
        LOCALES
            .lookup(
                lang_selector_to_language_id(&lang_selector),
                "dossier_editor.people",
            )
            .unwrap_or_default()
    }

    /// I18n for worths
    ///
    ///   * **lang_selector** - Language selector
    ///   * _return_ - The string for language
    pub fn i18n_worths(lang_selector: LangSelector) -> String {
        LOCALES
            .lookup(
                lang_selector_to_language_id(&lang_selector),
                "dossier_editor.worths",
            )
            .unwrap_or_default()
    }

    /// I18n for accounts
    ///
    ///   * **lang_selector** - Language selector
    ///   * _return_ - The string for language
    pub fn i18n_accounts(lang_selector: LangSelector) -> String {
        LOCALES
            .lookup(
                lang_selector_to_language_id(&lang_selector),
                "dossier_editor.accounts",
            )
            .unwrap_or_default()
    }

    /// I18n for cash_flows
    ///
    ///   * **lang_selector** - Language selector
    ///   * _return_ - The string for language
    pub fn i18n_cash_flows(lang_selector: LangSelector) -> String {
        LOCALES
            .lookup(
                lang_selector_to_language_id(&lang_selector),
                "dossier_editor.cash_flows",
            )
            .unwrap_or_default()
    }

    /// I18n for taxes
    ///
    ///   * **lang_selector** - Language selector
    ///   * _return_ - The string for language
    pub fn i18n_taxes(lang_selector: LangSelector) -> String {
        LOCALES
            .lookup(
                lang_selector_to_language_id(&lang_selector),
                "dossier_editor.taxes",
            )
            .unwrap_or_default()
    }

    /// I18n for assumptions
    ///
    ///   * **lang_selector** - Language selector
    ///   * _return_ - The string for language
    pub fn i18n_assumptions(lang_selector: LangSelector) -> String {
        LOCALES
            .lookup(
                lang_selector_to_language_id(&lang_selector),
                "dossier_editor.assumptions",
            )
            .unwrap_or_default()
    }

    // α <mod-def dossier_editor>
    // ω <mod-def dossier_editor>
}

/// Functions for i18n strings in DossierResolvedView
pub mod dossier_resolved_view {
    ////////////////////////////////////////////////////////////////////////////////////
    // --- module uses ---
    ////////////////////////////////////////////////////////////////////////////////////
    use super::*;

    ////////////////////////////////////////////////////////////////////////////////////
    // --- functions ---
    ////////////////////////////////////////////////////////////////////////////////////
    /// I18n for dim_index
    ///
    ///   * **lang_selector** - Language selector
    ///   * _return_ - The string for language
    pub fn i18n_dim_index(lang_selector: LangSelector) -> String {
        LOCALES
            .lookup(
                lang_selector_to_language_id(&lang_selector),
                "dossier_resolved_view.dim_index",
            )
            .unwrap_or_default()
    }

    /// I18n for matrix_index
    ///
    ///   * **lang_selector** - Language selector
    ///   * _return_ - The string for language
    pub fn i18n_matrix_index(lang_selector: LangSelector) -> String {
        LOCALES
            .lookup(
                lang_selector_to_language_id(&lang_selector),
                "dossier_resolved_view.matrix_index",
            )
            .unwrap_or_default()
    }

    /// I18n for name
    ///
    ///   * **lang_selector** - Language selector
    ///   * _return_ - The string for language
    pub fn i18n_name(lang_selector: LangSelector) -> String {
        LOCALES
            .lookup(
                lang_selector_to_language_id(&lang_selector),
                "dossier_resolved_view.name",
            )
            .unwrap_or_default()
    }

    /// I18n for user
    ///
    ///   * **lang_selector** - Language selector
    ///   * _return_ - The string for language
    pub fn i18n_user(lang_selector: LangSelector) -> String {
        LOCALES
            .lookup(
                lang_selector_to_language_id(&lang_selector),
                "dossier_resolved_view.user",
            )
            .unwrap_or_default()
    }

    /// I18n for outlook
    ///
    ///   * **lang_selector** - Language selector
    ///   * _return_ - The string for language
    pub fn i18n_outlook(lang_selector: LangSelector) -> String {
        LOCALES
            .lookup(
                lang_selector_to_language_id(&lang_selector),
                "dossier_resolved_view.outlook",
            )
            .unwrap_or_default()
    }

    /// I18n for category
    ///
    ///   * **lang_selector** - Language selector
    ///   * _return_ - The string for language
    pub fn i18n_category(lang_selector: LangSelector) -> String {
        LOCALES
            .lookup(
                lang_selector_to_language_id(&lang_selector),
                "dossier_resolved_view.category",
            )
            .unwrap_or_default()
    }

    /// I18n for growth
    ///
    ///   * **lang_selector** - Language selector
    ///   * _return_ - The string for language
    pub fn i18n_growth(lang_selector: LangSelector) -> String {
        LOCALES
            .lookup(
                lang_selector_to_language_id(&lang_selector),
                "dossier_resolved_view.growth",
            )
            .unwrap_or_default()
    }

    /// I18n for resolved_dossier
    ///
    ///   * **lang_selector** - Language selector
    ///   * _return_ - The string for language
    pub fn i18n_resolved_dossier(lang_selector: LangSelector) -> String {
        LOCALES
            .lookup(
                lang_selector_to_language_id(&lang_selector),
                "dossier_resolved_view.resolved_dossier",
            )
            .unwrap_or_default()
    }

    // α <mod-def dossier_resolved_view>
    // ω <mod-def dossier_resolved_view>
}

/// Functions for i18n strings in FlowSpecComponent
pub mod flow_spec_component {
    ////////////////////////////////////////////////////////////////////////////////////
    // --- module uses ---
    ////////////////////////////////////////////////////////////////////////////////////
    use super::*;

    ////////////////////////////////////////////////////////////////////////////////////
    // --- functions ---
    ////////////////////////////////////////////////////////////////////////////////////
    /// I18n for name
    ///
    ///   * **lang_selector** - Language selector
    ///   * _return_ - The string for language
    pub fn i18n_name(lang_selector: LangSelector) -> String {
        LOCALES
            .lookup(
                lang_selector_to_language_id(&lang_selector),
                "flow_spec_component.name",
            )
            .unwrap_or_default()
    }

    /// I18n for in_flow
    ///
    ///   * **lang_selector** - Language selector
    ///   * _return_ - The string for language
    pub fn i18n_in_flow(lang_selector: LangSelector) -> String {
        LOCALES
            .lookup(
                lang_selector_to_language_id(&lang_selector),
                "flow_spec_component.in_flow",
            )
            .unwrap_or_default()
    }

    /// I18n for out_flow
    ///
    ///   * **lang_selector** - Language selector
    ///   * _return_ - The string for language
    pub fn i18n_out_flow(lang_selector: LangSelector) -> String {
        LOCALES
            .lookup(
                lang_selector_to_language_id(&lang_selector),
                "flow_spec_component.out_flow",
            )
            .unwrap_or_default()
    }

    /// I18n for cash_flow
    ///
    ///   * **lang_selector** - Language selector
    ///   * _return_ - The string for language
    pub fn i18n_cash_flow(lang_selector: LangSelector) -> String {
        LOCALES
            .lookup(
                lang_selector_to_language_id(&lang_selector),
                "flow_spec_component.cash_flow",
            )
            .unwrap_or_default()
    }

    /// I18n for custom_flows
    ///
    ///   * **lang_selector** - Language selector
    ///   * _return_ - The string for language
    pub fn i18n_custom_flows(lang_selector: LangSelector) -> String {
        LOCALES
            .lookup(
                lang_selector_to_language_id(&lang_selector),
                "flow_spec_component.custom_flows",
            )
            .unwrap_or_default()
    }

    /// I18n for growing_flows
    ///
    ///   * **lang_selector** - Language selector
    ///   * _return_ - The string for language
    pub fn i18n_growing_flows(lang_selector: LangSelector) -> String {
        LOCALES
            .lookup(
                lang_selector_to_language_id(&lang_selector),
                "flow_spec_component.growing_flows",
            )
            .unwrap_or_default()
    }

    // α <mod-def flow_spec_component>
    // ω <mod-def flow_spec_component>
}

/// Functions for i18n strings in FlowSpecsGrid
pub mod flow_specs_grid {
    ////////////////////////////////////////////////////////////////////////////////////
    // --- module uses ---
    ////////////////////////////////////////////////////////////////////////////////////
    use super::*;

    ////////////////////////////////////////////////////////////////////////////////////
    // --- functions ---
    ////////////////////////////////////////////////////////////////////////////////////
    /// I18n for cash_flows
    ///
    ///   * **lang_selector** - Language selector
    ///   * _return_ - The string for language
    pub fn i18n_cash_flows(lang_selector: LangSelector) -> String {
        LOCALES
            .lookup(
                lang_selector_to_language_id(&lang_selector),
                "flow_specs_grid.cash_flows",
            )
            .unwrap_or_default()
    }

    /// I18n for name
    ///
    ///   * **lang_selector** - Language selector
    ///   * _return_ - The string for language
    pub fn i18n_name(lang_selector: LangSelector) -> String {
        LOCALES
            .lookup(
                lang_selector_to_language_id(&lang_selector),
                "flow_specs_grid.name",
            )
            .unwrap_or_default()
    }

    /// I18n for in_out
    ///
    ///   * **lang_selector** - Language selector
    ///   * _return_ - The string for language
    pub fn i18n_in_out(lang_selector: LangSelector) -> String {
        LOCALES
            .lookup(
                lang_selector_to_language_id(&lang_selector),
                "flow_specs_grid.in_out",
            )
            .unwrap_or_default()
    }

    /// I18n for year_range
    ///
    ///   * **lang_selector** - Language selector
    ///   * _return_ - The string for language
    pub fn i18n_year_range(lang_selector: LangSelector) -> String {
        LOCALES
            .lookup(
                lang_selector_to_language_id(&lang_selector),
                "flow_specs_grid.year_range",
            )
            .unwrap_or_default()
    }

    /// I18n for new_cash_flow
    ///
    ///   * **lang_selector** - Language selector
    ///   * _return_ - The string for language
    pub fn i18n_new_cash_flow(lang_selector: LangSelector) -> String {
        LOCALES
            .lookup(
                lang_selector_to_language_id(&lang_selector),
                "flow_specs_grid.new_cash_flow",
            )
            .unwrap_or_default()
    }

    // α <mod-def flow_specs_grid>
    // ω <mod-def flow_specs_grid>
}

/// Functions for i18n strings in ForecastConfigComponent
pub mod forecast_config_component {
    ////////////////////////////////////////////////////////////////////////////////////
    // --- module uses ---
    ////////////////////////////////////////////////////////////////////////////////////
    use super::*;

    ////////////////////////////////////////////////////////////////////////////////////
    // --- functions ---
    ////////////////////////////////////////////////////////////////////////////////////
    /// I18n for forecast_config
    ///
    ///   * **lang_selector** - Language selector
    ///   * _return_ - The string for language
    pub fn i18n_forecast_config(lang_selector: LangSelector) -> String {
        LOCALES
            .lookup(
                lang_selector_to_language_id(&lang_selector),
                "forecast_config_component.forecast_config",
            )
            .unwrap_or_default()
    }

    /// I18n for scoped_year_range
    ///
    ///   * **lang_selector** - Language selector
    ///   * _return_ - The string for language
    pub fn i18n_scoped_year_range(lang_selector: LangSelector) -> String {
        LOCALES
            .lookup(
                lang_selector_to_language_id(&lang_selector),
                "forecast_config_component.scoped_year_range",
            )
            .unwrap_or_default()
    }

    /// I18n for year_range_override
    ///
    ///   * **lang_selector** - Language selector
    ///   * _return_ - The string for language
    pub fn i18n_year_range_override(lang_selector: LangSelector) -> String {
        LOCALES
            .lookup(
                lang_selector_to_language_id(&lang_selector),
                "forecast_config_component.year_range_override",
            )
            .unwrap_or_default()
    }

    // α <mod-def forecast_config_component>
    // ω <mod-def forecast_config_component>
}

/// Functions for i18n strings in ForecastIdSelector
pub mod forecast_id_selector {
    ////////////////////////////////////////////////////////////////////////////////////
    // --- module uses ---
    ////////////////////////////////////////////////////////////////////////////////////
    use super::*;

    ////////////////////////////////////////////////////////////////////////////////////
    // --- functions ---
    ////////////////////////////////////////////////////////////////////////////////////
    /// I18n for geometric_mean_forecast
    ///
    ///   * **lang_selector** - Language selector
    ///   * _return_ - The string for language
    pub fn i18n_geometric_mean_forecast(lang_selector: LangSelector) -> String {
        LOCALES
            .lookup(
                lang_selector_to_language_id(&lang_selector),
                "forecast_id_selector.geometric_mean_forecast",
            )
            .unwrap_or_default()
    }

    /// I18n for random_forecast_id
    ///
    ///   * **lang_selector** - Language selector
    ///   * _return_ - The string for language
    pub fn i18n_random_forecast_id(lang_selector: LangSelector) -> String {
        LOCALES
            .lookup(
                lang_selector_to_language_id(&lang_selector),
                "forecast_id_selector.random_forecast_id",
            )
            .unwrap_or_default()
    }

    // α <mod-def forecast_id_selector>
    // ω <mod-def forecast_id_selector>
}

/// Functions for i18n strings in ForecastSummaryComponent
pub mod forecast_summary_component {
    ////////////////////////////////////////////////////////////////////////////////////
    // --- module uses ---
    ////////////////////////////////////////////////////////////////////////////////////
    use super::*;

    ////////////////////////////////////////////////////////////////////////////////////
    // --- functions ---
    ////////////////////////////////////////////////////////////////////////////////////
    /// I18n for forecast_summary
    ///
    ///   * **lang_selector** - Language selector
    ///   * _return_ - The string for language
    pub fn i18n_forecast_summary(lang_selector: LangSelector) -> String {
        LOCALES
            .lookup(
                lang_selector_to_language_id(&lang_selector),
                "forecast_summary_component.forecast_summary",
            )
            .unwrap_or_default()
    }

    // α <mod-def forecast_summary_component>
    // ω <mod-def forecast_summary_component>
}

/// Functions for i18n strings in ForecasterComponent
pub mod forecaster_component {
    ////////////////////////////////////////////////////////////////////////////////////
    // --- module uses ---
    ////////////////////////////////////////////////////////////////////////////////////
    use super::*;

    ////////////////////////////////////////////////////////////////////////////////////
    // --- functions ---
    ////////////////////////////////////////////////////////////////////////////////////
    /// I18n for forecaster
    ///
    ///   * **lang_selector** - Language selector
    ///   * _return_ - The string for language
    pub fn i18n_forecaster(lang_selector: LangSelector) -> String {
        LOCALES
            .lookup(
                lang_selector_to_language_id(&lang_selector),
                "forecaster_component.forecaster",
            )
            .unwrap_or_default()
    }

    // α <mod-def forecaster_component>
    // ω <mod-def forecaster_component>
}

/// Functions for i18n strings in GrowingFlowSpecComponent
pub mod growing_flow_spec_component {
    ////////////////////////////////////////////////////////////////////////////////////
    // --- module uses ---
    ////////////////////////////////////////////////////////////////////////////////////
    use super::*;

    ////////////////////////////////////////////////////////////////////////////////////
    // --- functions ---
    ////////////////////////////////////////////////////////////////////////////////////
    /// I18n for initial
    ///
    ///   * **lang_selector** - Language selector
    ///   * _return_ - The string for language
    pub fn i18n_initial(lang_selector: LangSelector) -> String {
        LOCALES
            .lookup(
                lang_selector_to_language_id(&lang_selector),
                "growing_flow_spec_component.initial",
            )
            .unwrap_or_default()
    }

    /// I18n for year_range
    ///
    ///   * **lang_selector** - Language selector
    ///   * _return_ - The string for language
    pub fn i18n_year_range(lang_selector: LangSelector) -> String {
        LOCALES
            .lookup(
                lang_selector_to_language_id(&lang_selector),
                "growing_flow_spec_component.year_range",
            )
            .unwrap_or_default()
    }

    /// I18n for year_placeholder
    ///
    ///   * **lang_selector** - Language selector
    ///   * _return_ - The string for language
    pub fn i18n_year_placeholder(lang_selector: LangSelector) -> String {
        LOCALES
            .lookup(
                lang_selector_to_language_id(&lang_selector),
                "growing_flow_spec_component.year_placeholder",
            )
            .unwrap_or_default()
    }

    /// I18n for value_placeholder
    ///
    ///   * **lang_selector** - Language selector
    ///   * _return_ - The string for language
    pub fn i18n_value_placeholder(lang_selector: LangSelector) -> String {
        LOCALES
            .lookup(
                lang_selector_to_language_id(&lang_selector),
                "growing_flow_spec_component.value_placeholder",
            )
            .unwrap_or_default()
    }

    // α <mod-def growing_flow_spec_component>
    // ω <mod-def growing_flow_spec_component>
}

/// Functions for i18n strings in GrowthAssumptionComponent
pub mod growth_assumption_component {
    ////////////////////////////////////////////////////////////////////////////////////
    // --- module uses ---
    ////////////////////////////////////////////////////////////////////////////////////
    use super::*;

    ////////////////////////////////////////////////////////////////////////////////////
    // --- functions ---
    ////////////////////////////////////////////////////////////////////////////////////
    /// I18n for normal_spec
    ///
    ///   * **lang_selector** - Language selector
    ///   * _return_ - The string for language
    pub fn i18n_normal_spec(lang_selector: LangSelector) -> String {
        LOCALES
            .lookup(
                lang_selector_to_language_id(&lang_selector),
                "growth_assumption_component.normal_spec",
            )
            .unwrap_or_default()
    }

    /// I18n for fixed_rate_curve
    ///
    ///   * **lang_selector** - Language selector
    ///   * _return_ - The string for language
    pub fn i18n_fixed_rate_curve(lang_selector: LangSelector) -> String {
        LOCALES
            .lookup(
                lang_selector_to_language_id(&lang_selector),
                "growth_assumption_component.fixed_rate_curve",
            )
            .unwrap_or_default()
    }

    // α <mod-def growth_assumption_component>
    // ω <mod-def growth_assumption_component>
}

/// Functions for i18n strings in HistoricRiskReturnComponent
pub mod historic_risk_return_component {
    ////////////////////////////////////////////////////////////////////////////////////
    // --- module uses ---
    ////////////////////////////////////////////////////////////////////////////////////
    use super::*;

    ////////////////////////////////////////////////////////////////////////////////////
    // --- functions ---
    ////////////////////////////////////////////////////////////////////////////////////
    /// I18n for holding_type
    ///
    ///   * **lang_selector** - Language selector
    ///   * _return_ - The string for language
    pub fn i18n_holding_type(lang_selector: LangSelector) -> String {
        LOCALES
            .lookup(
                lang_selector_to_language_id(&lang_selector),
                "historic_risk_return_component.holding_type",
            )
            .unwrap_or_default()
    }

    // α <mod-def historic_risk_return_component>
    // ω <mod-def historic_risk_return_component>
}

/// Functions for i18n strings in HoldingComponent
pub mod holding_component {
    ////////////////////////////////////////////////////////////////////////////////////
    // --- module uses ---
    ////////////////////////////////////////////////////////////////////////////////////
    use super::*;

    ////////////////////////////////////////////////////////////////////////////////////
    // --- functions ---
    ////////////////////////////////////////////////////////////////////////////////////
    /// I18n for cost
    ///
    ///   * **lang_selector** - Language selector
    ///   * _return_ - The string for language
    pub fn i18n_cost(lang_selector: LangSelector) -> String {
        LOCALES
            .lookup(
                lang_selector_to_language_id(&lang_selector),
                "holding_component.cost",
            )
            .unwrap_or_default()
    }

    /// I18n for current_price
    ///
    ///   * **lang_selector** - Language selector
    ///   * _return_ - The string for language
    pub fn i18n_current_price(lang_selector: LangSelector) -> String {
        LOCALES
            .lookup(
                lang_selector_to_language_id(&lang_selector),
                "holding_component.current_price",
            )
            .unwrap_or_default()
    }

    /// I18n for distributions
    ///
    ///   * **lang_selector** - Language selector
    ///   * _return_ - The string for language
    pub fn i18n_distributions(lang_selector: LangSelector) -> String {
        LOCALES
            .lookup(
                lang_selector_to_language_id(&lang_selector),
                "holding_component.distributions",
            )
            .unwrap_or_default()
    }

    /// I18n for growth
    ///
    ///   * **lang_selector** - Language selector
    ///   * _return_ - The string for language
    pub fn i18n_growth(lang_selector: LangSelector) -> String {
        LOCALES
            .lookup(
                lang_selector_to_language_id(&lang_selector),
                "holding_component.growth",
            )
            .unwrap_or_default()
    }

    /// I18n for holding
    ///
    ///   * **lang_selector** - Language selector
    ///   * _return_ - The string for language
    pub fn i18n_holding(lang_selector: LangSelector) -> String {
        LOCALES
            .lookup(
                lang_selector_to_language_id(&lang_selector),
                "holding_component.holding",
            )
            .unwrap_or_default()
    }

    /// I18n for mv
    ///
    ///   * **lang_selector** - Language selector
    ///   * _return_ - The string for language
    pub fn i18n_mv(lang_selector: LangSelector) -> String {
        LOCALES
            .lookup(
                lang_selector_to_language_id(&lang_selector),
                "holding_component.mv",
            )
            .unwrap_or_default()
    }

    /// I18n for price_placeholder
    ///
    ///   * **lang_selector** - Language selector
    ///   * _return_ - The string for language
    pub fn i18n_price_placeholder(lang_selector: LangSelector) -> String {
        LOCALES
            .lookup(
                lang_selector_to_language_id(&lang_selector),
                "holding_component.price_placeholder",
            )
            .unwrap_or_default()
    }

    /// I18n for qty_placeholder
    ///
    ///   * **lang_selector** - Language selector
    ///   * _return_ - The string for language
    pub fn i18n_qty_placeholder(lang_selector: LangSelector) -> String {
        LOCALES
            .lookup(
                lang_selector_to_language_id(&lang_selector),
                "holding_component.qty_placeholder",
            )
            .unwrap_or_default()
    }

    /// I18n for quantity
    ///
    ///   * **lang_selector** - Language selector
    ///   * _return_ - The string for language
    pub fn i18n_quantity(lang_selector: LangSelector) -> String {
        LOCALES
            .lookup(
                lang_selector_to_language_id(&lang_selector),
                "holding_component.quantity",
            )
            .unwrap_or_default()
    }

    /// I18n for symbol
    ///
    ///   * **lang_selector** - Language selector
    ///   * _return_ - The string for language
    pub fn i18n_symbol(lang_selector: LangSelector) -> String {
        LOCALES
            .lookup(
                lang_selector_to_language_id(&lang_selector),
                "holding_component.symbol",
            )
            .unwrap_or_default()
    }

    /// I18n for ugl
    ///
    ///   * **lang_selector** - Language selector
    ///   * _return_ - The string for language
    pub fn i18n_ugl(lang_selector: LangSelector) -> String {
        LOCALES
            .lookup(
                lang_selector_to_language_id(&lang_selector),
                "holding_component.ugl",
            )
            .unwrap_or_default()
    }

    /// I18n for year_placeholder
    ///
    ///   * **lang_selector** - Language selector
    ///   * _return_ - The string for language
    pub fn i18n_year_placeholder(lang_selector: LangSelector) -> String {
        LOCALES
            .lookup(
                lang_selector_to_language_id(&lang_selector),
                "holding_component.year_placeholder",
            )
            .unwrap_or_default()
    }

    // α <mod-def holding_component>
    // ω <mod-def holding_component>
}

/// Functions for i18n strings in HoldingsGrid
pub mod holdings_grid {
    ////////////////////////////////////////////////////////////////////////////////////
    // --- module uses ---
    ////////////////////////////////////////////////////////////////////////////////////
    use super::*;

    ////////////////////////////////////////////////////////////////////////////////////
    // --- functions ---
    ////////////////////////////////////////////////////////////////////////////////////
    /// I18n for symbol
    ///
    ///   * **lang_selector** - Language selector
    ///   * _return_ - The string for language
    pub fn i18n_symbol(lang_selector: LangSelector) -> String {
        LOCALES
            .lookup(
                lang_selector_to_language_id(&lang_selector),
                "holdings_grid.symbol",
            )
            .unwrap_or_default()
    }

    /// I18n for mv
    ///
    ///   * **lang_selector** - Language selector
    ///   * _return_ - The string for language
    pub fn i18n_mv(lang_selector: LangSelector) -> String {
        LOCALES
            .lookup(
                lang_selector_to_language_id(&lang_selector),
                "holdings_grid.mv",
            )
            .unwrap_or_default()
    }

    /// I18n for cb
    ///
    ///   * **lang_selector** - Language selector
    ///   * _return_ - The string for language
    pub fn i18n_cb(lang_selector: LangSelector) -> String {
        LOCALES
            .lookup(
                lang_selector_to_language_id(&lang_selector),
                "holdings_grid.cb",
            )
            .unwrap_or_default()
    }

    /// I18n for ugl
    ///
    ///   * **lang_selector** - Language selector
    ///   * _return_ - The string for language
    pub fn i18n_ugl(lang_selector: LangSelector) -> String {
        LOCALES
            .lookup(
                lang_selector_to_language_id(&lang_selector),
                "holdings_grid.ugl",
            )
            .unwrap_or_default()
    }

    /// I18n for holdings
    ///
    ///   * **lang_selector** - Language selector
    ///   * _return_ - The string for language
    pub fn i18n_holdings(lang_selector: LangSelector) -> String {
        LOCALES
            .lookup(
                lang_selector_to_language_id(&lang_selector),
                "holdings_grid.holdings",
            )
            .unwrap_or_default()
    }

    /// I18n for new_holding
    ///
    ///   * **lang_selector** - Language selector
    ///   * _return_ - The string for language
    pub fn i18n_new_holding(lang_selector: LangSelector) -> String {
        LOCALES
            .lookup(
                lang_selector_to_language_id(&lang_selector),
                "holdings_grid.new_holding",
            )
            .unwrap_or_default()
    }

    // α <mod-def holdings_grid>
    // ω <mod-def holdings_grid>
}

/// Functions for i18n strings in InvestmentPlanView
pub mod investment_plan_view {
    ////////////////////////////////////////////////////////////////////////////////////
    // --- module uses ---
    ////////////////////////////////////////////////////////////////////////////////////
    use super::*;

    ////////////////////////////////////////////////////////////////////////////////////
    // --- functions ---
    ////////////////////////////////////////////////////////////////////////////////////
    /// I18n for name
    ///
    ///   * **lang_selector** - Language selector
    ///   * _return_ - The string for language
    pub fn i18n_name(lang_selector: LangSelector) -> String {
        LOCALES
            .lookup(
                lang_selector_to_language_id(&lang_selector),
                "investment_plan_view.name",
            )
            .unwrap_or_default()
    }

    /// I18n for value
    ///
    ///   * **lang_selector** - Language selector
    ///   * _return_ - The string for language
    pub fn i18n_value(lang_selector: LangSelector) -> String {
        LOCALES
            .lookup(
                lang_selector_to_language_id(&lang_selector),
                "investment_plan_view.value",
            )
            .unwrap_or_default()
    }

    /// I18n for current
    ///
    ///   * **lang_selector** - Language selector
    ///   * _return_ - The string for language
    pub fn i18n_current(lang_selector: LangSelector) -> String {
        LOCALES
            .lookup(
                lang_selector_to_language_id(&lang_selector),
                "investment_plan_view.current",
            )
            .unwrap_or_default()
    }

    /// I18n for target
    ///
    ///   * **lang_selector** - Language selector
    ///   * _return_ - The string for language
    pub fn i18n_target(lang_selector: LangSelector) -> String {
        LOCALES
            .lookup(
                lang_selector_to_language_id(&lang_selector),
                "investment_plan_view.target",
            )
            .unwrap_or_default()
    }

    /// I18n for total_investments
    ///
    ///   * **lang_selector** - Language selector
    ///   * _return_ - The string for language
    pub fn i18n_total_investments(lang_selector: LangSelector) -> String {
        LOCALES
            .lookup(
                lang_selector_to_language_id(&lang_selector),
                "investment_plan_view.total_investments",
            )
            .unwrap_or_default()
    }

    /// I18n for investment_plan
    ///
    ///   * **lang_selector** - Language selector
    ///   * _return_ - The string for language
    pub fn i18n_investment_plan(lang_selector: LangSelector) -> String {
        LOCALES
            .lookup(
                lang_selector_to_language_id(&lang_selector),
                "investment_plan_view.investment_plan",
            )
            .unwrap_or_default()
    }

    // α <mod-def investment_plan_view>
    // ω <mod-def investment_plan_view>
}

/// Functions for i18n strings in ItemGrowthComponent
pub mod item_growth_component {
    ////////////////////////////////////////////////////////////////////////////////////
    // --- module uses ---
    ////////////////////////////////////////////////////////////////////////////////////
    use super::*;

    ////////////////////////////////////////////////////////////////////////////////////
    // --- functions ---
    ////////////////////////////////////////////////////////////////////////////////////
    /// I18n for override_system_growth
    ///
    ///   * **lang_selector** - Language selector
    ///   * _return_ - The string for language
    pub fn i18n_override_system_growth(lang_selector: LangSelector) -> String {
        LOCALES
            .lookup(
                lang_selector_to_language_id(&lang_selector),
                "item_growth_component.override_system_growth",
            )
            .unwrap_or_default()
    }

    /// I18n for use_system_growth
    ///
    ///   * **lang_selector** - Language selector
    ///   * _return_ - The string for language
    pub fn i18n_use_system_growth(lang_selector: LangSelector) -> String {
        LOCALES
            .lookup(
                lang_selector_to_language_id(&lang_selector),
                "item_growth_component.use_system_growth",
            )
            .unwrap_or_default()
    }

    // α <mod-def item_growth_component>
    // ω <mod-def item_growth_component>
}

/// Functions for i18n strings in MatrixResolvedView
pub mod matrix_resolved_view {
    ////////////////////////////////////////////////////////////////////////////////////
    // --- module uses ---
    ////////////////////////////////////////////////////////////////////////////////////
    use super::*;

    ////////////////////////////////////////////////////////////////////////////////////
    // --- functions ---
    ////////////////////////////////////////////////////////////////////////////////////
    /// I18n for resolved_matrix
    ///
    ///   * **lang_selector** - Language selector
    ///   * _return_ - The string for language
    pub fn i18n_resolved_matrix(lang_selector: LangSelector) -> String {
        LOCALES
            .lookup(
                lang_selector_to_language_id(&lang_selector),
                "matrix_resolved_view.resolved_matrix",
            )
            .unwrap_or_default()
    }

    // α <mod-def matrix_resolved_view>
    // ω <mod-def matrix_resolved_view>
}

/// Functions for i18n strings in MonteConfigComponent
pub mod monte_config_component {
    ////////////////////////////////////////////////////////////////////////////////////
    // --- module uses ---
    ////////////////////////////////////////////////////////////////////////////////////
    use super::*;

    ////////////////////////////////////////////////////////////////////////////////////
    // --- functions ---
    ////////////////////////////////////////////////////////////////////////////////////
    /// I18n for monte_config
    ///
    ///   * **lang_selector** - Language selector
    ///   * _return_ - The string for language
    pub fn i18n_monte_config(lang_selector: LangSelector) -> String {
        LOCALES
            .lookup(
                lang_selector_to_language_id(&lang_selector),
                "monte_config_component.monte_config",
            )
            .unwrap_or_default()
    }

    // α <mod-def monte_config_component>
    // ω <mod-def monte_config_component>
}

/// Functions for i18n strings in MonteSimulationSummaryComponent
pub mod monte_simulation_summary_component {
    ////////////////////////////////////////////////////////////////////////////////////
    // --- module uses ---
    ////////////////////////////////////////////////////////////////////////////////////
    use super::*;

    ////////////////////////////////////////////////////////////////////////////////////
    // --- functions ---
    ////////////////////////////////////////////////////////////////////////////////////
    /// I18n for simulation_summary
    ///
    ///   * **lang_selector** - Language selector
    ///   * _return_ - The string for language
    pub fn i18n_simulation_summary(lang_selector: LangSelector) -> String {
        LOCALES
            .lookup(
                lang_selector_to_language_id(&lang_selector),
                "monte_simulation_summary_component.simulation_summary",
            )
            .unwrap_or_default()
    }

    // α <mod-def monte_simulation_summary_component>
    // ω <mod-def monte_simulation_summary_component>
}

/// Functions for i18n strings in MonteSimulatorComponent
pub mod monte_simulator_component {
    ////////////////////////////////////////////////////////////////////////////////////
    // --- module uses ---
    ////////////////////////////////////////////////////////////////////////////////////
    use super::*;

    ////////////////////////////////////////////////////////////////////////////////////
    // --- functions ---
    ////////////////////////////////////////////////////////////////////////////////////
    /// I18n for monte_simulator
    ///
    ///   * **lang_selector** - Language selector
    ///   * _return_ - The string for language
    pub fn i18n_monte_simulator(lang_selector: LangSelector) -> String {
        LOCALES
            .lookup(
                lang_selector_to_language_id(&lang_selector),
                "monte_simulator_component.monte_simulator",
            )
            .unwrap_or_default()
    }

    /// I18n for run_simulation
    ///
    ///   * **lang_selector** - Language selector
    ///   * _return_ - The string for language
    pub fn i18n_run_simulation(lang_selector: LangSelector) -> String {
        LOCALES
            .lookup(
                lang_selector_to_language_id(&lang_selector),
                "monte_simulator_component.run_simulation",
            )
            .unwrap_or_default()
    }

    /// I18n for forecast_count
    ///
    ///   * **lang_selector** - Language selector
    ///   * _return_ - The string for language
    pub fn i18n_forecast_count(lang_selector: LangSelector) -> String {
        LOCALES
            .lookup(
                lang_selector_to_language_id(&lang_selector),
                "monte_simulator_component.forecast_count",
            )
            .unwrap_or_default()
    }

    // α <mod-def monte_simulator_component>
    // ω <mod-def monte_simulator_component>
}

/// Functions for i18n strings in NormalLossComponent
pub mod normal_loss_component {
    ////////////////////////////////////////////////////////////////////////////////////
    // --- module uses ---
    ////////////////////////////////////////////////////////////////////////////////////
    use super::*;

    ////////////////////////////////////////////////////////////////////////////////////
    // --- functions ---
    ////////////////////////////////////////////////////////////////////////////////////
    /// I18n for gain_pct
    ///
    ///   * **lang_selector** - Language selector
    ///   * _return_ - The string for language
    pub fn i18n_gain_pct(lang_selector: LangSelector) -> String {
        LOCALES
            .lookup(
                lang_selector_to_language_id(&lang_selector),
                "normal_loss_component.gain_pct",
            )
            .unwrap_or_default()
    }

    /// I18n for prob_pct
    ///
    ///   * **lang_selector** - Language selector
    ///   * _return_ - The string for language
    pub fn i18n_prob_pct(lang_selector: LangSelector) -> String {
        LOCALES
            .lookup(
                lang_selector_to_language_id(&lang_selector),
                "normal_loss_component.prob_pct",
            )
            .unwrap_or_default()
    }

    /// I18n for prob_abbrev
    ///
    ///   * **lang_selector** - Language selector
    ///   * _return_ - The string for language
    pub fn i18n_prob_abbrev(lang_selector: LangSelector) -> String {
        LOCALES
            .lookup(
                lang_selector_to_language_id(&lang_selector),
                "normal_loss_component.prob_abbrev",
            )
            .unwrap_or_default()
    }

    /// I18n for cdf_sample
    ///
    ///   * **lang_selector** - Language selector
    ///   * _return_ - The string for language
    pub fn i18n_cdf_sample(lang_selector: LangSelector) -> String {
        LOCALES
            .lookup(
                lang_selector_to_language_id(&lang_selector),
                "normal_loss_component.cdf_sample",
            )
            .unwrap_or_default()
    }

    /// I18n for gain_prefix
    ///
    ///   * **lang_selector** - Language selector
    ///   * _return_ - The string for language
    pub fn i18n_gain_prefix(lang_selector: LangSelector) -> String {
        LOCALES
            .lookup(
                lang_selector_to_language_id(&lang_selector),
                "normal_loss_component.gain_prefix",
            )
            .unwrap_or_default()
    }

    /// I18n for loss_table
    ///
    ///   * **lang_selector** - Language selector
    ///   * _return_ - The string for language
    pub fn i18n_loss_table(lang_selector: LangSelector) -> String {
        LOCALES
            .lookup(
                lang_selector_to_language_id(&lang_selector),
                "normal_loss_component.loss_table",
            )
            .unwrap_or_default()
    }

    // α <mod-def normal_loss_component>
    // ω <mod-def normal_loss_component>
}

/// Functions for i18n strings in NormalSpecComponent
pub mod normal_spec_component {
    ////////////////////////////////////////////////////////////////////////////////////
    // --- module uses ---
    ////////////////////////////////////////////////////////////////////////////////////
    use super::*;

    ////////////////////////////////////////////////////////////////////////////////////
    // --- functions ---
    ////////////////////////////////////////////////////////////////////////////////////
    /// I18n for mean_placeholder
    ///
    ///   * **lang_selector** - Language selector
    ///   * _return_ - The string for language
    pub fn i18n_mean_placeholder(lang_selector: LangSelector) -> String {
        LOCALES
            .lookup(
                lang_selector_to_language_id(&lang_selector),
                "normal_spec_component.mean_placeholder",
            )
            .unwrap_or_default()
    }

    /// I18n for std_dev_placeholder
    ///
    ///   * **lang_selector** - Language selector
    ///   * _return_ - The string for language
    pub fn i18n_std_dev_placeholder(lang_selector: LangSelector) -> String {
        LOCALES
            .lookup(
                lang_selector_to_language_id(&lang_selector),
                "normal_spec_component.std_dev_placeholder",
            )
            .unwrap_or_default()
    }

    // α <mod-def normal_spec_component>
    // ω <mod-def normal_spec_component>
}

/// Functions for i18n strings in OkCancelComponent
pub mod ok_cancel_component {
    ////////////////////////////////////////////////////////////////////////////////////
    // --- module uses ---
    ////////////////////////////////////////////////////////////////////////////////////
    use super::*;

    ////////////////////////////////////////////////////////////////////////////////////
    // --- functions ---
    ////////////////////////////////////////////////////////////////////////////////////
    /// I18n for ok
    ///
    ///   * **lang_selector** - Language selector
    ///   * _return_ - The string for language
    pub fn i18n_ok(lang_selector: LangSelector) -> String {
        LOCALES
            .lookup(
                lang_selector_to_language_id(&lang_selector),
                "ok_cancel_component.ok",
            )
            .unwrap_or_default()
    }

    /// I18n for cancel
    ///
    ///   * **lang_selector** - Language selector
    ///   * _return_ - The string for language
    pub fn i18n_cancel(lang_selector: LangSelector) -> String {
        LOCALES
            .lookup(
                lang_selector_to_language_id(&lang_selector),
                "ok_cancel_component.cancel",
            )
            .unwrap_or_default()
    }

    // α <mod-def ok_cancel_component>
    // ω <mod-def ok_cancel_component>
}

/// Functions for i18n strings in PersonComponent
pub mod person_component {
    ////////////////////////////////////////////////////////////////////////////////////
    // --- module uses ---
    ////////////////////////////////////////////////////////////////////////////////////
    use super::*;

    ////////////////////////////////////////////////////////////////////////////////////
    // --- functions ---
    ////////////////////////////////////////////////////////////////////////////////////
    /// I18n for name
    ///
    ///   * **lang_selector** - Language selector
    ///   * _return_ - The string for language
    pub fn i18n_name(lang_selector: LangSelector) -> String {
        LOCALES
            .lookup(
                lang_selector_to_language_id(&lang_selector),
                "person_component.name",
            )
            .unwrap_or_default()
    }

    /// I18n for person
    ///
    ///   * **lang_selector** - Language selector
    ///   * _return_ - The string for language
    pub fn i18n_person(lang_selector: LangSelector) -> String {
        LOCALES
            .lookup(
                lang_selector_to_language_id(&lang_selector),
                "person_component.person",
            )
            .unwrap_or_default()
    }

    /// I18n for birth_year
    ///
    ///   * **lang_selector** - Language selector
    ///   * _return_ - The string for language
    pub fn i18n_birth_year(lang_selector: LangSelector) -> String {
        LOCALES
            .lookup(
                lang_selector_to_language_id(&lang_selector),
                "person_component.birth_year",
            )
            .unwrap_or_default()
    }

    /// I18n for role
    ///
    ///   * **lang_selector** - Language selector
    ///   * _return_ - The string for language
    pub fn i18n_role(lang_selector: LangSelector) -> String {
        LOCALES
            .lookup(
                lang_selector_to_language_id(&lang_selector),
                "person_component.role",
            )
            .unwrap_or_default()
    }

    // α <mod-def person_component>
    // ω <mod-def person_component>
}

/// Functions for i18n strings in PersonsGrid
pub mod persons_grid {
    ////////////////////////////////////////////////////////////////////////////////////
    // --- module uses ---
    ////////////////////////////////////////////////////////////////////////////////////
    use super::*;

    ////////////////////////////////////////////////////////////////////////////////////
    // --- functions ---
    ////////////////////////////////////////////////////////////////////////////////////
    /// I18n for people
    ///
    ///   * **lang_selector** - Language selector
    ///   * _return_ - The string for language
    pub fn i18n_people(lang_selector: LangSelector) -> String {
        LOCALES
            .lookup(
                lang_selector_to_language_id(&lang_selector),
                "persons_grid.people",
            )
            .unwrap_or_default()
    }

    /// I18n for name
    ///
    ///   * **lang_selector** - Language selector
    ///   * _return_ - The string for language
    pub fn i18n_name(lang_selector: LangSelector) -> String {
        LOCALES
            .lookup(
                lang_selector_to_language_id(&lang_selector),
                "persons_grid.name",
            )
            .unwrap_or_default()
    }

    /// I18n for role
    ///
    ///   * **lang_selector** - Language selector
    ///   * _return_ - The string for language
    pub fn i18n_role(lang_selector: LangSelector) -> String {
        LOCALES
            .lookup(
                lang_selector_to_language_id(&lang_selector),
                "persons_grid.role",
            )
            .unwrap_or_default()
    }

    /// I18n for retirement_age
    ///
    ///   * **lang_selector** - Language selector
    ///   * _return_ - The string for language
    pub fn i18n_retirement_age(lang_selector: LangSelector) -> String {
        LOCALES
            .lookup(
                lang_selector_to_language_id(&lang_selector),
                "persons_grid.retirement_age",
            )
            .unwrap_or_default()
    }

    /// I18n for new_person
    ///
    ///   * **lang_selector** - Language selector
    ///   * _return_ - The string for language
    pub fn i18n_new_person(lang_selector: LangSelector) -> String {
        LOCALES
            .lookup(
                lang_selector_to_language_id(&lang_selector),
                "persons_grid.new_person",
            )
            .unwrap_or_default()
    }

    /// I18n for grid_help
    ///
    ///   * **lang_selector** - Language selector
    ///   * _return_ - The string for language
    pub fn i18n_grid_help(lang_selector: LangSelector) -> String {
        LOCALES
            .lookup(
                lang_selector_to_language_id(&lang_selector),
                "persons_grid.grid_help",
            )
            .unwrap_or_default()
    }

    // α <mod-def persons_grid>
    // ω <mod-def persons_grid>
}

/// Functions for i18n strings in SalesPlanView
pub mod sales_plan_view {
    ////////////////////////////////////////////////////////////////////////////////////
    // --- module uses ---
    ////////////////////////////////////////////////////////////////////////////////////
    use super::*;

    ////////////////////////////////////////////////////////////////////////////////////
    // --- functions ---
    ////////////////////////////////////////////////////////////////////////////////////
    /// I18n for name
    ///
    ///   * **lang_selector** - Language selector
    ///   * _return_ - The string for language
    pub fn i18n_name(lang_selector: LangSelector) -> String {
        LOCALES
            .lookup(
                lang_selector_to_language_id(&lang_selector),
                "sales_plan_view.name",
            )
            .unwrap_or_default()
    }

    /// I18n for value
    ///
    ///   * **lang_selector** - Language selector
    ///   * _return_ - The string for language
    pub fn i18n_value(lang_selector: LangSelector) -> String {
        LOCALES
            .lookup(
                lang_selector_to_language_id(&lang_selector),
                "sales_plan_view.value",
            )
            .unwrap_or_default()
    }

    /// I18n for current
    ///
    ///   * **lang_selector** - Language selector
    ///   * _return_ - The string for language
    pub fn i18n_current(lang_selector: LangSelector) -> String {
        LOCALES
            .lookup(
                lang_selector_to_language_id(&lang_selector),
                "sales_plan_view.current",
            )
            .unwrap_or_default()
    }

    /// I18n for target
    ///
    ///   * **lang_selector** - Language selector
    ///   * _return_ - The string for language
    pub fn i18n_target(lang_selector: LangSelector) -> String {
        LOCALES
            .lookup(
                lang_selector_to_language_id(&lang_selector),
                "sales_plan_view.target",
            )
            .unwrap_or_default()
    }

    /// I18n for toward_target
    ///
    ///   * **lang_selector** - Language selector
    ///   * _return_ - The string for language
    pub fn i18n_toward_target(lang_selector: LangSelector) -> String {
        LOCALES
            .lookup(
                lang_selector_to_language_id(&lang_selector),
                "sales_plan_view.toward_target",
            )
            .unwrap_or_default()
    }

    /// I18n for from_target
    ///
    ///   * **lang_selector** - Language selector
    ///   * _return_ - The string for language
    pub fn i18n_from_target(lang_selector: LangSelector) -> String {
        LOCALES
            .lookup(
                lang_selector_to_language_id(&lang_selector),
                "sales_plan_view.from_target",
            )
            .unwrap_or_default()
    }

    /// I18n for total_sales
    ///
    ///   * **lang_selector** - Language selector
    ///   * _return_ - The string for language
    pub fn i18n_total_sales(lang_selector: LangSelector) -> String {
        LOCALES
            .lookup(
                lang_selector_to_language_id(&lang_selector),
                "sales_plan_view.total_sales",
            )
            .unwrap_or_default()
    }

    /// I18n for sales_plan
    ///
    ///   * **lang_selector** - Language selector
    ///   * _return_ - The string for language
    pub fn i18n_sales_plan(lang_selector: LangSelector) -> String {
        LOCALES
            .lookup(
                lang_selector_to_language_id(&lang_selector),
                "sales_plan_view.sales_plan",
            )
            .unwrap_or_default()
    }

    // α <mod-def sales_plan_view>
    // ω <mod-def sales_plan_view>
}

/// Functions for i18n strings in SymbolInput
pub mod symbol_input {
    ////////////////////////////////////////////////////////////////////////////////////
    // --- module uses ---
    ////////////////////////////////////////////////////////////////////////////////////
    use super::*;

    ////////////////////////////////////////////////////////////////////////////////////
    // --- functions ---
    ////////////////////////////////////////////////////////////////////////////////////
    /// I18n for symbol
    ///
    ///   * **lang_selector** - Language selector
    ///   * _return_ - The string for language
    pub fn i18n_symbol(lang_selector: LangSelector) -> String {
        LOCALES
            .lookup(
                lang_selector_to_language_id(&lang_selector),
                "symbol_input.symbol",
            )
            .unwrap_or_default()
    }

    // α <mod-def symbol_input>
    // ω <mod-def symbol_input>
}

/// Functions for i18n strings in TaxDeterminantsComponent
pub mod tax_determinants_component {
    ////////////////////////////////////////////////////////////////////////////////////
    // --- module uses ---
    ////////////////////////////////////////////////////////////////////////////////////
    use super::*;

    ////////////////////////////////////////////////////////////////////////////////////
    // --- functions ---
    ////////////////////////////////////////////////////////////////////////////////////
    /// I18n for simple_effective_rate
    ///
    ///   * **lang_selector** - Language selector
    ///   * _return_ - The string for language
    pub fn i18n_simple_effective_rate(lang_selector: LangSelector) -> String {
        LOCALES
            .lookup(
                lang_selector_to_language_id(&lang_selector),
                "tax_determinants_component.simple_effective_rate",
            )
            .unwrap_or_default()
    }

    /// I18n for by_country
    ///
    ///   * **lang_selector** - Language selector
    ///   * _return_ - The string for language
    pub fn i18n_by_country(lang_selector: LangSelector) -> String {
        LOCALES
            .lookup(
                lang_selector_to_language_id(&lang_selector),
                "tax_determinants_component.by_country",
            )
            .unwrap_or_default()
    }

    /// I18n for tax_determinants
    ///
    ///   * **lang_selector** - Language selector
    ///   * _return_ - The string for language
    pub fn i18n_tax_determinants(lang_selector: LangSelector) -> String {
        LOCALES
            .lookup(
                lang_selector_to_language_id(&lang_selector),
                "tax_determinants_component.tax_determinants",
            )
            .unwrap_or_default()
    }

    // α <mod-def tax_determinants_component>
    // ω <mod-def tax_determinants_component>
}

/// Functions for i18n strings in TimelineGroupComponent
pub mod timeline_group_component {
    ////////////////////////////////////////////////////////////////////////////////////
    // --- module uses ---
    ////////////////////////////////////////////////////////////////////////////////////
    use super::*;

    ////////////////////////////////////////////////////////////////////////////////////
    // --- functions ---
    ////////////////////////////////////////////////////////////////////////////////////
    /// I18n for deterministic
    ///
    ///   * **lang_selector** - Language selector
    ///   * _return_ - The string for language
    pub fn i18n_deterministic(lang_selector: LangSelector) -> String {
        LOCALES
            .lookup(
                lang_selector_to_language_id(&lang_selector),
                "timeline_group_component.deterministic",
            )
            .unwrap_or_default()
    }

    /// I18n for random
    ///
    ///   * **lang_selector** - Language selector
    ///   * _return_ - The string for language
    pub fn i18n_random(lang_selector: LangSelector) -> String {
        LOCALES
            .lookup(
                lang_selector_to_language_id(&lang_selector),
                "timeline_group_component.random",
            )
            .unwrap_or_default()
    }

    /// I18n for year
    ///
    ///   * **lang_selector** - Language selector
    ///   * _return_ - The string for language
    pub fn i18n_year(lang_selector: LangSelector) -> String {
        LOCALES
            .lookup(
                lang_selector_to_language_id(&lang_selector),
                "timeline_group_component.year",
            )
            .unwrap_or_default()
    }

    /// I18n for forecast
    ///
    ///   * **lang_selector** - Language selector
    ///   * _return_ - The string for language
    pub fn i18n_forecast(lang_selector: LangSelector) -> String {
        LOCALES
            .lookup(
                lang_selector_to_language_id(&lang_selector),
                "timeline_group_component.forecast",
            )
            .unwrap_or_default()
    }

    // α <mod-def timeline_group_component>
    // ω <mod-def timeline_group_component>
}

/// Functions for i18n strings in UsTaxStatementView
pub mod us_tax_statement_view {
    ////////////////////////////////////////////////////////////////////////////////////
    // --- module uses ---
    ////////////////////////////////////////////////////////////////////////////////////
    use super::*;

    ////////////////////////////////////////////////////////////////////////////////////
    // --- functions ---
    ////////////////////////////////////////////////////////////////////////////////////
    /// I18n for us_tax_statement
    ///
    ///   * **lang_selector** - Language selector
    ///   * _return_ - The string for language
    pub fn i18n_us_tax_statement(lang_selector: LangSelector) -> String {
        LOCALES
            .lookup(
                lang_selector_to_language_id(&lang_selector),
                "us_tax_statement_view.us_tax_statement",
            )
            .unwrap_or_default()
    }

    /// I18n for us_taxes
    ///
    ///   * **lang_selector** - Language selector
    ///   * _return_ - The string for language
    pub fn i18n_us_taxes(lang_selector: LangSelector) -> String {
        LOCALES
            .lookup(
                lang_selector_to_language_id(&lang_selector),
                "us_tax_statement_view.us_taxes",
            )
            .unwrap_or_default()
    }

    /// I18n for tax_basis
    ///
    ///   * **lang_selector** - Language selector
    ///   * _return_ - The string for language
    pub fn i18n_tax_basis(lang_selector: LangSelector) -> String {
        LOCALES
            .lookup(
                lang_selector_to_language_id(&lang_selector),
                "us_tax_statement_view.tax_basis",
            )
            .unwrap_or_default()
    }

    /// I18n for tax
    ///
    ///   * **lang_selector** - Language selector
    ///   * _return_ - The string for language
    pub fn i18n_tax(lang_selector: LangSelector) -> String {
        LOCALES
            .lookup(
                lang_selector_to_language_id(&lang_selector),
                "us_tax_statement_view.tax",
            )
            .unwrap_or_default()
    }

    /// I18n for earned_income
    ///
    ///   * **lang_selector** - Language selector
    ///   * _return_ - The string for language
    pub fn i18n_earned_income(lang_selector: LangSelector) -> String {
        LOCALES
            .lookup(
                lang_selector_to_language_id(&lang_selector),
                "us_tax_statement_view.earned_income",
            )
            .unwrap_or_default()
    }

    /// I18n for estimate_from_dossier
    ///
    ///   * **lang_selector** - Language selector
    ///   * _return_ - The string for language
    pub fn i18n_estimate_from_dossier(lang_selector: LangSelector) -> String {
        LOCALES
            .lookup(
                lang_selector_to_language_id(&lang_selector),
                "us_tax_statement_view.estimate_from_dossier",
            )
            .unwrap_or_default()
    }

    /// I18n for taxable_distributions
    ///
    ///   * **lang_selector** - Language selector
    ///   * _return_ - The string for language
    pub fn i18n_taxable_distributions(lang_selector: LangSelector) -> String {
        LOCALES
            .lookup(
                lang_selector_to_language_id(&lang_selector),
                "us_tax_statement_view.taxable_distributions",
            )
            .unwrap_or_default()
    }

    /// I18n for qualified_div
    ///
    ///   * **lang_selector** - Language selector
    ///   * _return_ - The string for language
    pub fn i18n_qualified_div(lang_selector: LangSelector) -> String {
        LOCALES
            .lookup(
                lang_selector_to_language_id(&lang_selector),
                "us_tax_statement_view.qualified_div",
            )
            .unwrap_or_default()
    }

    /// I18n for unqualified_div
    ///
    ///   * **lang_selector** - Language selector
    ///   * _return_ - The string for language
    pub fn i18n_unqualified_div(lang_selector: LangSelector) -> String {
        LOCALES
            .lookup(
                lang_selector_to_language_id(&lang_selector),
                "us_tax_statement_view.unqualified_div",
            )
            .unwrap_or_default()
    }

    /// I18n for adjusted_gross_income
    ///
    ///   * **lang_selector** - Language selector
    ///   * _return_ - The string for language
    pub fn i18n_adjusted_gross_income(lang_selector: LangSelector) -> String {
        LOCALES
            .lookup(
                lang_selector_to_language_id(&lang_selector),
                "us_tax_statement_view.adjusted_gross_income",
            )
            .unwrap_or_default()
    }

    /// I18n for capital_gain_distributions
    ///
    ///   * **lang_selector** - Language selector
    ///   * _return_ - The string for language
    pub fn i18n_capital_gain_distributions(lang_selector: LangSelector) -> String {
        LOCALES
            .lookup(
                lang_selector_to_language_id(&lang_selector),
                "us_tax_statement_view.capital_gain_distributions",
            )
            .unwrap_or_default()
    }

    /// I18n for interest
    ///
    ///   * **lang_selector** - Language selector
    ///   * _return_ - The string for language
    pub fn i18n_interest(lang_selector: LangSelector) -> String {
        LOCALES
            .lookup(
                lang_selector_to_language_id(&lang_selector),
                "us_tax_statement_view.interest",
            )
            .unwrap_or_default()
    }

    /// I18n for medicare
    ///
    ///   * **lang_selector** - Language selector
    ///   * _return_ - The string for language
    pub fn i18n_medicare(lang_selector: LangSelector) -> String {
        LOCALES
            .lookup(
                lang_selector_to_language_id(&lang_selector),
                "us_tax_statement_view.medicare",
            )
            .unwrap_or_default()
    }

    /// I18n for other_ordinary_income
    ///
    ///   * **lang_selector** - Language selector
    ///   * _return_ - The string for language
    pub fn i18n_other_ordinary_income(lang_selector: LangSelector) -> String {
        LOCALES
            .lookup(
                lang_selector_to_language_id(&lang_selector),
                "us_tax_statement_view.other_ordinary_income",
            )
            .unwrap_or_default()
    }

    /// I18n for passive_income
    ///
    ///   * **lang_selector** - Language selector
    ///   * _return_ - The string for language
    pub fn i18n_passive_income(lang_selector: LangSelector) -> String {
        LOCALES
            .lookup(
                lang_selector_to_language_id(&lang_selector),
                "us_tax_statement_view.passive_income",
            )
            .unwrap_or_default()
    }

    /// I18n for social_security
    ///
    ///   * **lang_selector** - Language selector
    ///   * _return_ - The string for language
    pub fn i18n_social_security(lang_selector: LangSelector) -> String {
        LOCALES
            .lookup(
                lang_selector_to_language_id(&lang_selector),
                "us_tax_statement_view.social_security",
            )
            .unwrap_or_default()
    }

    /// I18n for standard_deduction
    ///
    ///   * **lang_selector** - Language selector
    ///   * _return_ - The string for language
    pub fn i18n_standard_deduction(lang_selector: LangSelector) -> String {
        LOCALES
            .lookup(
                lang_selector_to_language_id(&lang_selector),
                "us_tax_statement_view.standard_deduction",
            )
            .unwrap_or_default()
    }

    /// I18n for long_term_capital_gain
    ///
    ///   * **lang_selector** - Language selector
    ///   * _return_ - The string for language
    pub fn i18n_long_term_capital_gain(lang_selector: LangSelector) -> String {
        LOCALES
            .lookup(
                lang_selector_to_language_id(&lang_selector),
                "us_tax_statement_view.long_term_capital_gain",
            )
            .unwrap_or_default()
    }

    /// I18n for tax_bill
    ///
    ///   * **lang_selector** - Language selector
    ///   * _return_ - The string for language
    pub fn i18n_tax_bill(lang_selector: LangSelector) -> String {
        LOCALES
            .lookup(
                lang_selector_to_language_id(&lang_selector),
                "us_tax_statement_view.tax_bill",
            )
            .unwrap_or_default()
    }

    // α <mod-def us_tax_statement_view>
    // ω <mod-def us_tax_statement_view>
}

/// Functions for i18n strings in WorthComponent
pub mod worth_component {
    ////////////////////////////////////////////////////////////////////////////////////
    // --- module uses ---
    ////////////////////////////////////////////////////////////////////////////////////
    use super::*;

    ////////////////////////////////////////////////////////////////////////////////////
    // --- functions ---
    ////////////////////////////////////////////////////////////////////////////////////
    /// I18n for worth
    ///
    ///   * **lang_selector** - Language selector
    ///   * _return_ - The string for language
    pub fn i18n_worth(lang_selector: LangSelector) -> String {
        LOCALES
            .lookup(
                lang_selector_to_language_id(&lang_selector),
                "worth_component.worth",
            )
            .unwrap_or_default()
    }

    /// I18n for name
    ///
    ///   * **lang_selector** - Language selector
    ///   * _return_ - The string for language
    pub fn i18n_name(lang_selector: LangSelector) -> String {
        LOCALES
            .lookup(
                lang_selector_to_language_id(&lang_selector),
                "worth_component.name",
            )
            .unwrap_or_default()
    }

    /// I18n for current_value
    ///
    ///   * **lang_selector** - Language selector
    ///   * _return_ - The string for language
    pub fn i18n_current_value(lang_selector: LangSelector) -> String {
        LOCALES
            .lookup(
                lang_selector_to_language_id(&lang_selector),
                "worth_component.current_value",
            )
            .unwrap_or_default()
    }

    /// I18n for cost
    ///
    ///   * **lang_selector** - Language selector
    ///   * _return_ - The string for language
    pub fn i18n_cost(lang_selector: LangSelector) -> String {
        LOCALES
            .lookup(
                lang_selector_to_language_id(&lang_selector),
                "worth_component.cost",
            )
            .unwrap_or_default()
    }

    /// I18n for year_placeholder
    ///
    ///   * **lang_selector** - Language selector
    ///   * _return_ - The string for language
    pub fn i18n_year_placeholder(lang_selector: LangSelector) -> String {
        LOCALES
            .lookup(
                lang_selector_to_language_id(&lang_selector),
                "worth_component.year_placeholder",
            )
            .unwrap_or_default()
    }

    /// I18n for value_placeholder
    ///
    ///   * **lang_selector** - Language selector
    ///   * _return_ - The string for language
    pub fn i18n_value_placeholder(lang_selector: LangSelector) -> String {
        LOCALES
            .lookup(
                lang_selector_to_language_id(&lang_selector),
                "worth_component.value_placeholder",
            )
            .unwrap_or_default()
    }

    // α <mod-def worth_component>
    // ω <mod-def worth_component>
}

/// Functions for i18n strings in WorthsGrid
pub mod worths_grid {
    ////////////////////////////////////////////////////////////////////////////////////
    // --- module uses ---
    ////////////////////////////////////////////////////////////////////////////////////
    use super::*;

    ////////////////////////////////////////////////////////////////////////////////////
    // --- functions ---
    ////////////////////////////////////////////////////////////////////////////////////
    /// I18n for worths
    ///
    ///   * **lang_selector** - Language selector
    ///   * _return_ - The string for language
    pub fn i18n_worths(lang_selector: LangSelector) -> String {
        LOCALES
            .lookup(
                lang_selector_to_language_id(&lang_selector),
                "worths_grid.worths",
            )
            .unwrap_or_default()
    }

    /// I18n for name
    ///
    ///   * **lang_selector** - Language selector
    ///   * _return_ - The string for language
    pub fn i18n_name(lang_selector: LangSelector) -> String {
        LOCALES
            .lookup(
                lang_selector_to_language_id(&lang_selector),
                "worths_grid.name",
            )
            .unwrap_or_default()
    }

    /// I18n for type
    ///
    ///   * **lang_selector** - Language selector
    ///   * _return_ - The string for language
    pub fn i18n_grid_type(lang_selector: LangSelector) -> String {
        LOCALES
            .lookup(
                lang_selector_to_language_id(&lang_selector),
                "worths_grid.type",
            )
            .unwrap_or_default()
    }

    /// I18n for new_worth
    ///
    ///   * **lang_selector** - Language selector
    ///   * _return_ - The string for language
    pub fn i18n_new_worth(lang_selector: LangSelector) -> String {
        LOCALES
            .lookup(
                lang_selector_to_language_id(&lang_selector),
                "worths_grid.new_worth",
            )
            .unwrap_or_default()
    }

    /// I18n for value
    ///
    ///   * **lang_selector** - Language selector
    ///   * _return_ - The string for language
    pub fn i18n_value(lang_selector: LangSelector) -> String {
        LOCALES
            .lookup(
                lang_selector_to_language_id(&lang_selector),
                "worths_grid.value",
            )
            .unwrap_or_default()
    }

    /// I18n for grid_help
    ///
    ///   * **lang_selector** - Language selector
    ///   * _return_ - The string for language
    pub fn i18n_grid_help(lang_selector: LangSelector) -> String {
        LOCALES
            .lookup(
                lang_selector_to_language_id(&lang_selector),
                "worths_grid.grid_help",
            )
            .unwrap_or_default()
    }

    // α <mod-def worths_grid>
    // ω <mod-def worths_grid>
}

/// Functions for i18n strings in YearCurrencyValueInput
pub mod year_currency_value_input {
    ////////////////////////////////////////////////////////////////////////////////////
    // --- module uses ---
    ////////////////////////////////////////////////////////////////////////////////////
    use super::*;

    ////////////////////////////////////////////////////////////////////////////////////
    // --- functions ---
    ////////////////////////////////////////////////////////////////////////////////////
    /// I18n for as_of
    ///
    ///   * **lang_selector** - Language selector
    ///   * _return_ - The string for language
    pub fn i18n_as_of(lang_selector: LangSelector) -> String {
        LOCALES
            .lookup(
                lang_selector_to_language_id(&lang_selector),
                "year_currency_value_input.as_of",
            )
            .unwrap_or_default()
    }

    // α <mod-def year_currency_value_input>
    // ω <mod-def year_currency_value_input>
}

/// Functions for i18n strings in YearRangeInput
pub mod year_range_input {
    ////////////////////////////////////////////////////////////////////////////////////
    // --- module uses ---
    ////////////////////////////////////////////////////////////////////////////////////
    use super::*;

    ////////////////////////////////////////////////////////////////////////////////////
    // --- functions ---
    ////////////////////////////////////////////////////////////////////////////////////
    /// I18n for start_placeholder
    ///
    ///   * **lang_selector** - Language selector
    ///   * _return_ - The string for language
    pub fn i18n_start_placeholder(lang_selector: LangSelector) -> String {
        LOCALES
            .lookup(
                lang_selector_to_language_id(&lang_selector),
                "year_range_input.start_placeholder",
            )
            .unwrap_or_default()
    }

    /// I18n for end_placeholder
    ///
    ///   * **lang_selector** - Language selector
    ///   * _return_ - The string for language
    pub fn i18n_end_placeholder(lang_selector: LangSelector) -> String {
        LOCALES
            .lookup(
                lang_selector_to_language_id(&lang_selector),
                "year_range_input.end_placeholder",
            )
            .unwrap_or_default()
    }

    // α <mod-def year_range_input>
    // ω <mod-def year_range_input>
}

/// Functions for i18n strings in YearValueSeriesComponent
pub mod year_value_series_component {
    ////////////////////////////////////////////////////////////////////////////////////
    // --- module uses ---
    ////////////////////////////////////////////////////////////////////////////////////
    use super::*;

    ////////////////////////////////////////////////////////////////////////////////////
    // --- functions ---
    ////////////////////////////////////////////////////////////////////////////////////
    /// I18n for show_rate_curve
    ///
    ///   * **lang_selector** - Language selector
    ///   * _return_ - The string for language
    pub fn i18n_show_rate_curve(lang_selector: LangSelector) -> String {
        LOCALES
            .lookup(
                lang_selector_to_language_id(&lang_selector),
                "year_value_series_component.show_rate_curve",
            )
            .unwrap_or_default()
    }

    /// I18n for hide_rate_curve
    ///
    ///   * **lang_selector** - Language selector
    ///   * _return_ - The string for language
    pub fn i18n_hide_rate_curve(lang_selector: LangSelector) -> String {
        LOCALES
            .lookup(
                lang_selector_to_language_id(&lang_selector),
                "year_value_series_component.hide_rate_curve",
            )
            .unwrap_or_default()
    }

    /// I18n for rate_placeholder
    ///
    ///   * **lang_selector** - Language selector
    ///   * _return_ - The string for language
    pub fn i18n_rate_placeholder(lang_selector: LangSelector) -> String {
        LOCALES
            .lookup(
                lang_selector_to_language_id(&lang_selector),
                "year_value_series_component.rate_placeholder",
            )
            .unwrap_or_default()
    }

    /// I18n for rate_percent
    ///
    ///   * **lang_selector** - Language selector
    ///   * _return_ - The string for language
    pub fn i18n_rate_percent(lang_selector: LangSelector) -> String {
        LOCALES
            .lookup(
                lang_selector_to_language_id(&lang_selector),
                "year_value_series_component.rate_percent",
            )
            .unwrap_or_default()
    }

    /// I18n for value
    ///
    ///   * **lang_selector** - Language selector
    ///   * _return_ - The string for language
    pub fn i18n_value(lang_selector: LangSelector) -> String {
        LOCALES
            .lookup(
                lang_selector_to_language_id(&lang_selector),
                "year_value_series_component.value",
            )
            .unwrap_or_default()
    }

    /// I18n for year
    ///
    ///   * **lang_selector** - Language selector
    ///   * _return_ - The string for language
    pub fn i18n_year(lang_selector: LangSelector) -> String {
        LOCALES
            .lookup(
                lang_selector_to_language_id(&lang_selector),
                "year_value_series_component.year",
            )
            .unwrap_or_default()
    }

    /// I18n for value_placeholder
    ///
    ///   * **lang_selector** - Language selector
    ///   * _return_ - The string for language
    pub fn i18n_value_placeholder(lang_selector: LangSelector) -> String {
        LOCALES
            .lookup(
                lang_selector_to_language_id(&lang_selector),
                "year_value_series_component.value_placeholder",
            )
            .unwrap_or_default()
    }

    /// I18n for year_placeholder
    ///
    ///   * **lang_selector** - Language selector
    ///   * _return_ - The string for language
    pub fn i18n_year_placeholder(lang_selector: LangSelector) -> String {
        LOCALES
            .lookup(
                lang_selector_to_language_id(&lang_selector),
                "year_value_series_component.year_placeholder",
            )
            .unwrap_or_default()
    }

    // α <mod-def year_value_series_component>
    // ω <mod-def year_value_series_component>
}

// α <mod-def i18n>
// ω <mod-def i18n>
