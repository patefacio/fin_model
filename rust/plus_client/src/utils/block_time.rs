//! Basic RAII support for timing blocks of code

////////////////////////////////////////////////////////////////////////////////////
// --- module uses ---
////////////////////////////////////////////////////////////////////////////////////
use cfg_if::cfg_if;

// α <mod-def block_time>

use leptos_dom::log;

cfg_if! {
    // server-only stuff
    if #[cfg(feature = "ssr")] {
        use std::time::{Duration, Instant};

        /// Tracks when a block with `label` is entered in order to time on exit
        pub struct BlockTime {
            label: String,
            start: Instant
        }

    } else {

        /// Tracks when a block with `label` is entered in order to time on exit
        pub struct BlockTime {
            label: String,
            start: f64
        }
    }
}

cfg_if! {
    if #[cfg(not(feature = "ssr"))] {
        fn web_now() -> f64 {
            web_sys::window()
                .expect("should have a Window")
                .performance()
                .expect("should have a Performance")
                .now()
        }
    }
}

impl BlockTime {
    cfg_if! {

        
        if #[cfg(feature = "ssr")] {
            /// Create new time for block with `label`
            pub fn new(label: &str) -> BlockTime {
                BlockTime { label: label.into(), start: Instant::now() }
            }
        } else {

            /// Create new time for block with `label`
            pub fn new(label: &str) -> BlockTime {
                log!("Open block `{}`", label);
                BlockTime { label: label.into(), start: web_now() }
            }
        }
    }
}

impl Drop for BlockTime {
    cfg_if! {
        if #[cfg(feature = "ssr")] {

        fn drop(&mut self) {
            let duration = self.start.elapsed();
            let message = format!("`{}`: Duration {:?}", self.label, duration);
            println!("{}", message);
        }

        } else {

            fn drop(&mut self) {
                let duration = web_now() - self.start;
                let message = format!("Close `{}`: Duration {:?} millis", self.label, duration);
                log!("{}", message);
            }

        }
    }
}

// ω <mod-def block_time>
