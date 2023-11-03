//! TODO: Document Module(svg)

////////////////////////////////////////////////////////////////////////////////////
// --- structs ---
////////////////////////////////////////////////////////////////////////////////////
/// Basic point class for svg
#[derive(Debug, Copy, Clone, Default)]
pub struct SvgPoint {
    /// X value
    pub x: f64,
    /// Y value
    pub y: f64,
}

/// Basic dimension class svg
#[derive(Debug, Copy, Clone, Default)]
pub struct SvgDim {
    /// Width
    pub width: f64,
    /// Height
    pub height: f64,
}

/// Represents a rectangular area for drawing content
#[derive(Debug, Copy, Clone, Default)]
pub struct SvgArea {
    /// Origin of the area
    pub origin: SvgPoint,
    /// Dimensions of the area
    pub dim: SvgDim,
}

////////////////////////////////////////////////////////////////////////////////////
// --- type impls ---
////////////////////////////////////////////////////////////////////////////////////
impl SvgPoint {
    /// Basic point initialization
    ///
    ///   * **x** - X value
    ///   * **y** - Y value
    ///   * _return_ - The constructed instance
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
}

impl SvgDim {
    /// Basic dimension initialization
    ///
    ///   * **width** - Width
    ///   * **height** - Height
    ///   * _return_ - The constructed instance
    pub fn new(width: f64, height: f64) -> Self {
        Self { width, height }
    }
}

impl SvgArea {
    /// Basic area initialization
    ///
    ///   * **origin** - Origin of the area
    ///   * **dim** - Dimensions of the area
    ///   * _return_ - The constructed instance
    pub fn new(origin: SvgPoint, dim: SvgDim) -> Self {
        Self { origin, dim }
    }
}

// α <mod-def svg>
// ω <mod-def svg>
