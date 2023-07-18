//! Functions to assist in usage of leptos

////////////////////////////////////////////////////////////////////////////////////
// --- module uses ---
////////////////////////////////////////////////////////////////////////////////////
use std::cell::Ref;
use std::cell::RefCell;
use std::cell::RefMut;
use std::rc::Rc;

////////////////////////////////////////////////////////////////////////////////////
// --- type aliases ---
////////////////////////////////////////////////////////////////////////////////////
type Sharable<T> = Rc<RefCell<T>>;

////////////////////////////////////////////////////////////////////////////////////
// --- traits ---
////////////////////////////////////////////////////////////////////////////////////
/// Preferred way to share data across components - simply wraps Rc<RefCell<...>>
pub trait SharedData {
    /// The shared data
    type T;

    /// Get sharable of the data
    ///
    ///   * _return_ - Reference to the data.
    fn shallow_clone(&self) -> Sharable<Self::T>;

    /// Borrow the data mutably
    ///
    ///   * _return_ - Reference to the data.
    fn borrow_mut(&self) -> RefMut<'_, Self::T>;
}

////////////////////////////////////////////////////////////////////////////////////
// --- functions ---
////////////////////////////////////////////////////////////////////////////////////
/// Create an Rc<RefCell<T>>
///
///   * **t** - Data to make shareable
///   * _return_ - T wrapped in [RefCell] (for interior mutability) and [Rc] for breaking borrow chains
#[inline]
pub fn make_shared_data<T>(t: T) -> Sharable<T> {
    // α <fn make_shared_data>
    Rc::new(RefCell::new(t))
    // ω <fn make_shared_data>
}

////////////////////////////////////////////////////////////////////////////////////
// --- trait impls ---
////////////////////////////////////////////////////////////////////////////////////
impl<T> SharedData for Sharable<T> {
    type T = T;
    /// Get sharable of the data
    ///
    ///   * _return_ - Reference to the data.
    fn shallow_clone(&self) -> Sharable<Self::T> {
        // α <fn SharedData::shallow_clone for Sharable<T>>
        Rc::clone(self)
        // ω <fn SharedData::shallow_clone for Sharable<T>>
    }

    /// Borrow the data mutably
    ///
    ///   * _return_ - Reference to the data.
    fn borrow_mut(&self) -> RefMut<'_, Self::T> {
        // α <fn SharedData::borrow_mut for Sharable<T>>
        self.as_ref().borrow_mut()
        // ω <fn SharedData::borrow_mut for Sharable<T>>
    }
}

/// Unit tests for `leptos_helpers`
#[cfg(test)]
pub mod unit_tests {

    /// Test trait shared_data on Sharable<T>
    pub mod test_shared_data_on_sharable_t {
        ////////////////////////////////////////////////////////////////////////////////////
        // --- module uses ---
        ////////////////////////////////////////////////////////////////////////////////////
        use test_log::test;

        ////////////////////////////////////////////////////////////////////////////////////
        // --- functions ---
        ////////////////////////////////////////////////////////////////////////////////////
        #[test]
        fn sharable() {
            // α <fn test SharedData::sharable on Sharable[T]>
            use crate::Updatable;
            use leptos::create_runtime;
            use leptos::create_scope;
            use leptos::log;
            use leptos::run_scope;
            use leptos::store_value;
    
            #[derive(Debug)]
            struct S {
                t: String,
            }
    
            impl S {
                fn new(t: &str) -> Self {
                    println!("Creating new s with {t}");
                    S {
                        t: t.to_string(),
                    }
                }
            }
    
            fn user_callback(s: &mut S) {
                let runtime = create_runtime();
                println!("Callback exclusively using {s:?}");
                run_scope(runtime, |cx| {
                    let s = store_value(cx, true);
                })
            }
    
            impl Drop for S {
                fn drop(&mut self) {
                    println!("Dropping s");
                }
            }
    
            fn make_updatable(t: &str) -> Updatable<S> {
                Updatable::new(S::new(t), |s| println!("Updating {s:?}"))
            }
    
            fn should_panic() {
                let updatable = make_updatable("should panic");
                let runtime = create_runtime();
                run_scope(runtime, |cx| {
                    let value = store_value(cx, make_shared_data(updatable));
                    value.update_value(|value| {
                        // do updates on S
                        user_callback(&mut value.as_ref().borrow_mut().value);
                    })
                })
            }
    
            fn should_not_panic() {
                let updatable = make_updatable("should not panic");
                let runtime = create_runtime();
                run_scope(runtime, |cx| {
                    let ref_counted = store_value(cx, make_shared_data(updatable));
                    ref_counted.update_value(|_| {
                        // do updates on S
                    });
                    let value = ref_counted.with_value(|v| v.shallow_clone());
                    user_callback(&mut value.as_ref().borrow_mut().value);
                })
            }
    
            // This panics
            assert!(std::panic::catch_unwind(should_panic).is_err());
            should_not_panic();
    
            println!("DONE RUNNING");
    
            // ω <fn test SharedData::sharable on Sharable[T]>
        }

        // α <mod-def test_shared_data_on_sharable_t>
        use super::*;
        // ω <mod-def test_shared_data_on_sharable_t>
    }

    // α <mod-def unit_tests>
    use super::*;
    // ω <mod-def unit_tests>
}

// α <mod-def leptos_helpers>
// ω <mod-def leptos_helpers>
