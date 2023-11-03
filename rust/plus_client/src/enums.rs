//! Enums shared by components

////////////////////////////////////////////////////////////////////////////////////
// --- enums ---
////////////////////////////////////////////////////////////////////////////////////
/// Indicates direction to flow the select items: can be displayed left-to-right or top-to-bottom.
#[derive(Debug, Copy, Clone)]
pub enum SelectDirection {
    /// Fill in values row by row.
    LeftToRight,
    /// Fill in values column by column.
    TopToBottom,
}

/// Indicates side of view that items may be located.
#[derive(Debug, Copy, Clone)]
pub enum ViewSide {
    /// View positioned at top.
    Top,
    /// View positioned at bottom.
    Bottom,
    /// View positioned at left.
    Left,
    /// View positioned at right.
    Right,
}

// α <mod-def enums>
// ω <mod-def enums>
