//! Module for css_show leptos function/component

////////////////////////////////////////////////////////////////////////////////////
// --- module uses ---
////////////////////////////////////////////////////////////////////////////////////
use leptos::component;
use leptos::view;
use leptos::ChildrenFn;
#[allow(unused_imports)]
use leptos::IntoAttribute;
use leptos::IntoView;
use leptos::MaybeSignal;

////////////////////////////////////////////////////////////////////////////////////
// --- functions ---
////////////////////////////////////////////////////////////////////////////////////
/// A show component which always has the same view but show is controlled via css hidden display type
///
///   * **children** - The children will be shown whenever the condition in the `when` closure returns `true`
///   * **when** - Value controlling the display hidden vs not state.
///   * **display_type** - Style display type when not hidden
///   * _return_ - View for css_show
#[component]
pub fn CssShow(
    /// The children will be shown whenever the condition in the `when` closure returns `true`
    children: ChildrenFn,
    /// Value controlling the display hidden vs not state.
    #[prop(default=MaybeSignal::Static(true), into)]
    when: MaybeSignal<bool>,
    /// Style display type when not hidden
    display_type: String,
) -> impl IntoView {
    crate::log_component!("`CssShow`");
    // α <fn css_show>

    use leptos::SignalGet;

    let style = move || {
        if when.get() {
            format!("display: {display_type}")
        } else {
            "display: none;".to_string()
        }
    };

    // ω <fn css_show>
    view! {
          // α <plus-css-show-view>

          <div style=style>{children}</div> 

          // ω <plus-css-show-view>
    }
}

// α <mod-def css_show>
// ω <mod-def css_show>
