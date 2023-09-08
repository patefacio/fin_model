//! TODO: Document Module(constants)

////////////////////////////////////////////////////////////////////////////////////
// --- constants ---
////////////////////////////////////////////////////////////////////////////////////
/// Escape - often used to leave a dialog without any effects
pub const ESCAPE_KEY: u32 = 27;
/// Enter - finalize some action
pub const ENTER_KEY: u32 = 13;
/// Delete key
pub const DELETE_KEY: u32 = 127;
/// Backspace key
pub const BACKSPACE_KEY: u32 = 8;
/// Space character and in some contexts the same as `Enter`
pub const SPACE_KEY: u32 = 32;
/// Move to next tab item
pub const TAB_KEY: u32 = 9;
/// Move left
pub const LEFT_KEY: u32 = 37;
/// Move left
pub const RIGHT_KEY: u32 = 39;
/// Move left
pub const UP_KEY: u32 = 38;
/// Move left
pub const DOWN_KEY: u32 = 40;

// α <mod-def constants>

use once_cell::sync::Lazy;
use plotters::prelude::*;

pub const PLOT_TEXT_STYLE: Lazy<TextStyle> = Lazy::new(|| ("sans-serif", 18).into());

// ω <mod-def constants>
