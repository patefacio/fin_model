////////////////////////////////////////////////////////////////////////////////////
// --- module uses ---
////////////////////////////////////////////////////////////////////////////////////
use std::boxed::Box;
use std::fmt::Debug;

////////////////////////////////////////////////////////////////////////////////////
// --- structs ---
////////////////////////////////////////////////////////////////////////////////////
/// Owns a piece of data and supports in-place modification and signalling the update
/// by calling provided `on_update`.
pub struct Updatable<T> {
    /// The current value.
    pub value: T,
    /// Function called to signal value has been updated.
    on_update: Box<dyn FnMut(&T)>,
}

////////////////////////////////////////////////////////////////////////////////////
// --- type impls ---
////////////////////////////////////////////////////////////////////////////////////
impl<T> Updatable<T>
where
    T: Debug,
{
    /// Create new [Updatable].
    ///
    ///   * **value** - The initial value.
    ///   * **on_update** - The update callback.
    ///   * _return_ - The new [Updatable].
    pub fn new(value: T, on_update: Box<dyn FnMut(&T)>) -> Self {
        // α <fn Updatable[T]::new>
        leptos_dom::console_log(&format!(
            "Creating Updatable<`{}`>",
            std::any::type_name::<T>()
        ));
        Updatable { value, on_update }
        // ω <fn Updatable[T]::new>
    }

    /// Update the value in-place by invoking `updater` and then signal the update
    /// by calling `on_update`.
    ///
    ///   * **updater** - Function responsible for making the update.
    pub fn update<U>(&mut self, updater: U)
    where
        U: Fn(&mut T),
    {
        // α <fn Updatable[T]::update>
        leptos_dom::console_log(&format!("Updating to {:?}", self.value));
        updater(&mut self.value);
        (self.on_update)(&self.value);
        // ω <fn Updatable[T]::update>
    }
}

////////////////////////////////////////////////////////////////////////////////////
// --- trait impls ---
////////////////////////////////////////////////////////////////////////////////////
impl<T> Drop for Updatable<T> {
    /// Custom code within the destructor
    fn drop(&mut self) {
        // α <fn Drop::drop for Updatable<T>>
        leptos_dom::console_log(&format!(
            "Dropping Updatable<`{}`>",
            std::any::type_name::<T>()
        ));
        // ω <fn Drop::drop for Updatable<T>>
    }
}

/// Unit tests for `updatable`
#[cfg(test)]
pub mod unit_tests {

    /// Test type Updatable<T>
    mod test_updatable_t {
        ////////////////////////////////////////////////////////////////////////////////////
        // --- module uses ---
        ////////////////////////////////////////////////////////////////////////////////////
        use test_log::test;

        ////////////////////////////////////////////////////////////////////////////////////
        // --- functions ---
        ////////////////////////////////////////////////////////////////////////////////////
        #[test]
        fn new() {
            // α <fn test Updatable[T]::new>
            todo!("Test new")
            // ω <fn test Updatable[T]::new>
        }

        #[test]
        fn update() {
            // α <fn test Updatable[T]::update>
            todo!("Test update")
            // ω <fn test Updatable[T]::update>
        }

        // α <mod-def test_updatable_t>
        // ω <mod-def test_updatable_t>
    }

    // α <mod-def unit_tests>
    // ω <mod-def unit_tests>
}

// α <mod-def updatable>
// ω <mod-def updatable>
