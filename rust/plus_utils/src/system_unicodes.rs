//! Enumeration of system unicode characters

////////////////////////////////////////////////////////////////////////////////////
// --- enums ---
////////////////////////////////////////////////////////////////////////////////////
/// Enumeration of unicodes used in the system.
pub enum SystemUnicodes {
    /// Character(🎲) - Game die
    GameDie,
    /// Character(🏦) - Bank
    Bank,
    /// Character(🧠) - Brain
    Brain,
    /// Character(💡) - Light bulb
    LightBulb,
    /// Character(🎲) - Game die representing single instrument, as opposed to category
    Instrument,
    /// Character(🗃) - Card file box
    CardFileBox,
    /// Character(🗄) - File cabinet, candidate for account
    FileCabinet,
    /// Character(🌌) - Galaxy, indicating complexity
    Galaxy,
    /// Character(🧮) - Abacus, candidate for account
    Abacus,
    /// Character(📋) - Clipboard, candidate for account
    Clipboard,
    /// Character(🗄) - File cabinet, for accounts
    Account,
    /// Character(📗) - Indicates data came from the dossier (e.g. embedded growth).
    DossierFolderSourced,
    /// Character(⚙️) - Indicates data came from user assumptions.
    UserAssumptionSourced,
    /// Character(X̄) - Indicates value is a sample mean.
    MathSampleMean,
    /// Character(s) - Indicates value is sample standard deviation
    MathSampleStdDev,
    /// Character(μg) - Indicates the estimate for geometric mean
    MathGeometricMeanEst,
    /// Character(σ) - Indicates value is a standard deviation (e.g. in a normal spec.
    MathSigma,
    /// Character(μ) - Indicates value is a mean (e.g. in a normal spec).
    MathMu,
    /// Character(ρ) - Indicates value is a correlation.
    MathRho,
    /// Character(ϱ) - The symbol used for measured correlation.
    MathRhoSymbol,
    /// Character(ε) - Indicates value is an error term
    MathError,
    /// Character(𝜎𝑥,𝑦) - Indicates covariance
    MathCovariance,
    /// Character(💰) - Indicates source from _plusauri_ user.
    PlusauriUser,
    /// Character(👤) - General user.
    GeneralUser,
    /// Character(🧪) - Indicates source from _test1_ user.
    Test1User,
    /// Character(💈) - Indicates source from _wealthy_barber_ user.
    WealthyBarberUser,
    /// Character(🤷) - Indicates something unknown.
    Unknown,
    /// Character(👎) - Indicates _gloomy_ outlook.
    OutlookGloomy,
    /// Character(⛈) - Indicates stormy weather ahead outlook.
    OutlookHoldOnTight,
    /// Character(👍) - Indicates _bright_ outlook.
    OutlookBright,
    /// Character(👉) - Indicates _standard_ outlook.
    OutlookStandard,
    /// Character(🏠) - House - used to indicate generally `Worth` or _Real Estate_.
    House,
    /// Character(📈) - Stock Chart - used to indicate `Holdings`
    StockChart,
    /// Character(📈) - Holding, currently stock chart
    Holding,
    /// Character(🚰) - Water faucet - used to indicate flows.
    Faucet,
    /// Character(∞) - Indicates infinity.
    Infinity,
    /// Character(➡️) - Right arrow block.
    RightArrowBlock,
    /// Character(➜) - Right arrow fat
    RightArrowFat,
    /// Character(⤐) - Right arrow fancy long
    RightArrowFancy1,
    /// Character(⤠) - Right arrow fancy long
    RightArrowFancy2,
    /// Character(⤘) - Right arrow fancy long
    RightArrowFancy3,
    /// Character(⇽) - Left arrow open head
    LeftArrowOpenHead,
    /// Character(﹩) - Small dollar sign
    SmallDollar,
    /// Character(⇚) - Left arrow triple.
    LeftArrowTriple,
    /// Character(⬅) - Left arrow fat.
    LeftArrowFat,
    /// Character(⬆) - In flow
    InFlow,
    /// Character(⬇) - Out flow
    OutFlow,
    /// Character(🪦) - Tombstone
    Tombstone,
    /// Character(🏖) - Beach Umbrella
    BeachUmbrella,
    /// Character(▼) - Down Triangle
    DownTriangle,
    /// Character(▲) - Up Triangle
    UpTriangle,
}

