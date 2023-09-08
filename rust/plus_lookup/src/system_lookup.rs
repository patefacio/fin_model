//! Supports lookup of mappings between various id's and names in the system.    

////////////////////////////////////////////////////////////////////////////////////
// --- module uses ---
////////////////////////////////////////////////////////////////////////////////////
use crate::SystemUnicodes;
use plus_modeled::SystemGrowthId;
use plus_modeled::SystemId;
use std::collections::HashMap;

////////////////////////////////////////////////////////////////////////////////////
// --- type aliases ---
////////////////////////////////////////////////////////////////////////////////////
pub type Lookup = HashMap<u32, String>;
pub type SystemIdLookup = HashMap<SystemId, String>;

////////////////////////////////////////////////////////////////////////////////////
// --- structs ---
////////////////////////////////////////////////////////////////////////////////////
/// Contains the mappings for system lookups.
#[derive(Default)]
pub struct SystemLookup {
    /// Maps `User` ids to name. *Note* Not intended to store all users - just predefined system users.
    pub users: Lookup,
    /// Maps `Outlook` ids to name.
    pub outlooks: Lookup,
    /// Maps `GrowthItem` ids to name.
    pub growth_items: SystemIdLookup,
    /// Maps `Instrument` ids to name.
    pub instruments: SystemIdLookup,
}

////////////////////////////////////////////////////////////////////////////////////
// --- type impls ---
////////////////////////////////////////////////////////////////////////////////////
impl SystemLookup {
    /// Return string with `user` name if found or U(id) otherwise.
    ///
    ///   * **id** - User id to lookup.
    ///   * _return_ - Found `user` or id.
    pub fn lookup_user(&self, id: u32) -> String {
        // α <fn SystemLookup::lookup_user>

        match id {
            0 => SystemUnicodes::PlusauriUser.as_unicode().into(),
            1 => SystemUnicodes::WealthyBarberUser.as_unicode().into(),
            id => self
                .users
                .get(&id)
                .map(|u| format!("{}({})", SystemUnicodes::GeneralUser.as_unicode(), u))
                .unwrap_or(format!(
                    "{}({})",
                    SystemUnicodes::GeneralUser.as_unicode(),
                    id
                )),
        }

        // ω <fn SystemLookup::lookup_user>
    }

    /// Return string with `outlook` name if found or O(id) otherwise.
    ///
    ///   * **id** - Outlook id to lookup.
    ///   * _return_ - Found `outlook` or id.
    pub fn lookup_outlook(&self, id: u32) -> String {
        // α <fn SystemLookup::lookup_outlook>

        match id {
            0 => SystemUnicodes::OutlookStandard.as_unicode().into(),
            1 => SystemUnicodes::OutlookGloomy.as_unicode().into(),
            _ => self
                .outlooks
                .get(&id)
                .map(|o| format!("O({})", o))
                .unwrap_or(format!("O({})", id)),
        }

        // ω <fn SystemLookup::lookup_outlook>
    }

    /// Return string with name of system id and name if not found either G(id) or I(id) for `growth_item` or `instrument`.
    ///
    ///   * **id** - System growth item id to lookup.
    ///   * _return_ - Found `growth item`, `instrument`, or id.
    pub fn lookup_system_growth_item(&self, id: &SystemGrowthId) -> String {
        // α <fn SystemLookup::lookup_system_growth_item>
        if let Some(system_id) = id.system_id {
            self.lookup_category_name(&system_id)
        } else {
            String::from("invalid")
        }
        // ω <fn SystemLookup::lookup_system_growth_item>
    }

