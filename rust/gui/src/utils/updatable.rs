////////////////////////////////////////////////////////////////////////////////////
// --- module uses ---
////////////////////////////////////////////////////////////////////////////////////
use std::cell::RefCell;
use std::fmt::Debug;

////////////////////////////////////////////////////////////////////////////////////
// --- structs ---
////////////////////////////////////////////////////////////////////////////////////
/// Owns a piece of data and supports updating it and calling a function when updated.
pub struct Updatable<T, F> {
    /// The current value.
    pub value: RefCell<T>,
    /// Indicates value has been modified
    pub on_update: F,
}

////////////////////////////////////////////////////////////////////////////////////
// --- type impls ---
////////////////////////////////////////////////////////////////////////////////////
impl<T, F> Updatable<T, F>
where
    T: Debug,
    F: Fn(&T) + 'static,
{
    /// Create new [Updatable].
    ///
    ///   * **initial_value** - The initial value.
    ///   * **on_update** - The update callback.
    ///   * _return_ - The new [Updatable].
    pub fn new(initial_value: T, on_update: F) -> Self {
        // α <fn Updatable[T, F]::new>
        Updatable { value: RefCell::new(initial_value), on_update }
        // ω <fn Updatable[T, F]::new>
    }

    /// Update with new value.
    ///
    ///   * **value** - New value.
    pub fn update(&self, value: T) {
        // α <fn Updatable[T, F]::update>
        leptos_dom::console_log(&format!("Updating to {value:?}"));
        (self.on_update)(&value);
        self.value.replace(value);
        // ω <fn Updatable[T, F]::update>
    }
}

/// Unit tests for `updatable`
#[cfg(test)]
pub mod unit_tests {

    /// Test type Updatable<T, F>
    mod test_updatable_tf {
        ////////////////////////////////////////////////////////////////////////////////////
        // --- module uses ---
        ////////////////////////////////////////////////////////////////////////////////////
        use test_log::test;

        ////////////////////////////////////////////////////////////////////////////////////
        // --- functions ---
        ////////////////////////////////////////////////////////////////////////////////////
        #[test]
        fn new() {
            // α <fn test Updatable[T, F]::new>
            todo!("Test new")
            // ω <fn test Updatable[T, F]::new>
        }

        #[test]
        fn update() {
            // α <fn test Updatable[T, F]::update>
            todo!("Test update")
            // ω <fn test Updatable[T, F]::update>
        }

        // α <mod-def test_updatable_tf>
        // ω <mod-def test_updatable_tf>
    }

    // α <mod-def unit_tests>
    // ω <mod-def unit_tests>
}

// α <mod-def updatable>
// ω <mod-def updatable>
