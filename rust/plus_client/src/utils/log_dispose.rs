//! Support for logging when items get cleaned up.
//! Potentially helpful for ensuring items are getting cleaned up.

////////////////////////////////////////////////////////////////////////////////////
// --- module uses ---
////////////////////////////////////////////////////////////////////////////////////
use cfg_if::cfg_if;

////////////////////////////////////////////////////////////////////////////////////
// --- structs ---
////////////////////////////////////////////////////////////////////////////////////
/// Simply logs when disposed, for development to confirm cleanup.
#[derive(Debug, Clone)]
pub struct LogDispose {
    /// Identifier for item
    pub id: String,
}

////////////////////////////////////////////////////////////////////////////////////
// --- type impls ---
////////////////////////////////////////////////////////////////////////////////////
impl LogDispose {
    /// Create new instance of LogDispose
    ///
    ///   * **id** - Identifier for item
    ///   * _return_ - The new instance
    pub fn new(id: String) -> LogDispose {
        LogDispose { id }
    }
}

////////////////////////////////////////////////////////////////////////////////////
// --- trait impls ---
////////////////////////////////////////////////////////////////////////////////////
impl Drop for LogDispose {
    /// Custom code within the destructor
    fn drop(&mut self) {
        // α <fn Drop::drop for LogDispose>

        cfg_if! {
            // server-only stuff
            if #[cfg(feature = "csr")] {
                leptos_dom::console_log(&format!("Dispose(`{}`)", self.id));
            } else {
                log::info!("Dispose(`{}`)", self.id);
            }
        }

        // ω <fn Drop::drop for LogDispose>
    }
}

// α <mod-def log_dispose>
// ω <mod-def log_dispose>
