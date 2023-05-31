////////////////////////////////////////////////////////////////////////////////////
// --- module uses ---
////////////////////////////////////////////////////////////////////////////////////
use std::fmt::Debug;

////////////////////////////////////////////////////////////////////////////////////
// --- structs ---
////////////////////////////////////////////////////////////////////////////////////
/// Owns a piece of data and supports in-place modification and signalling the update
/// by calling provided `on_update`. Parameterized by `T`, the type for the data to
/// store and `F` the signalling update function.
pub struct Updatable<T, F> {
    /// The current value.
    pub value: T,
    /// Indicates value has been modified
    pub on_update: F,
}

////////////////////////////////////////////////////////////////////////////////////
// --- type impls ---
////////////////////////////////////////////////////////////////////////////////////
impl<T, F> Updatable<T, F>
where
    T: Debug,
    F: FnMut(&T),
{
    /// Create new [Updatable].
    ///
    ///   * **value** - The initial value.
    ///   * **on_update** - The update callback.
    ///   * _return_ - The new [Updatable].
    pub fn new(value: T, on_update: F) -> Self {
        // α <fn Updatable[T, F]::new>
        Updatable { value, on_update }
        // ω <fn Updatable[T, F]::new>
    }

    /// Update the value in-place by invoking `updater` and then signal the update
    /// by calling `on_update`.
    ///
    ///   * **updater** - Function responsible for making the update.
    pub fn update<U>(&mut self, updater: U)
    where
        U: Fn(&mut T),
    {
        // α <fn Updatable[T, F]::update>
        leptos_dom::console_log(&format!("Updating to {:?}", self.value));
        updater(&mut self.value);
        (self.on_update)(&self.value);
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
