//! Display implementations

////////////////////////////////////////////////////////////////////////////////////
// --- module uses ---
////////////////////////////////////////////////////////////////////////////////////
use crate::FlowSpec;
use crate::GrowingFlowSpec;
use crate::HoldingLinks;
use crate::ValueFlowSpec;
use core::fmt::Display;
use core::fmt::Formatter;

////////////////////////////////////////////////////////////////////////////////////
// --- trait impls ---
////////////////////////////////////////////////////////////////////////////////////
impl Display for ValueFlowSpec {
    /// Format the instance.
    ///
    ///   * **f** - Formatter to push formatted item to.
    ///   * _return_ - Formatted instance
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        // α <fn Display::fmt for ValueFlowSpec>
        todo!("Implement `fmt`")
        // ω <fn Display::fmt for ValueFlowSpec>
    }
}

impl Display for GrowingFlowSpec {
    /// Format the instance.
    ///
    ///   * **f** - Formatter to push formatted item to.
    ///   * _return_ - Formatted instance
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        // α <fn Display::fmt for GrowingFlowSpec>
        todo!("Implement `fmt`")
        // ω <fn Display::fmt for GrowingFlowSpec>
    }
}

impl Display for HoldingLinks {
    /// Format the instance.
    ///
    ///   * **f** - Formatter to push formatted item to.
    ///   * _return_ - Formatted instance
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        // α <fn Display::fmt for HoldingLinks>
        todo!("Implement `fmt`")
        // ω <fn Display::fmt for HoldingLinks>
    }
}

impl Display for FlowSpec {
    /// Format the instance.
    ///
    ///   * **f** - Formatter to push formatted item to.
    ///   * _return_ - Formatted instance
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        // α <fn Display::fmt for FlowSpec>
        todo!("Implement `fmt`")
        // ω <fn Display::fmt for FlowSpec>
    }
}

// α <mod-def flow_specs_display>
// ω <mod-def flow_specs_display>
