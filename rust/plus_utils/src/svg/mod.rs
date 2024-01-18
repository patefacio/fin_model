//! TODO: Document Module(svg)

////////////////////////////////////////////////////////////////////////////////////
// --- mod decls ---
////////////////////////////////////////////////////////////////////////////////////
pub mod histogram;

////////////////////////////////////////////////////////////////////////////////////
// --- enums ---
////////////////////////////////////////////////////////////////////////////////////
/// Define a color
#[derive(Debug, Clone, Default)]
pub enum SvgColor {
    /// Rgba definition
    Rgba {
        /// The rgba definition
        color: RgbaColor,
    },
    /// Rgb definition
    Rgb {
        /// The rgb definition
        color: RgbColor,
    },
    /// The color `alice_blue`
    AliceBlue,
    /// The color `antique_white`
    AntiqueWhite,
    /// The color `aqua`
    Aqua,
    /// The color `aquamarine`
    Aquamarine,
    /// The color `azure`
    Azure,
    /// The color `beige`
    Beige,
    /// The color `bisque`
    Bisque,
    #[default]
    /// The color `black`
    Black,
    /// The color `blanched_almond`
    BlanchedAlmond,
    /// The color `blue`
    Blue,
    /// The color `blue_violet`
    BlueViolet,
    /// The color `brown`
    Brown,
    /// The color `burly_wood`
    BurlyWood,
    /// The color `cadet_blue`
    CadetBlue,
    /// The color `chartreuse`
    Chartreuse,
    /// The color `chocolate`
    Chocolate,
    /// The color `coral`
    Coral,
    /// The color `cornflower_blue`
    CornflowerBlue,
    /// The color `cornsilk`
    Cornsilk,
    /// The color `crimson`
    Crimson,
    /// The color `cyan`
    Cyan,
    /// The color `dark_blue`
    DarkBlue,
    /// The color `dark_cyan`
    DarkCyan,
    /// The color `dark_golden_rod`
    DarkGoldenRod,
    /// The color `dark_gray`
    DarkGray,
    /// The color `dark_grey`
    DarkGrey,
    /// The color `dark_green`
    DarkGreen,
    /// The color `dark_khaki`
    DarkKhaki,
    /// The color `dark_magenta`
    DarkMagenta,
    /// The color `dark_olive_green`
    DarkOliveGreen,
    /// The color `dark_orange`
    DarkOrange,
    /// The color `dark_orchid`
    DarkOrchid,
    /// The color `dark_red`
    DarkRed,
    /// The color `dark_salmon`
    DarkSalmon,
    /// The color `dark_sea_green`
    DarkSeaGreen,
    /// The color `dark_slate_blue`
    DarkSlateBlue,
    /// The color `dark_slate_gray`
    DarkSlateGray,
    /// The color `dark_slate_grey`
    DarkSlateGrey,
    /// The color `dark_turquoise`
    DarkTurquoise,
    /// The color `dark_violet`
    DarkViolet,
    /// The color `deep_pink`
    DeepPink,
    /// The color `deep_sky_blue`
    DeepSkyBlue,
    /// The color `dim_gray`
    DimGray,
    /// The color `dim_grey`
    DimGrey,
    /// The color `dodger_blue`
    DodgerBlue,
    /// The color `fire_brick`
    FireBrick,
    /// The color `floral_white`
    FloralWhite,
    /// The color `forest_green`
    ForestGreen,
    /// The color `fuchsia`
    Fuchsia,
    /// The color `gainsboro`
    Gainsboro,
    /// The color `ghost_white`
    GhostWhite,
    /// The color `gold`
    Gold,
    /// The color `golden_rod`
    GoldenRod,
    /// The color `gray`
    Gray,
    /// The color `grey`
    Grey,
    /// The color `green`
    Green,
    /// The color `green_yellow`
    GreenYellow,
    /// The color `honey_dew`
    HoneyDew,
    /// The color `hot_pink`
    HotPink,
    /// The color `indian_red`
    IndianRed,
    /// The color `indigo`
    Indigo,
    /// The color `ivory`
    Ivory,
    /// The color `khaki`
    Khaki,
    /// The color `lavender`
    Lavender,
    /// The color `lavender_blush`
    LavenderBlush,
    /// The color `lawn_green`
    LawnGreen,
    /// The color `lemon_chiffon`
    LemonChiffon,
    /// The color `light_blue`
    LightBlue,
    /// The color `light_coral`
    LightCoral,
    /// The color `light_cyan`
    LightCyan,
    /// The color `light_golden_rod_yellow`
    LightGoldenRodYellow,
    /// The color `light_gray`
    LightGray,
    /// The color `light_grey`
    LightGrey,
    /// The color `light_green`
    LightGreen,
    /// The color `light_pink`
    LightPink,
    /// The color `light_salmon`
    LightSalmon,
    /// The color `light_sea_green`
    LightSeaGreen,
    /// The color `light_sky_blue`
    LightSkyBlue,
    /// The color `light_slate_gray`
    LightSlateGray,
    /// The color `light_slate_grey`
    LightSlateGrey,
    /// The color `light_steel_blue`
    LightSteelBlue,
    /// The color `light_yellow`
    LightYellow,
    /// The color `lime`
    Lime,
    /// The color `lime_green`
    LimeGreen,
    /// The color `linen`
    Linen,
    /// The color `magenta`
    Magenta,
    /// The color `maroon`
    Maroon,
    /// The color `medium_aqua_marine`
    MediumAquaMarine,
    /// The color `medium_blue`
    MediumBlue,
    /// The color `medium_orchid`
    MediumOrchid,
    /// The color `medium_purple`
    MediumPurple,
    /// The color `medium_sea_green`
    MediumSeaGreen,
    /// The color `medium_slate_blue`
    MediumSlateBlue,
    /// The color `medium_spring_green`
    MediumSpringGreen,
    /// The color `medium_turquoise`
    MediumTurquoise,
    /// The color `medium_violet_red`
    MediumVioletRed,
    /// The color `midnight_blue`
    MidnightBlue,
    /// The color `mint_cream`
    MintCream,
    /// The color `misty_rose`
    MistyRose,
    /// The color `moccasin`
    Moccasin,
    /// The color `navajo_white`
    NavajoWhite,
    /// The color `navy`
    Navy,
    /// The color `old_lace`
    OldLace,
    /// The color `olive`
    Olive,
    /// The color `olive_drab`
    OliveDrab,
    /// The color `orange`
    Orange,
    /// The color `orange_red`
    OrangeRed,
    /// The color `orchid`
    Orchid,
    /// The color `pale_golden_rod`
    PaleGoldenRod,
    /// The color `pale_green`
    PaleGreen,
    /// The color `pale_turquoise`
    PaleTurquoise,
    /// The color `pale_violet_red`
    PaleVioletRed,
    /// The color `papaya_whip`
    PapayaWhip,
    /// The color `peach_puff`
    PeachPuff,
    /// The color `peru`
    Peru,
    /// The color `pink`
    Pink,
    /// The color `plum`
    Plum,
    /// The color `powder_blue`
    PowderBlue,
    /// The color `purple`
    Purple,
    /// The color `red`
    Red,
    /// The color `rosy_brown`
    RosyBrown,
    /// The color `royal_blue`
    RoyalBlue,
    /// The color `saddle_brown`
    SaddleBrown,
    /// The color `salmon`
    Salmon,
    /// The color `sandy_brown`
    SandyBrown,
    /// The color `sea_green`
    SeaGreen,
    /// The color `sea_shell`
    SeaShell,
    /// The color `sienna`
    Sienna,
    /// The color `silver`
    Silver,
    /// The color `sky_blue`
    SkyBlue,
    /// The color `slate_blue`
    SlateBlue,
    /// The color `slate_gray`
    SlateGray,
    /// The color `slate_grey`
    SlateGrey,
    /// The color `snow`
    Snow,
    /// The color `spring_green`
    SpringGreen,
    /// The color `steel_blue`
    SteelBlue,
    /// The color `tan`
    Tan,
    /// The color `teal`
    Teal,
    /// The color `thistle`
    Thistle,
    /// The color `tomato`
    Tomato,
    /// The color `turquoise`
    Turquoise,
    /// The color `violet`
    Violet,
    /// The color `wheat`
    Wheat,
    /// The color `white`
    White,
    /// The color `white_smoke`
    WhiteSmoke,
    /// The color `yellow`
    Yellow,
    /// The color `yellow_green`
    YellowGreen,
}

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

