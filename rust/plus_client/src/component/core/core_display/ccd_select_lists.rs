//! Module for ccd_select_lists leptos function/component

////////////////////////////////////////////////////////////////////////////////////
// --- module uses ---
////////////////////////////////////////////////////////////////////////////////////
use leptos::component;
use leptos::view;
#[allow(unused_imports)]
use leptos::IntoAttribute;
use leptos::IntoView;
use leptos::WriteSignal;

////////////////////////////////////////////////////////////////////////////////////
// --- functions ---
////////////////////////////////////////////////////////////////////////////////////
/// Show various select lists
///
///   * **show_update** - Function to display state updates
///   * _return_ - View for ccd_select_lists
#[component]
pub fn CcdSelectLists(
    /// Function to display state updates
    show_update: WriteSignal<String>,
) -> impl IntoView {
    pub const SELF_CLASS: &str = "plus-csl; ccd-section";
    let component_id = crate::component_id!("`CcdSelectLists`");
    #[cfg(debug_assertions)]
    crate::log_component!(crate::COMPONENT_LOG_LEVEL, component_id);
    // α <fn ccd_select_lists>

    use crate::CurrencySelect;
    use crate::EnumSelect;
    use crate::SelectDirection;
    use crate::Updatable;
    use leptos::SignalSet;
    use plus_modeled::Currency;
    use plus_modeled::StateOfResidence;

    // ω <fn ccd_select_lists>
    view! {
        <div class=SELF_CLASS>
            // α <plus-csl-view>
            <div class="title">"Select Lists"</div>
            <div class="ccd-selects">
                <div style="padding: 1em;">
                    <h4>"Mutli-Column Select (Top To Bottom)"</h4>
                    <p inner_html="
                    <p>
                    A component that supports a drop down selection which can span multiple columns.
                    The purpose is to be able to better style large selection lists that otherwise would
                    be very long vertical lists. The features include:
                    </p>
                    <ul>
                    <li>Navigating the selections with left/right and up/down arrow keys</li>
                    <li> Tab support for each entry</li>
                    <li>Focus <strong>auto-selects</strong> the item so tabbing and navigating to selection makes it the
                    current selection</li>
                    <li>Accept the selection <strong>and signals</strong> on Enter, Space</li>
                    <li>No-Op on Escape</li>
                    <li>Display Selections Left-To-Right or Top-To-Bottom</li>
                    <li>EnumSelect supports filtering (e.g. this has filtered out <strong>Il</strong> and <strong>Ca</strong>)</li>
                    </ul>
                    "></p>
                    <EnumSelect
                        updatable=Updatable::new(
                            StateOfResidence::Fl,
                            move |state| { show_update.set(format!("State updated -> {state:?}")) },
                        )

                        direction=SelectDirection::TopToBottom
                        column_count=5
                        filter=Some(
                            std::boxed::Box::new(|&e| {
                                e != StateOfResidence::Il && e != StateOfResidence::Ca
                            }),
                        )
                    />

                </div>
                <div style="padding: 1em;">
                    <h4>"Mutli-Column Select (Left To Right)"</h4>
                    <EnumSelect
                        updatable=Updatable::new(
                            StateOfResidence::Fl,
                            move |state| { show_update.set(format!("State updated -> {state:?}")) },
                        )

                        direction=SelectDirection::LeftToRight
                        column_count=5
                    />
                </div>
                <div style="padding: 1em;">
                    <h4>"Currency Select"</h4>
                    <p inner_html="
                    <em>Two Field, Three Column Example of <strong>MultiColumnSelect</strong></em>.
                    "></p>
                    <CurrencySelect updatable=Updatable::new(
                        Currency::Eur,
                        move |currency: &Currency| {
                            show_update.set(format!("Currency set to {currency:?}"));
                        },
                    )/>
                </div>
            </div>
        // ω <plus-csl-view>
        </div>
    }
}

// α <mod-def ccd_select_lists>
// ω <mod-def ccd_select_lists>