////////////////////////////////////////////////////////////////////////////////////
// --- type impls ---
////////////////////////////////////////////////////////////////////////////////////
impl SystemUnicodes {
    /// Returns the unicode text.
    ///
    ///   * _return_ - The unicode character
    pub fn as_unicode(&self) -> &'static str {
        match self {
            SystemUnicodes::GameDie => "🎲",
            SystemUnicodes::Bank => "🏦",
            SystemUnicodes::Brain => "🧠",
            SystemUnicodes::LightBulb => "💡",
            SystemUnicodes::Instrument => "🎲",
            SystemUnicodes::CardFileBox => "🗃",
            SystemUnicodes::FileCabinet => "🗄",
            SystemUnicodes::Galaxy => "🌌",
            SystemUnicodes::Abacus => "🧮",
            SystemUnicodes::Clipboard => "📋",
            SystemUnicodes::Account => "🗄",
            SystemUnicodes::DossierFolderSourced => "📗",
            SystemUnicodes::UserAssumptionSourced => "⚙️",
            SystemUnicodes::MathSampleMean => "X̄",
            SystemUnicodes::MathSampleStdDev => "s",
            SystemUnicodes::MathGeometricMeanEst => "μg",
            SystemUnicodes::MathSigma => "σ",
            SystemUnicodes::MathMu => "μ",
            SystemUnicodes::MathRho => "ρ",
            SystemUnicodes::MathRhoSymbol => "ϱ",
            SystemUnicodes::MathError => "ε",
            SystemUnicodes::MathCovariance => "𝜎𝑥,𝑦",
            SystemUnicodes::PlusauriUser => "💰",
            SystemUnicodes::GeneralUser => "👤",
            SystemUnicodes::Test1User => "🧪",
            SystemUnicodes::WealthyBarberUser => "💈",
            SystemUnicodes::Unknown => "🤷",
            SystemUnicodes::OutlookGloomy => "👎",
            SystemUnicodes::OutlookHoldOnTight => "⛈",
            SystemUnicodes::OutlookBright => "👍",
            SystemUnicodes::OutlookStandard => "👉",
            SystemUnicodes::House => "🏠",
            SystemUnicodes::StockChart => "📈",
            SystemUnicodes::Holding => "📈",
            SystemUnicodes::Faucet => "🚰",
            SystemUnicodes::Infinity => "∞",
            SystemUnicodes::RightArrowBlock => "➡️",
            SystemUnicodes::RightArrowFat => "➜",
            SystemUnicodes::RightArrowFancy1 => "⤐",
            SystemUnicodes::RightArrowFancy2 => "⤠",
            SystemUnicodes::RightArrowFancy3 => "⤘",
            SystemUnicodes::LeftArrowOpenHead => "⇽",
            SystemUnicodes::SmallDollar => "﹩",
            SystemUnicodes::LeftArrowTriple => "⇚",
            SystemUnicodes::LeftArrowFat => "⬅",
            SystemUnicodes::InFlow => "⬆",
            SystemUnicodes::OutFlow => "⬇",
            SystemUnicodes::Tombstone => "🪦",
            SystemUnicodes::BeachUmbrella => "🏖",
            SystemUnicodes::DownTriangle => "▼",
            SystemUnicodes::UpTriangle => "▲",
        }
    }
}

/// Unit tests for `system_unicodes`
#[cfg(test)]
pub mod unit_tests {

    /// Test type SystemUnicodes
    mod test_system_unicodes {
        ////////////////////////////////////////////////////////////////////////////////////
        // --- module uses ---
        ////////////////////////////////////////////////////////////////////////////////////
        use test_log::test;

        ////////////////////////////////////////////////////////////////////////////////////
        // --- functions ---
        ////////////////////////////////////////////////////////////////////////////////////
        #[test]
        fn as_unicode() {
            // α <fn test SystemUnicodes::as_unicode>
            assert_eq!("σ", SystemUnicodes::MathSigma.as_unicode());
            assert_eq!("μ", SystemUnicodes::MathMu.as_unicode());
            assert_eq!("ρ", SystemUnicodes::MathRho.as_unicode());
            // ω <fn test SystemUnicodes::as_unicode>
        }

        // α <mod-def test_system_unicodes>
        use super::*;
        // ω <mod-def test_system_unicodes>
    }

    // α <mod-def unit_tests>
    use super::*;
    // ω <mod-def unit_tests>
}

// α <mod-def system_unicodes>
// ω <mod-def system_unicodes>
