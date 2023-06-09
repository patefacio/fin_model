//! Module for dispose_test leptos function/component

////////////////////////////////////////////////////////////////////////////////////
// --- module uses ---
////////////////////////////////////////////////////////////////////////////////////
#[allow(unused_imports)]
use leptos::log;
use leptos::{component, view, IntoView, Scope};
#[allow(unused_imports)]
use leptos_dom::console_log;

////////////////////////////////////////////////////////////////////////////////////
// --- functions ---
////////////////////////////////////////////////////////////////////////////////////
/// See what gets cleaned up.
///
///   * **cx** - Context
///   * _return_ - View for dispose_test
#[component]
pub fn DisposeTest(
    /// Context
    cx: Scope,
) -> impl IntoView {
    // α <fn dispose_test>

    use leptos::*;
    use std::rc::Rc;

    let sz = std::mem::size_of_val(&cx);

    let some_data = Rc::new(SomeData::new(&format!("DisposeTest Badabing:{cx:?}")));

    /*
        let (some_data, some_data_write) = leptos::create_signal(
            cx,
           SomeData::new(&format!("DisposeTest Badabing:{cx:?}"))
        );
    */
    let do_stuff = move || {
        leptos_dom::console_log(&format!("Doing stuff"));
    };

    let do_stuff = leptos::store_value(cx, do_stuff);

    // leptos_dom::console_log(&format!(
    //     "Size of read signal is {}, size of write signal is {}",
    //     std::mem::size_of_val(&some_data),
    //     std::mem::size_of_val(&some_data_write),
    // ));

    leptos_dom::console_log(&format!("DisposeTest cx({cx:?}"));
    let log_dispose_item = crate::utils::log_dispose::LogDispose::new("dIsPoSe".into());

    let on_click = leptos::store_value(cx, move |_| {
        println!("{log_dispose_item:?}");
        console_log("Clicked Bam Button");
        // leptos_dom::console_log(&format!(
        //     "Clicked on {:?} -> {:?}({}) size({sz})",
        //     cloned,
        //     if let Some(guts) = cloned.upgrade() {
        //         format!("{guts:?}")
        //     } else {
        //         "No stuff".to_string()
        //     },
        //     cloned.strong_count(),
        // ));
    });

    view! { cx,
        <button on:click=move |e| { on_click.with_value(|on_click| on_click(e)) }>"Bam"</button>
        <p>{&format!("Scope {cx:?}")}</p>
        <p>{&format!("Some data string -> {}", some_data.data.clone())}</p>
    }

    // ω <fn dispose_test>
}

// α <mod-def dispose_test>

#[derive(Debug)]
struct SomeData {
    data: String,
    pad: [u32; 1024 * 16],
}

impl SomeData {
    fn new(message: &str) -> Self {
        leptos_dom::console_log(&format!("Creating SomeData(`{message}`)"));
        Self {
            data: message.to_string(),
            pad: [1; 1024 * 16],
        }
    }
}

impl Drop for SomeData {
    fn drop(&mut self) {
        leptos_dom::console_log(&format!("Dropping SomeData(`{}`)", self.data));
    }
}

// ω <mod-def dispose_test>
