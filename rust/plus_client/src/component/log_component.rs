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

            tracing::info!(concat!("Initializing: ", $component, "({})"), count);
            leptos::on_cleanup(move || {
                tracing::info!(concat!("Cleanup:", $component, "({})"), count)
            });
        }
        // ω <log_component(component:expr)>
    };
}
pub(crate) use log_component;

// α <mod-def log_component>
// ω <mod-def log_component>
