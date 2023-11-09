//! Macro to log construction/destruction of components

////////////////////////////////////////////////////////////////////////////////////
// --- declarative macros ---
////////////////////////////////////////////////////////////////////////////////////
/// Log the construction/cleanup of the component
#[macro_export]
macro_rules! log_component {
    ($component:expr) => {
        // α <log_component(component:expr)>
        #[cfg(debug_assertions)]
        {
            let count = unsafe {
                static mut COUNTER: u32 = 0;
                COUNTER += 1;
                COUNTER
            };

            tracing::warn!(concat!("Initializing: ", $component, "({})"), count);
            leptos::on_cleanup(move || {
                tracing::warn!(concat!("Cleanup:", $component, "({})"), count)
            });
        }
        // ω <log_component(component:expr)>
    };
}
#[allow(unused_imports)]
pub(crate) use log_component;

// α <mod-def log_component>
// ω <mod-def log_component>
