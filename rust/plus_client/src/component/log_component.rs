//! Macro to log construction/destruction of components

////////////////////////////////////////////////////////////////////////////////////
// --- declarative macros ---
////////////////////////////////////////////////////////////////////////////////////
/// Return a unique component id based on component_name
#[macro_export]
macro_rules! component_id {
    ($component_name:expr) => {
        // α <component_id(component_name:expr)>
        {
            let count = unsafe {
                static mut COUNTER: u32 = 0;
                COUNTER += 1;
                COUNTER
            };
            format!("{}({count})", $component_name)
        }
        // ω <component_id(component_name:expr)>
    };
}
#[allow(unused_imports)]
pub(crate) use component_id;
/// Log the construction/cleanup of the component
#[macro_export]
macro_rules! log_component {
    ($level:expr, $component_id:expr) => {
        // α <log_component(level:expr:component_id:expr)>
        use tracing::Level;
        match $level {
            Level::TRACE => {
                tracing::trace!("Initializing: {}", $component_id);
                leptos::on_cleanup(move || tracing::trace!("Cleanup: {}", $component_id));
            }
            Level::DEBUG => {
                tracing::debug!("Initializing: {}", $component_id);
                leptos::on_cleanup(move || tracing::debug!("Cleanup: {}", $component_id));
            }
            Level::INFO => {
                tracing::info!("Initializing: {}", $component_id);
                leptos::on_cleanup(move || tracing::info!("Cleanup: {}", $component_id));
            }
            Level::WARN => {
                tracing::warn!("Initializing: {}", $component_id);
                leptos::on_cleanup(move || tracing::warn!("Cleanup: {}", $component_id));
            }
            Level::ERROR => {
                tracing::error!("Initializing: {}", $component_id);
                leptos::on_cleanup(move || tracing::error!("Cleanup: {}", $component_id));
            }
        }
        // ω <log_component(level:expr:component_id:expr)>
    };
}
#[allow(unused_imports)]
pub(crate) use log_component;

////////////////////////////////////////////////////////////////////////////////////
// --- constants ---
////////////////////////////////////////////////////////////////////////////////////
/// Default log level for component logging
pub const COMPONENT_LOG_LEVEL: tracing::Level = tracing::Level::WARN;

// α <mod-def log_component>
// ω <mod-def log_component>
