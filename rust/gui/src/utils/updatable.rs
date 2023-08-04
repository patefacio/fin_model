////////////////////////////////////////////////////////////////////////////////////
// --- module uses ---
////////////////////////////////////////////////////////////////////////////////////
use std::boxed::Box;
use std::fmt::Debug;
use std::ops::AddAssign;

////////////////////////////////////////////////////////////////////////////////////
// --- enums ---
////////////////////////////////////////////////////////////////////////////////////
/// Indicates which field(s) were updated.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum UpdatePairType {
    /// No updates
    UpdateNone,
    /// First value updated
    UpdateFirst,
    /// Second value updated
    UpdateSecond,
    /// Both values updated
    UpdateBoth,
}

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

/// Like [Updatable] but owns a pair of data and supports in-place modification and signalling the update
/// on either or both of the values.
pub struct UpdatablePair<F, S> {
    /// The current value.
    pub first_value: F,
    /// The current value.
    pub second_value: S,
    /// Function called to signal value has been updated.
    on_update: Box<dyn FnMut((&F, &S, UpdatePairType))>,
    /// Current state of the update.
    /// Reset to `UpdatePairType::UpdateNone` on signal.
    pub update_state: UpdatePairType,
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
    pub fn update<U>(&mut self, updater: U)
    where
        U: Fn(&mut T),
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
    pub fn update_and_then_signal<U>(&mut self, updater: U)
    where
        U: Fn(&mut T),
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

impl<F, S> UpdatablePair<F, S>
where
    F: Debug,
    S: Debug,
{
    /// Create new [UpdatablePair].
    ///
    ///   * **first_value** - The first value.
    ///   * **second_value** - The second value.
    ///   * **on_update** - The update callback.
    ///   * _return_ - The new [Updatable].
    pub fn new(
        first_value: F,
        second_value: S,
        on_update: impl FnMut((&F, &S, UpdatePairType)) + 'static,
    ) -> Self {
        // α <fn UpdatablePair[F, S]::new>

        UpdatablePair {
            first_value,
            second_value,
            on_update: Box::new(on_update),
            update_state: UpdatePairType::UpdateNone,
        }

        // ω <fn UpdatablePair[F, S]::new>
    }

    /// Update the value in-place by invoking `updater`.
    ///
    ///   * **updater** - Function responsible for making the update, called with mutable
    /// access to both the item and the shared context and must return the
    /// proper indication of which values were updated.
    #[inline]
    pub fn update<U>(&mut self, updater: U)
    where
        U: Fn((&mut F, &mut S)) -> UpdatePairType,
    {
        // α <fn UpdatablePair[F, S]::update>

        self.update_state += &updater((&mut self.first_value, &mut self.second_value));

        // ω <fn UpdatablePair[F, S]::update>
    }

    /// Update the value in-place by invoking `updater` and then signal the update
    /// by calling `on_update`.
    ///
    ///   * **updater** - Function responsible for making the update.
    #[inline]
    pub fn update_and_then_signal<U>(&mut self, updater: U)
    where
        U: Fn((&mut F, &mut S)) -> UpdatePairType,
    {
        // α <fn UpdatablePair[F, S]::update_and_then_signal>

        self.update(updater);
        self.signal();

        // ω <fn UpdatablePair[F, S]::update_and_then_signal>
    }

    /// Signals the latest value
    #[inline]
    pub fn signal(&mut self) {
        // α <fn UpdatablePair[F, S]::signal>
        (self.on_update)((&self.first_value, &self.second_value, self.update_state));
        self.update_state = UpdatePairType::UpdateNone;
        // ω <fn UpdatablePair[F, S]::signal>
    }
}

////////////////////////////////////////////////////////////////////////////////////
// --- trait impls ---
////////////////////////////////////////////////////////////////////////////////////
impl AddAssign<&Self> for UpdatePairType {
    /// Add `Rhs` to `self`
    ///
    ///   * **rhs** - Right hand side
    fn add_assign(&mut self, rhs: &Self) {
        // α <fn AddAssign::add_assign for UpdatePairType>

        match self {
            UpdatePairType::UpdateFirst => match rhs {
                UpdatePairType::UpdateSecond => *self = UpdatePairType::UpdateBoth,
                _ => (),
            },
            UpdatePairType::UpdateSecond => match rhs {
                UpdatePairType::UpdateFirst => *self = UpdatePairType::UpdateBoth,
                _ => (),
            },
            UpdatePairType::UpdateNone => *self = *rhs,
            _ => (),
        }

        // ω <fn AddAssign::add_assign for UpdatePairType>
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

    /// Test type UpdatablePair<F, S>
    mod test_updatable_pair_fs {
        ////////////////////////////////////////////////////////////////////////////////////
        // --- module uses ---
        ////////////////////////////////////////////////////////////////////////////////////
        use test_log::test;

        ////////////////////////////////////////////////////////////////////////////////////
        // --- functions ---
        ////////////////////////////////////////////////////////////////////////////////////
        #[test]
        fn update() {
            // α <fn test UpdatablePair[F, S]::update>

            let updated_values =
                Rc::new(RefCell::new(Vec::<(String, String, UpdatePairType)>::new()));
            let updates_to_move = Rc::clone(&updated_values);

            {
                let mut updatable = UpdatablePair::new(
                    "First String:".to_string(),
                    "Second String:".to_string(),
                    move |(first, second, update_pair_type)| {
                        updates_to_move.borrow_mut().push((
                            first.clone(),
                            second.clone(),
                            update_pair_type,
                        ));
                    },
                );

                updatable.update(|(first, _)| {
                    first.push('1');
                    UpdatePairType::UpdateFirst
                });
                updatable.update(|(_, second)| {
                    second.push('1');
                    UpdatePairType::UpdateSecond
                });
                updatable.update(|(first, second)| {
                    first.push('2');
                    second.push('2');
                    UpdatePairType::UpdateBoth
                });
                updatable.signal();
            }

            assert_eq!(
                vec![(
                    "First String:12".to_string(),
                    "Second String:12".to_string(),
                    UpdatePairType::UpdateBoth
                )],
                *updated_values.borrow()
            );

            // ω <fn test UpdatablePair[F, S]::update>
        }

        #[test]
        fn update_and_then_signal() {
            // α <fn test UpdatablePair[F, S]::update_and_then_signal>
            let updated_values =
                Rc::new(RefCell::new(Vec::<(String, String, UpdatePairType)>::new()));
            let updates_to_move = Rc::clone(&updated_values);

            {
                let mut updatable = UpdatablePair::new(
                    "First String:".to_string(),
                    "Second String:".to_string(),
                    move |(first, second, update_pair_type)| {
                        updates_to_move.borrow_mut().push((
                            first.clone(),
                            second.clone(),
                            update_pair_type,
                        ));
                    },
                );

                updatable.update_and_then_signal(|(first, _)| {
                    first.push('1');
                    UpdatePairType::UpdateFirst
                });
                updatable.update_and_then_signal(|(_, second)| {
                    second.push('1');
                    UpdatePairType::UpdateSecond
                });
                updatable.update_and_then_signal(|(first, second)| {
                    first.push('2');
                    second.push('2');
                    UpdatePairType::UpdateBoth
                });
            }

            assert_eq!(
                vec![
                    (
                        "First String:1".to_string(),
                        "Second String:".to_string(),
                        UpdatePairType::UpdateFirst
                    ),
                    (
                        "First String:1".to_string(),
                        "Second String:1".to_string(),
                        UpdatePairType::UpdateSecond
                    ),
                    (
                        "First String:12".to_string(),
                        "Second String:12".to_string(),
                        UpdatePairType::UpdateBoth
                    )
                ],
                *updated_values.borrow()
            );
            // ω <fn test UpdatablePair[F, S]::update_and_then_signal>
        }

        #[test]
        fn signal() {
            // α <fn test UpdatablePair[F, S]::signal>

            let updated_values =
                Rc::new(RefCell::new(Vec::<(String, String, UpdatePairType)>::new()));
            let updates_to_move = Rc::clone(&updated_values);

            {
                let mut updatable = UpdatablePair::new(
                    "First String:".to_string(),
                    "Second String:".to_string(),
                    move |(first, second, update_pair_type)| {
                        updates_to_move.borrow_mut().push((
                            first.clone(),
                            second.clone(),
                            update_pair_type,
                        ));
                    },
                );

                updatable.signal();
                updatable.signal();
                updatable.signal();
            }

            assert_eq!(
                vec![
                    (
                        "First String:".to_string(),
                        "Second String:".to_string(),
                        UpdatePairType::UpdateNone
                    ),
                    (
                        "First String:".to_string(),
                        "Second String:".to_string(),
                        UpdatePairType::UpdateNone
                    ),
                    (
                        "First String:".to_string(),
                        "Second String:".to_string(),
                        UpdatePairType::UpdateNone
                    )
                ],
                *updated_values.borrow()
            );

            // ω <fn test UpdatablePair[F, S]::signal>
        }

        // α <mod-def test_updatable_pair_fs>
        use super::*;
        use std::cell::RefCell;
        use std::rc::Rc;
        // ω <mod-def test_updatable_pair_fs>
    }

    // α <mod-def unit_tests>
    use super::*;
    // ω <mod-def unit_tests>
}

// α <mod-def updatable>
// ω <mod-def updatable>