    /// Return string with name of system id.
    ///
    ///   * **system_id** - SystemId to lookup.
    ///   * _return_ - Found system id.
    pub fn lookup_category_name(&self, system_id: &SystemId) -> String {
        // α <fn SystemLookup::lookup_category_name>

        match system_id {
            SystemId::WorthItemId(_) => self
                .growth_items
                .get(system_id)
                .map(|gi| format!("{} {}", SystemUnicodes::House.as_unicode(), gi)),
            SystemId::HoldingItemId(_) => self
                .growth_items
                .get(system_id)
                .map(|gi| format!("{} {}", SystemUnicodes::Holding.as_unicode(), gi)),
            SystemId::InstrumentId(_) => self
                .instruments
                .get(system_id)
                .map(|ii| format!("{} {}", SystemUnicodes::Instrument.as_unicode(), ii)),
            SystemId::FlowItemId(_) => self
                .growth_items
                .get(system_id)
                .map(|gi| format!("{} {}", SystemUnicodes::Faucet.as_unicode(), gi)),
        }
        .unwrap_or_else(|| format!("OOPS -> {:?}", system_id))

        // ω <fn SystemLookup::lookup_category_name>
    }
}

/// Unit tests for `system_lookup`
#[cfg(test)]
pub mod unit_tests {

    /// Test type SystemLookup
    mod test_system_lookup {
        ////////////////////////////////////////////////////////////////////////////////////
        // --- module uses ---
        ////////////////////////////////////////////////////////////////////////////////////
        use test_log::test;

        ////////////////////////////////////////////////////////////////////////////////////
        // --- functions ---
        ////////////////////////////////////////////////////////////////////////////////////
        #[test]
        fn lookup_user() {
            // α <fn test SystemLookup::lookup_user>
            assert_eq!(
                SystemUnicodes::PlusauriUser.as_unicode(),
                GENERATED_SYSTEM_LOOKUP.lookup_user(0)
            );
            assert_eq!(
                SystemUnicodes::WealthyBarberUser.as_unicode(),
                GENERATED_SYSTEM_LOOKUP.lookup_user(1)
            );
            // ω <fn test SystemLookup::lookup_user>
        }

        #[test]
        fn lookup_outlook() {
            // α <fn test SystemLookup::lookup_outlook>
            assert_eq!(
                SystemUnicodes::OutlookStandard.as_unicode(),
                GENERATED_SYSTEM_LOOKUP.lookup_outlook(0)
            );
            assert_eq!(
                SystemUnicodes::OutlookGloomy.as_unicode(),
                GENERATED_SYSTEM_LOOKUP.lookup_outlook(1)
            );
            // ω <fn test SystemLookup::lookup_outlook>
        }

        #[test]
        fn lookup_system_growth_item() {
            // α <fn test SystemLookup::lookup_system_growth_item>
            use plus_modeled::WorthType;

            assert_eq!(
                "🏠 ResidentialRealEstate",
                GENERATED_SYSTEM_LOOKUP.lookup_system_growth_item(&SystemGrowthId {
                    system_id: Some(SystemId::WorthItemId(
                        WorthType::ResidentialRealEstate as u32
                    ))
                })
            );

            assert_eq!(
                "🎲 A",
                GENERATED_SYSTEM_LOOKUP.lookup_system_growth_item(&SystemGrowthId {
                    system_id: Some(SystemId::InstrumentId(0))
                })
            );
            // ω <fn test SystemLookup::lookup_system_growth_item>
        }

        #[test]
        fn lookup_category_name() {
            // α <fn test SystemLookup::lookup_category_name>
            use plus_modeled::WorthType;

            assert_eq!(
                format!(
                    "{} ResidentialRealEstate",
                    SystemUnicodes::House.as_unicode()
                ),
                GENERATED_SYSTEM_LOOKUP.lookup_category_name(
                    &(SystemId::WorthItemId(WorthType::ResidentialRealEstate as u32))
                )
            )
            // ω <fn test SystemLookup::lookup_category_name>
        }

        // α <mod-def test_system_lookup>
        use crate::SystemUnicodes;
        use crate::GENERATED_SYSTEM_LOOKUP;

        use plus_modeled::SystemGrowthId;
        use plus_modeled::SystemId;
        // ω <mod-def test_system_lookup>
    }

    // α <mod-def unit_tests>
    // ω <mod-def unit_tests>
}

// α <mod-def system_lookup>
// ω <mod-def system_lookup>
