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
    pub fn new(value: T, on_update: impl FnMut(&T) + 'static) -> Self {
        // α <fn Updatable[T]::new>
        leptos_dom::console_log(&format!(
            "Creating Updatable<`{}`>",
            std::any::type_name::<T>()
        ));
        Updatable {
            value,
            on_update: Box::new(on_update),
        }
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
        (self.on_update)(&self.value);
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
        fn update() {
            // α <fn test Updatable[T]::update>

            // To test update we have an updatable that shares data with the block 
            // calling the updates.
            use std::cell::RefCell;
            use std::rc::Rc;

            let updates = Vec::<String>::new();
            let updates = Rc::new(RefCell::new(updates));
            let updates_to_move = Rc::clone(&updates);

            {
                let mut updatable = Updatable::new("The String:".to_string(), move |s| {
                    updates_to_move.borrow_mut().push(s.clone());
                });

                updatable.update(|the_string| the_string.push('1'));
                updatable.update(|the_string| the_string.push('2'));
                updatable.update(|the_string| the_string.push('3'));
            }

            assert_eq!(
                vec![
                    "The String:1".to_string(),
                    "The String:12".to_string(),
                    "The String:123".to_string(),
                    // The extra at the end is the signal on dispose.
                    // TODO: Rethink the approach
                    "The String:123".to_string(),
                ],
                *updates.borrow()
            );
            // ω <fn test Updatable[T]::update>
        }

        // α <mod-def test_updatable_t>
        use super::*;
        // ω <mod-def test_updatable_t>
    }

    // α <mod-def unit_tests>
    use super::*;
    // ω <mod-def unit_tests>
}

// α <mod-def updatable>
// ω <mod-def updatable>
