//! Components for plusauri financial planning application

////////////////////////////////////////////////////////////////////////////////////
// --- module uses ---
////////////////////////////////////////////////////////////////////////////////////
use ::core::fmt::Display;
use ::core::fmt::Formatter;

////////////////////////////////////////////////////////////////////////////////////
// --- mod decls ---
////////////////////////////////////////////////////////////////////////////////////
pub mod app;
pub mod core;
pub mod log_component;

////////////////////////////////////////////////////////////////////////////////////
// --- enums ---
////////////////////////////////////////////////////////////////////////////////////
/// `plus_client` css classes.
pub enum ClientCssClasses {
    /// Adds `text-align=right;`
    TxtRight,
    /// Adds `text-align=right; padding-left=1rem;`
    TxtRightPadLeft,
    /// Adds `padding-left=1rem;`
    PlLeft1,
    /// Style text that is a title to some data like a chart.
    Title,
    /// Style for invalid content
    Invalid,
    /// Style table caption
    TblCaption,
    /// Style in flow text
    InFlow,
    /// Style out flow text
    OutFlow,
    /// Styles a button toolbar to the top
    BtnTbTop,
    /// Styles a button toolbar to the right
    BtnTbRight,
    /// Styles a button toolbar to the bottom
    BtnTbBottom,
    /// Styles a button toolbar to the left
    BtnTbLeft,
    /// Styles an image button with a label beneath
    LblBtn,
    /// Styles the image in a ToggleImageButton
    TibImg,
    /// Styles the label for a ToggleImageButton
    TibLbl,
    /// Style grid label in various CollectionGridComponents
    GridLbl,
    /// Style header to left
    HeaderLeft,
    /// Style header to right
    HeaderRight,
    /// Style used on askama tables.
    /// Used extensively and the css class will not show up in a search
    /// that does not include askama html templates
    ContentTable,
    /// Styles numeric `td` elements
    Numeric,
    /// Tax bill statement's tax bill row
    TaxBillTr,
    /// Style form content
    Form,
    /// Style form row
    FormRow,
    /// Style form row 2 cols
    FormRow2Col,
    /// Styles dynamic labels such as HoldingComponent's _market value_
    /// and _unrealized gain/loss_ and ItemGrowthSelect display of the NormalSpec
    InfoLbl,
    /// Style AccountComponent holdings
    AcHoldings,
    /// Styles the view portion of the MultiButtonSelect
    MbsView,
    /// Style the interior div that has grid which is dynamically styled
    MbsInterior,
    /// Style of bar under ToggleImageButton when selected (ie on)
    TibOn,
    /// Style of bar under ToggleImageButton when selected (ie on)
    TibOff,
    /// Style the dossier toolbar
    DossierToolbar,
    /// Style the shown area of dossier editor
    DossierShow,
    /// Style the dossier name input in the editor
    DossierNameInput,
    /// Style the language select in the nav bar
    AppLangSelect,
    /// Style the inner div of lang select
    AppLangSelectInner,
    /// Style the currency select in the nav bar
    AppCurrencySelect,
    /// Style the collapsible header
    CollapsibleHeader,
    /// Style the collapse button of collapsible
    CollapseButton,
    /// Style the collection grid container
    CollectionGrid,
    /// Style CollectionGridComponent editable section
    CgcEditable,
    /// Style the cell contents
    CgcCell,
    /// Style the add row button
    CgcAddRow,
    /// Style the CashFlowTimeline zoom div
    CftZoom,
    /// Style the OkCancelBar
    OkCancelBar,
    /// Style the container (grid) for the FlowSpecComponent
    FscCtnr,
    /// Style/Position the name in the FlowSpecComponent
    FscNameInput,
    /// Style/Position the OneOfComponent outlining the Growth vs Custom in the FlowSpecComponent.
    /// Used to position this element within the grid
    FscType,
    /// Style/Position the _in_ vs _out_ radio buttons in the FlowSpecComponent
    FscDirectionRbs,
    /// YearRangeInput start entry
    YriStart,
    /// YearRangeInput end entry
    YriEnd,
    /// Style the MultiColumnSelect labels
    McsLabel,
    /// Style the MultiColumnSelect labels
    McsSelectLabel,
    /// Style the icon label in MultiColumnSelect
    McsIconLabel,
    /// Style the icon in MultiColumnSelect
    McsIcon,
    /// Style the NormalSpecComponent fieldset
    NscFieldset,
    /// Style the explore normal div
    NscExplore,
    /// Value Flow Spec container
    VfscContainer,
    /// Value Flow Spec category
    VfscCategory,
    /// Value Flow Spec currency
    VfscCurrency,
    /// Value Flow Spec flows
    VfscFlows,
    /// YearCurrencyValue currency element
    YcvCurrency,
    /// YearCurrencyValue value element
    YcvValue,
    /// YearCurrencyValue _as of_ element
    YcvAsOf,
    /// YearCurrencyValue year input element
    YcvYear,
    /// Wraps the MultiColumnSelect
    McsGrid,
    /// Main button in MultiColumnSelect
    McsMainBtn,
    /// Select button in MultiColumnSelect
    McsSelectBtn,
    /// Container holding selections of MultiColumnSelect
    McsCtnr,
    /// OkCancel component ok button
    OccOkBtn,
    /// OkCancel component cancel button
    OccCancelBtn,
    /// Internal OneOfComponent container
    OocCtnr,
    /// OneOfComponent radio button left-to-right
    OocRbLtr,
    /// OneOfComponent radio button top-to-bottom
    OocRbTtb,
    /// Input field in YearInput
    YiInput,
    /// The slider inside SliderWithNumericInput
    SwniSlider,
    /// HistoricRiskReturn container
    HrrcCtnr,
    /// HistoricRiskReturn dot
    HrrcDot,
    /// HistoricRiskReturn label container
    HrrcLblCtnr,
    /// HistoricRiskReturn label
    HrrcLbl,
    /// HistoricRiskReturn plot
    HrrcPlot,
    /// NormalLossComponent label
    NlcLblCtnr,
    /// HoldingComponent container for growth and distributions
    HcExtrasCtnr,
    /// Histogram selected row
    HistSelectedRow,
    /// Histogram pair of svg and table
    HistPair,
    /// MonteSimulationSummaryComponent distribution plot
    MsscDist,
    /// MonteSimulationSummaryComponent distribution plot falling circles 1
    MsscCircle1,
    /// MonteSimulationSummaryComponent distribution plot falling circles 2
    MsscCircle2,
    /// MonteSimulationSummaryComponent distribution plot falling circles 3
    MsscCircle3,
    /// MonteSimulationSummaryComponent distribution plot falling circles 4
    MsscCircle4,
    /// MonteSimulationSummaryComponent distribution median
    MsscMedian,
    /// MonteSimulationSummaryComponent distribution mean
    MsscMean,
    /// MonteSimulationSummaryComponent distribution `geometric_mean_forecast`
    MsscGmf,
    /// CcdHistogram histograms div
    ChHists,
    /// CcdHistogram normal div
    ChNormal,
    /// CcdHistogram normal div
    ChLognormal,
    /// ReconciledView grid to hold the equation
    RvGrid,
    /// ReconciledView income term
    RvIncome,
    /// ReconciledView expense term
    RvExpense,
    /// ReconciledView assets term
    RvAssets,
    /// ReconciledView equation term label
    RvGridTerm,
    /// ReconciledView operator (ie `plus` `equals`)
    RvGridOp,
    /// ReconciledView numeric value
    RvGridVal,
    /// ReconciledView _holding plan table_ and _tax table_
    RvTables,
    /// Balance Sheet Statement caption - which includes detail toggle button
    BssCaption,
    /// AssetLiabilityComponent inner content, inside the OneOfComponent
    AlcInner,
    /// AssetLiabilityComponent svg with balance beam
    AlcSvg,
    /// AssetLiabilityComponent svg container
    AlcSvgCtnr,
    /// AssetLiabilityComponent asset and liability tables
    AlcTables,
    /// AssetLiabilityComponent Asset table
    AlcAssetTbl,
    /// AssetLiabilityComponent Liability table
    AlcLiabilityTbl,
    /// Monte Carlo distribution summary
    McDistSvg,
    /// AppNavBar Plusauri Dossiers
    AnbPlusDossiers,
}

