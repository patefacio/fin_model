//! Basic RAII support for timing blocks of code

////////////////////////////////////////////////////////////////////////////////////
// --- module uses ---
////////////////////////////////////////////////////////////////////////////////////
use cfg_if::cfg_if;

// α <mod-def block_time>
use leptos_dom::tracing;

cfg_if! {

    if #[cfg(any(feature = "csr", feature = "hydrate"))] {

        /// Tracks when a block with `label` is entered in order to time on exit
        pub struct BlockTime {
            label: String,
            start: f64
        }


        impl BlockTime {
            /// Create new time for block with `label`
            pub fn new(label: &str) -> BlockTime {
                BlockTime {
                    label: label.into(),
                    start: instant::now(),
                }
            }
        }

        impl Drop for BlockTime {
            fn drop(&mut self) {
                let duration = instant::now() - self.start;
                let message = format!("`{}`: Duration {:?}ms", self.label, duration);
                tracing::info!("{message}");
            }
        }
    } else {

        /// Tracks when a block with `label` is entered in order to time on exit
        pub struct BlockTime {
            label: String,
            start: instant::Instant
        }

        impl BlockTime {
            /// Create new time for block with `label`
            pub fn new(label: &str) -> BlockTime {
                BlockTime {
                    label: label.into(),
                    start: instant::Instant::now(),
                }
            }
        }

        impl Drop for BlockTime {
            fn drop(&mut self) {
                let duration = self.start.elapsed();
                let message = format!("`{}`: Duration {:?}", self.label, duration);
                tracing::info!("{message}");
            }
        }
    }
}

// ω <mod-def block_time>