/// Represents a color
#[derive(Debug, Clone, Default, Copy)]
pub struct RgbColor {
    /// Red component
    pub r: u8,
    /// Green component
    pub g: u8,
    /// Blue component
    pub b: u8,
}

/// Represents a color with alpha channel
#[derive(Debug, Clone, Default, Copy)]
pub struct RgbaColor {
    /// Red component
    pub r: u8,
    /// Green component
    pub g: u8,
    /// Blue component
    pub b: u8,
    /// Alpha channel for transparency
    pub alpha: f64,
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

impl RgbColor {
    /// Initializer
    ///
    ///   * **r** - Red component
    ///   * **g** - Green component
    ///   * **b** - Blue component
    ///   * _return_ - The constructed instance
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }
}

impl RgbaColor {
    /// Initializer
    ///
    ///   * **r** - Red component
    ///   * **g** - Green component
    ///   * **b** - Blue component
    ///   * **alpha** - Alpha channel for transparency
    ///   * _return_ - The constructed instance
    pub fn new(r: u8, g: u8, b: u8, alpha: f64) -> Self {
        Self { r, g, b, alpha }
    }
}

impl SvgColor {
    /// Convert to html
    pub fn to_html() {
        // α <fn SvgColor::to_html>
        todo!("Implement `to_html`")
        // ω <fn SvgColor::to_html>
    }
}

/// Unit tests for `svg`
#[cfg(test)]
pub mod unit_tests {

    /// Test type SvgColor
    mod test_svg_color {
        ////////////////////////////////////////////////////////////////////////////////////
        // --- module uses ---
        ////////////////////////////////////////////////////////////////////////////////////
        use test_log::test;

        ////////////////////////////////////////////////////////////////////////////////////
        // --- functions ---
        ////////////////////////////////////////////////////////////////////////////////////
        #[test]
        fn to_html() {
            // α <fn test SvgColor::to_html>
            todo!("Test to_html")
            // ω <fn test SvgColor::to_html>
        }

        // α <mod-def test_svg_color>
        // ω <mod-def test_svg_color>
    }

    // α <mod-def unit_tests>
    // ω <mod-def unit_tests>
}

// α <mod-def svg>
// ω <mod-def svg>
