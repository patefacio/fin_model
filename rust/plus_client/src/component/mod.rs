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
/// All css classes not directly associated with a component.
pub enum CssClasses {
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
}

////////////////////////////////////////////////////////////////////////////////////
// --- type impls ---
////////////////////////////////////////////////////////////////////////////////////
impl CssClasses {
    /// Get the class name
    ///
    ///   * _return_ - The class name
    #[inline]
    pub fn as_str(&self) -> &'static str {
        match self {
            CssClasses::TxtRight => "txt-right",
            CssClasses::TxtRightPadLeft => "txt-right-pad-left",
            CssClasses::PlLeft1 => "pl-left-1",
            CssClasses::Title => "title",
            CssClasses::Invalid => "invalid",
            CssClasses::TblCaption => "tbl-caption",
            CssClasses::InFlow => "in-flow",
            CssClasses::OutFlow => "out-flow",
            CssClasses::BtnTbTop => "btn-tb-top",
            CssClasses::BtnTbRight => "btn-tb-right",
            CssClasses::BtnTbBottom => "btn-tb-bottom",
            CssClasses::BtnTbLeft => "btn-tb-left",
            CssClasses::LblBtn => "lbl-btn",
            CssClasses::TibImg => "tib-img",
            CssClasses::TibLbl => "tib-lbl",
            CssClasses::GridLbl => "grid-lbl",
            CssClasses::HeaderLeft => "header-left",
            CssClasses::HeaderRight => "header-right",
            CssClasses::ContentTable => "content-table",
            CssClasses::Numeric => "numeric",
            CssClasses::TaxBillTr => "tax-bill-tr",
            CssClasses::Form => "form",
            CssClasses::FormRow => "form-row",
            CssClasses::InfoLbl => "info-lbl",
            CssClasses::AcHoldings => "ac-holdings",
            CssClasses::MbsView => "mbs-view",
            CssClasses::MbsInterior => "mbs-interior",
            CssClasses::TibOn => "tib-on",
            CssClasses::TibOff => "tib-off",
            CssClasses::DossierToolbar => "dossier-toolbar",
            CssClasses::DossierShow => "dossier-show",
            CssClasses::DossierNameInput => "dossier-name-input",
            CssClasses::AppLangSelect => "app-lang-select",
            CssClasses::AppLangSelectInner => "app-lang-select-inner",
            CssClasses::AppCurrencySelect => "app-currency-select",
            CssClasses::CollapsibleHeader => "collapsible-header",
            CssClasses::CollapseButton => "collapse-button",
            CssClasses::CollectionGrid => "collection-grid",
            CssClasses::CgcEditable => "cgc-editable",
            CssClasses::CgcCell => "cgc-cell",
            CssClasses::CgcAddRow => "cgc-add-row",
            CssClasses::CftZoom => "cft-zoom",
            CssClasses::OkCancelBar => "ok-cancel-bar",
            CssClasses::FscCtnr => "fsc-ctnr",
            CssClasses::FscNameInput => "fsc-name-input",
            CssClasses::FscType => "fsc-type",
            CssClasses::FscDirectionRbs => "fsc-direction-rbs",
            CssClasses::YriStart => "yri-start",
            CssClasses::YriEnd => "yri-end",
            CssClasses::McsLabel => "mcs-label",
            CssClasses::McsSelectLabel => "mcs-select-label",
            CssClasses::McsIconLabel => "mcs-icon-label",
            CssClasses::McsIcon => "mcs-icon",
            CssClasses::NscFieldset => "nsc-fieldset",
            CssClasses::NscExplore => "nsc-explore",
            CssClasses::VfscContainer => "vfsc-container",
            CssClasses::VfscCategory => "vfsc-category",
            CssClasses::VfscCurrency => "vfsc-currency",
            CssClasses::VfscFlows => "vfsc-flows",
            CssClasses::YcvCurrency => "ycv-currency",
            CssClasses::YcvValue => "ycv-value",
            CssClasses::YcvAsOf => "ycv-as-of",
            CssClasses::YcvYear => "ycv-year",
            CssClasses::McsGrid => "mcs-grid",
            CssClasses::McsMainBtn => "mcs-main-btn",
            CssClasses::McsSelectBtn => "mcs-select-btn",
            CssClasses::McsCtnr => "mcs-ctnr",
            CssClasses::OccOkBtn => "occ-ok-btn",
            CssClasses::OccCancelBtn => "occ-cancel-btn",
            CssClasses::OocCtnr => "ooc-ctnr",
            CssClasses::OocRbLtr => "ooc-rb-ltr",
            CssClasses::OocRbTtb => "ooc-rb-ttb",
            CssClasses::YiInput => "yi-input",
            CssClasses::SwniSlider => "swni-slider",
            CssClasses::HrrcCtnr => "hrrc-ctnr",
            CssClasses::HrrcDot => "hrrc-dot",
            CssClasses::HrrcLblCtnr => "hrrc-lbl-ctnr",
            CssClasses::HrrcLbl => "hrrc-lbl",
            CssClasses::HrrcPlot => "hrrc-plot",
            CssClasses::NlcLblCtnr => "nlc-lbl-ctnr",
            CssClasses::HcExtrasCtnr => "hc-extras-ctnr",
            CssClasses::HistSelectedRow => "hist-selected-row",
            CssClasses::MsscDist => "mssc-dist",
            CssClasses::MsscCircle1 => "mssc-circle-1",
            CssClasses::MsscCircle2 => "mssc-circle-2",
            CssClasses::MsscCircle3 => "mssc-circle-3",
            CssClasses::MsscCircle4 => "mssc-circle-4",
            CssClasses::MsscMedian => "mssc-median",
            CssClasses::MsscMean => "mssc-mean",
            CssClasses::MsscGmf => "mssc-gmf",
            CssClasses::ChHists => "ch-hists",
            CssClasses::ChNormal => "ch-normal",
            CssClasses::ChLognormal => "ch-lognormal",
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////
// --- trait impls ---
////////////////////////////////////////////////////////////////////////////////////
impl Display for CssClasses {
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
