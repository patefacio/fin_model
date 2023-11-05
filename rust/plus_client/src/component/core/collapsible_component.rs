//! Module for collapsible_component leptos function/component

////////////////////////////////////////////////////////////////////////////////////
// --- module uses ---
////////////////////////////////////////////////////////////////////////////////////
use leptos::ChildrenFn;
use leptos::{component, view, IntoView};

////////////////////////////////////////////////////////////////////////////////////
// --- functions ---
////////////////////////////////////////////////////////////////////////////////////
/// Provides a bar with a label to expand/collapse contained content.
///
///   * **collapsed_header** - Content of the header bar when collapsed
///   * **expanded_header** - Content of the header bar when expanded.
/// If `None` will be same as `collapsed_header`
///   * **children** - Children to show when expanded
///   * **is_expanded** - Is the item expanded
///   * _return_ - View for collapsible_component
#[component]
pub fn CollapsibleComponent(
    /// Content of the header bar when collapsed
    collapsed_header: String,
    /// Content of the header bar when expanded.
    /// If `None` will be same as `collapsed_header`
    #[prop(default=None)]
    expanded_header: Option<String>,
    /// Children to show when expanded
    children: ChildrenFn,
    /// Is the item expanded
    #[prop(default = false)]
    is_expanded: bool,
) -> impl IntoView {
    crate::log_component!("`CollapsibleComponent`");
    // α <fn collapsible_component>

    use crate::CssClasses;
    use leptos::create_rw_signal;
    use leptos::IntoAttribute;
    use leptos::Show;
    use leptos::SignalGet;
    use leptos::SignalUpdate;
    use plus_utils::SystemUnicodes;
    let is_expanded = create_rw_signal(is_expanded);
    let expanded_header = expanded_header.unwrap_or_else(|| collapsed_header.clone());

    let header = move || {
        if is_expanded.get() {
            expanded_header.clone()
        } else {
            collapsed_header.clone()
        }
    };

    view! {
        <div class=CssClasses::CollapsibleHeader.to_string()>
            <div>{move || header()}</div>
            <button on:click=move |_| {
                is_expanded.update(|is_expanded| *is_expanded = !*is_expanded)
            }>
                {move || {
                    if is_expanded.get() {
                        SystemUnicodes::UpTriangle.as_unicode()
                    } else {
                        SystemUnicodes::DownTriangle.as_unicode()
                    }
                }}

            </button>
        </div>
        <Show when=move || is_expanded.get() fallback=|| ()>
            {children()}
        </Show>
    }

    // ω <fn collapsible_component>
}

// α <mod-def collapsible_component>
// ω <mod-def collapsible_component>