////////////////////////////////////////////////////////////////////////////////////
// --- type impls ---
////////////////////////////////////////////////////////////////////////////////////
impl ClientCssClasses {
    /// Get the class name
    ///
    ///   * _return_ - The class name
    #[inline]
    pub fn as_str(&self) -> &'static str {
        match self {
            ClientCssClasses::TxtRight => "txt-right",
            ClientCssClasses::TxtRightPadLeft => "txt-right-pad-left",
            ClientCssClasses::PlLeft1 => "pl-left-1",
            ClientCssClasses::Title => "title",
            ClientCssClasses::Invalid => "invalid",
            ClientCssClasses::TblCaption => "tbl-caption",
            ClientCssClasses::InFlow => "in-flow",
            ClientCssClasses::OutFlow => "out-flow",
            ClientCssClasses::BtnTbTop => "btn-tb-top",
            ClientCssClasses::BtnTbRight => "btn-tb-right",
            ClientCssClasses::BtnTbBottom => "btn-tb-bottom",
            ClientCssClasses::BtnTbLeft => "btn-tb-left",
            ClientCssClasses::LblBtn => "lbl-btn",
            ClientCssClasses::TibImg => "tib-img",
            ClientCssClasses::TibLbl => "tib-lbl",
            ClientCssClasses::GridLbl => "grid-lbl",
            ClientCssClasses::HeaderLeft => "header-left",
            ClientCssClasses::HeaderRight => "header-right",
            ClientCssClasses::ContentTable => "content-table",
            ClientCssClasses::Numeric => "numeric",
            ClientCssClasses::TaxBillTr => "tax-bill-tr",
            ClientCssClasses::Form => "form",
            ClientCssClasses::FormRow => "form-row",
            ClientCssClasses::FormRow2Col => "form-row-2-col",
            ClientCssClasses::InfoLbl => "info-lbl",
            ClientCssClasses::AcHoldings => "ac-holdings",
            ClientCssClasses::MbsView => "mbs-view",
            ClientCssClasses::MbsInterior => "mbs-interior",
            ClientCssClasses::TibOn => "tib-on",
            ClientCssClasses::TibOff => "tib-off",
            ClientCssClasses::DossierToolbar => "dossier-toolbar",
            ClientCssClasses::DossierShow => "dossier-show",
            ClientCssClasses::DossierNameInput => "dossier-name-input",
            ClientCssClasses::AppLangSelect => "app-lang-select",
            ClientCssClasses::AppLangSelectInner => "app-lang-select-inner",
            ClientCssClasses::AppCurrencySelect => "app-currency-select",
            ClientCssClasses::CollapsibleHeader => "collapsible-header",
            ClientCssClasses::CollapseButton => "collapse-button",
            ClientCssClasses::CollectionGrid => "collection-grid",
            ClientCssClasses::CgcEditable => "cgc-editable",
            ClientCssClasses::CgcCell => "cgc-cell",
            ClientCssClasses::CgcAddRow => "cgc-add-row",
            ClientCssClasses::CftZoom => "cft-zoom",
            ClientCssClasses::OkCancelBar => "ok-cancel-bar",
            ClientCssClasses::FscCtnr => "fsc-ctnr",
            ClientCssClasses::FscNameInput => "fsc-name-input",
            ClientCssClasses::FscType => "fsc-type",
            ClientCssClasses::FscDirectionRbs => "fsc-direction-rbs",
            ClientCssClasses::YriStart => "yri-start",
            ClientCssClasses::YriEnd => "yri-end",
            ClientCssClasses::McsLabel => "mcs-label",
            ClientCssClasses::McsSelectLabel => "mcs-select-label",
            ClientCssClasses::McsIconLabel => "mcs-icon-label",
            ClientCssClasses::McsIcon => "mcs-icon",
            ClientCssClasses::NscFieldset => "nsc-fieldset",
            ClientCssClasses::NscExplore => "nsc-explore",
            ClientCssClasses::VfscContainer => "vfsc-container",
            ClientCssClasses::VfscCategory => "vfsc-category",
            ClientCssClasses::VfscCurrency => "vfsc-currency",
            ClientCssClasses::VfscFlows => "vfsc-flows",
            ClientCssClasses::YcvCurrency => "ycv-currency",
            ClientCssClasses::YcvValue => "ycv-value",
            ClientCssClasses::YcvAsOf => "ycv-as-of",
            ClientCssClasses::YcvYear => "ycv-year",
            ClientCssClasses::McsGrid => "mcs-grid",
            ClientCssClasses::McsMainBtn => "mcs-main-btn",
            ClientCssClasses::McsSelectBtn => "mcs-select-btn",
            ClientCssClasses::McsCtnr => "mcs-ctnr",
            ClientCssClasses::OccOkBtn => "occ-ok-btn",
            ClientCssClasses::OccCancelBtn => "occ-cancel-btn",
            ClientCssClasses::OocCtnr => "ooc-ctnr",
            ClientCssClasses::OocRbLtr => "ooc-rb-ltr",
            ClientCssClasses::OocRbTtb => "ooc-rb-ttb",
            ClientCssClasses::YiInput => "yi-input",
            ClientCssClasses::SwniSlider => "swni-slider",
            ClientCssClasses::HrrcCtnr => "hrrc-ctnr",
            ClientCssClasses::HrrcDot => "hrrc-dot",
            ClientCssClasses::HrrcLblCtnr => "hrrc-lbl-ctnr",
            ClientCssClasses::HrrcLbl => "hrrc-lbl",
            ClientCssClasses::HrrcPlot => "hrrc-plot",
            ClientCssClasses::NlcLblCtnr => "nlc-lbl-ctnr",
            ClientCssClasses::HcExtrasCtnr => "hc-extras-ctnr",
            ClientCssClasses::HistSelectedRow => "hist-selected-row",
            ClientCssClasses::HistPair => "hist-pair",
            ClientCssClasses::MsscDist => "mssc-dist",
            ClientCssClasses::MsscCircle1 => "mssc-circle-1",
            ClientCssClasses::MsscCircle2 => "mssc-circle-2",
            ClientCssClasses::MsscCircle3 => "mssc-circle-3",
            ClientCssClasses::MsscCircle4 => "mssc-circle-4",
            ClientCssClasses::MsscMedian => "mssc-median",
            ClientCssClasses::MsscMean => "mssc-mean",
            ClientCssClasses::MsscGmf => "mssc-gmf",
            ClientCssClasses::ChHists => "ch-hists",
            ClientCssClasses::ChNormal => "ch-normal",
            ClientCssClasses::ChLognormal => "ch-lognormal",
            ClientCssClasses::RvGrid => "rv-grid",
            ClientCssClasses::RvIncome => "rv-income",
            ClientCssClasses::RvExpense => "rv-expense",
            ClientCssClasses::RvAssets => "rv-assets",
            ClientCssClasses::RvGridTerm => "rv-grid-term",
            ClientCssClasses::RvGridOp => "rv-grid-op",
            ClientCssClasses::RvGridVal => "rv-grid-val",
            ClientCssClasses::RvTables => "rv-tables",
            ClientCssClasses::BssCaption => "bss-caption",
            ClientCssClasses::AlcInner => "alc-inner",
            ClientCssClasses::AlcSvg => "alc-svg",
            ClientCssClasses::AlcSvgCtnr => "alc-svg-ctnr",
            ClientCssClasses::AlcTables => "alc-tables",
            ClientCssClasses::AlcAssetTbl => "alc-asset-tbl",
            ClientCssClasses::AlcLiabilityTbl => "alc-liability-tbl",
            ClientCssClasses::McDistSvg => "mc-dist-svg",
            ClientCssClasses::AnbPlusDossiers => "anb-plus-dossiers",
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////
// --- trait impls ---
////////////////////////////////////////////////////////////////////////////////////
impl Display for ClientCssClasses {
    /// Format the instance.
    ///
    ///   * **f** - Formatter to push formatted item to.
    ///   * _return_ - Formatted instance
    fn fmt(&self, #[allow(unused)] f: &mut Formatter<'_>) -> ::core::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

// α <mod-def component>
// ω <mod-def component>
