//! Display implementations

////////////////////////////////////////////////////////////////////////////////////
// --- module uses ---
////////////////////////////////////////////////////////////////////////////////////
use crate::Person;
use core::fmt::Display;
use core::fmt::Formatter;

////////////////////////////////////////////////////////////////////////////////////
// --- trait impls ---
////////////////////////////////////////////////////////////////////////////////////
impl Display for Person {
    /// Format the instance.
    ///
    ///   * **f** - Formatter to push formatted item to.
    ///   * _return_ - Formatted instance
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        // α <fn Display::fmt for Person>
        use crate::PersonType;
        write!(
            f,
            "`{}`({})",
            self.name,
            PersonType::from_i32(self.person_type)
                .unwrap()
                .as_str_name()
        )
        // ω <fn Display::fmt for Person>
    }
}

// α <mod-def person_display>
// ω <mod-def person_display>
