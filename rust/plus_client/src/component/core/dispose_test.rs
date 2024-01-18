//! Module for dispose_test leptos function/component

////////////////////////////////////////////////////////////////////////////////////
// --- module uses ---
////////////////////////////////////////////////////////////////////////////////////
use leptos::component;
use leptos::view;
use leptos::IntoView;

////////////////////////////////////////////////////////////////////////////////////
// --- functions ---
////////////////////////////////////////////////////////////////////////////////////
/// See what gets cleaned up.
///
///   * _return_ - View for dispose_test
#[component]
pub fn DisposeTest() -> impl IntoView {
    let component_id = crate::component_id!("`DisposeTest`");
    #[cfg(debug_assertions)]
    crate::log_component!(crate::COMPONENT_LOG_LEVEL, component_id);
    // α <fn dispose_test>

    use leptos::*;
    use std::rc::Rc;

    let some_data = Rc::new(SomeData::new(&format!("DisposeTest Badabing")));

    /*
        let (some_data, some_data_write) = leptos::create_signal(
           SomeData::new(&format!("DisposeTest Badabing"))
        );
    */
    let do_stuff = move || {
        tracing::debug!("Doing stuff");
    };

    let _do_stuff = leptos::store_value(do_stuff);

    // leptos_dom::console_log(&format!(
    //     "Size of read signal is {}, size of write signal is {}",
    //     std::mem::size_of_val(&some_data),
    //     std::mem::size_of_val(&some_data_write),
    // ));

    tracing::debug!("DisposeTest");
    let log_dispose_item = crate::utils::log_dispose::LogDispose::new("dIsPoSe".into());

    let on_click = leptos::store_value(move |_| {
        println!("{log_dispose_item:?}");
        tracing::debug!("Clicked Bam Button");
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

    view! {
        <button on:click=move |e| { on_click.with_value(|on_click| on_click(e)) }>"Bam"</button>
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
        tracing::debug!("Creating SomeData(`{message}`)");
        tracing::debug!("Creating some data again!");
        Self {
            data: message.to_string(),
            pad: [1; 1024 * 16],
        }
    }
}

impl Drop for SomeData {
    fn drop(&mut self) {
        tracing::debug!(
            "Dropping SomeData(`{}`) with pad size {}",
            self.data,
            self.pad.len()
        );
    }
}

// ω <mod-def dispose_test>
