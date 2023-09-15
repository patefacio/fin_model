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
/// Updates can be done without signaling via `update`.
/// Updates can be done with signalling via `update_and_then_signal`.
/// Signal without update via `signal`.
///
/// Prefer `update_and_then_signal` for simple live components like
/// `NumericInput`. For more complex components, update model with
/// update as changes come in and then on some completion event (e.g.
/// press of Ok button) signal complete/final model.
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
        Updatable {
            value,
            on_update: Box::new(on_update),
        }
        // ω <fn Updatable[T]::new>
    }

    /// Update the value in-place by invoking `updater`.
    ///
    ///   * **updater** - Function responsible for making the update.
    #[inline]
    pub fn update<U>(&mut self, mut updater: U)
    where
        U: FnMut(&mut T),
    {
        // α <fn Updatable[T]::update>
        updater(&mut self.value);
        // ω <fn Updatable[T]::update>
    }

    /// Update the value in-place by invoking `updater` and then signal the update
    /// by calling `on_update`.
    ///
    ///   * **updater** - Function responsible for making the update.
    #[inline]
    pub fn update_and_then_signal<U>(&mut self, mut updater: U)
    where
        U: FnMut(&mut T),
    {
        // α <fn Updatable[T]::update_and_then_signal>
        updater(&mut self.value);
        self.signal();
        // ω <fn Updatable[T]::update_and_then_signal>
    }

    /// Signals the latest value
    #[inline]
    pub fn signal(&mut self) {
        // α <fn Updatable[T]::signal>
        (self.on_update)(&self.value);
        // ω <fn Updatable[T]::signal>
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
                updatable.signal();
            }

            assert_eq!(vec!["The String:123".to_string(),], *updates.borrow());
            // ω <fn test Updatable[T]::update>
        }

        #[test]
        fn update_and_then_signal() {
            // α <fn test Updatable[T]::update_and_then_signal>
            // To test update we have an updatable that shares data with the block
            // calling the updates.
            let updates = Vec::<String>::new();
            let updates = Rc::new(RefCell::new(updates));
            let updates_to_move = Rc::clone(&updates);

            {
                let mut updatable = Updatable::new("The String:".to_string(), move |s| {
                    updates_to_move.borrow_mut().push(s.clone());
                });

                updatable.update_and_then_signal(|the_string| the_string.push('1'));
                updatable.update_and_then_signal(|the_string| the_string.push('2'));
                updatable.update_and_then_signal(|the_string| the_string.push('3'));
            }

            assert_eq!(
                vec![
                    "The String:1".to_string(),
                    "The String:12".to_string(),
                    "The String:123".to_string(),
                ],
                *updates.borrow()
            );
            // ω <fn test Updatable[T]::update_and_then_signal>
        }

        #[test]
        fn signal() {
            // α <fn test Updatable[T]::signal>

            let count = Rc::new(RefCell::new(0));
            let closure_count = Rc::clone(&count);
            let mut updatable = Updatable::new(Some(42), move |&value| {
                assert_eq!(Some(42), value);
                *closure_count.borrow_mut() += 1;
            });
            updatable.signal();
            updatable.signal();
            updatable.signal();
            assert_eq!(3, *count.as_ref().borrow());

            // ω <fn test Updatable[T]::signal>
        }

        // α <mod-def test_updatable_t>
        use super::*;
        use std::cell::RefCell;
        use std::rc::Rc;
        // ω <mod-def test_updatable_t>
    }

    // α <mod-def unit_tests>
    use super::*;
    // ω <mod-def unit_tests>
}

// α <mod-def updatable>
// ω <mod-def updatable>
