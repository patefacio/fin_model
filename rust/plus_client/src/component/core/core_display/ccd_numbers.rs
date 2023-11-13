//! Module for ccd_numbers leptos function/component

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
/// Show various number inputs
///
///   * **show_update** - Function to display state updates
///   * _return_ - View for ccd_numbers
#[component]
pub fn CcdNumbers(
    /// Function to display state updates
    show_update: WriteSignal<String>,
) -> impl IntoView {
    pub const SELF_CLASS: &str = "plus-cn; ccd-section";
    crate::log_component!("`CcdNumbers`");
    // α <fn ccd_numbers>

    use crate::IntegerInput;
    use crate::Modification;
    use crate::NumericInput;
    use crate::PercentInput;
    use crate::Updatable;
    use leptos::MaybeSignal;
    use leptos::SignalSet;

    // ω <fn ccd_numbers>
    view! {
        <div class=SELF_CLASS>
            // α <plus-cn-view>

            <div class="title">"Numbers"</div>
            <div class="ccd-numbers">
                <div style="padding: 1em;">
                    <h4>"Numeric Input Range(-5.0,5.0)"</h4>
                    <p>"Models a single floating point number."</p>
                    <h5>"Special Features"</h5>
                    <p inner_html="
                    <ul>
                    <li>Auto commify</li>
                    <li>Special Characters (e.g. type in `2.5k`)</li>
                    <ul>
                    <li>'k' -> Convert to thousands</li>
                    <li>'m' -> Convert to millions</li>
                    <li>'b' -> Convert to billions</li>
                    </ul>
                    <li>Up/Down Arrow Increment/Decrement digit to left</li>
                    <li>Support for <em>Prefix</em> and/or <em>Suffix</em> (See <em>Percent</em> 
                    for suffix example and <em>NormalSpec</em> for prefix and suffix)
                    </li>
                    <li>Specify Range</li>
                    <li>max_len: maps to html attribute <em>maxlength</em></li>
                    <li>size: maps to html attribute <em>size</em></li>
                    </ul>
                    "></p>
                    <NumericInput
                        updatable=Updatable::new(
                            Some(32.3),
                            move |n| { show_update.set(format!("Number updated -> {n:?}")) },
                        )

                        range=Some(-5.0..=5.0)
                        placeholder="temp"
                        size=13
                    />
                </div>
                <div style="padding: 1em;">
                    <div>
                        <h4>"Numeric Input With Prefix"</h4>
                        <p inner_html="
                        Provides a <em>NumericInput</em> with <em>prefix</em>.
                        "></p>
                        <NumericInput
                            updatable=Updatable::new(
                                None,
                                move |n| { show_update.set(format!("Input updated -> {n:?}")) },
                            )

                            modification=Some(
                                Modification::Prefix(MaybeSignal::Static("$ ".to_string())),
                            )

                            placeholder="dollars"
                            size=12
                        />
                    </div>
                    <div>
                        <h4>"Numeric Input With Prefix Unicode"</h4>
                        <p inner_html="
                        Provides a <em>NumericInput</em> with <em>prefix</em>.
                        "></p>
                        <NumericInput
                            updatable=Updatable::new(
                                None,
                                move |n| { show_update.set(format!("Input updated -> {n:?}")) },
                            )

                            modification=Some(
                                Modification::Prefix(MaybeSignal::Static("€ ".to_string())),
                            )

                            placeholder="euros"
                            size=12
                        />
                    </div>
                    <div>
                        <h4>"Numeric Input Prefix & Suffix RangeInclusive(0 to 5,000)"</h4>
                        <p inner_html="
                        Provides a <em>NumericInput</em> with <em>prefix</em> and <em>suffix</em>.
                        "></p>
                        <NumericInput
                            updatable=Updatable::new(
                                None,
                                move |n| { show_update.set(format!("Input updated -> {n:?}")) },
                            )

                            modification=Some(Modification::PrefixAndSuffix {
                                prefix: "€ ".into(),
                                suffix: "/yr".into(),
                            })

                            placeholder="expense/yr"
                            range=Some(0.0..=5_000.0)
                            max_len=14
                            size=12
                        />
                    </div>
                    <div>
                        <h4>"Numeric Input (Valid when 42.0)"</h4>
                        <p inner_html="
                        Provides a <em>NumericInput</em> which is only valid via custom validator requiring value to be 42.
                        "></p>
                        <NumericInput
                            updatable=Updatable::new(
                                None,
                                move |n| { show_update.set(format!("Input updated -> {n:?}")) },
                            )

                            validator=Some(Box::new(|v| v == 42.0))
                            max_len=14
                            size=12
                        />
                    </div>
                </div>
                <div style="padding: 1em;">
                    <div>
                        <h4>"Integer Input"</h4>
                        <p inner_html="Models a single integer with similar features to <em>Numeric Input</em> without decimals.
                        <ul>
                        <li>Special characters ('k', 'm', 'b')</li>
                        <li>Optional commify</li>
                        </ul>
                        "></p>
                        <IntegerInput
                            updatable=Updatable::new(
                                None,
                                move |n| { show_update.set(format!("Input updated -> {n:?}")) },
                            )

                            placeholder="Integer"
                            range=Some(0..=5000)
                        />
                    </div>
                    <div>
                        <h4>"Integer Input With Validator"</h4>
                        <p inner_html="Integer input with validator requiring value to be even"></p>
                        <IntegerInput
                            updatable=Updatable::new(
                                None,
                                move |n| { show_update.set(format!("Input updated -> {n:?}")) },
                            )

                            placeholder="Integer"
                            validator=Some(Box::new(|v| v % 2 == 0))
                        />
                    </div>
                    <div>
                        <h4 inner_html="Percent Input (i.e. suffix `%`) <strong>max_len=8</strong> RangeInclusive(0% to 40%)"></h4>
                        <p inner_html="
                        Provides a <em>NumericInput</em> with a percent suffix modification.
                        "></p>
                        <PercentInput
                            updatable=Updatable::new(
                                Some(0.0315),
                                move |n| { show_update.set(format!("Percent updated -> {n:?}")) },
                            )

                            placeholder="pct complete"
                            max_len=8
                            range=Some(0.0..=0.4)
                        />
                    </div>
                </div>
            </div>

        // ω <plus-cn-view>
        </div>
    }
}

// α <mod-def ccd_numbers>
// ω <mod-def ccd_numbers>
