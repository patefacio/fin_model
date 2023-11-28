//! Module for ccd_histogram leptos function/component

////////////////////////////////////////////////////////////////////////////////////
// --- module uses ---
////////////////////////////////////////////////////////////////////////////////////
use crate::Updatable;
use leptos::component;
use leptos::view;
#[allow(unused_imports)]
use leptos::IntoAttribute;
use leptos::IntoView;
use leptos::WriteSignal;
use rand::rngs::ThreadRng;
use rand::Rng;
use rand_distr::Distribution;
use rand_distr::LogNormal;
use rand_distr::Normal;

////////////////////////////////////////////////////////////////////////////////////
// --- enums ---
////////////////////////////////////////////////////////////////////////////////////
/// Enumerates normal or log-normal
pub enum NormalBasedDist {
    /// A normal distribution
    NormalDist {
        /// Normal distribution
        distribution: Normal<f64>,
    },
    /// A lognormal distribution
    LognormalDist {
        /// Lognormal distribution
        distribution: LogNormal<f64>,
    },
}

////////////////////////////////////////////////////////////////////////////////////
// --- structs ---
////////////////////////////////////////////////////////////////////////////////////
/// Mimics generator for normal distribution data
pub struct NormalSeries {
    /// Type of distribution
    pub distribution: NormalBasedDist,
    /// Random number generator
    pub thread_rng: ThreadRng,
}

////////////////////////////////////////////////////////////////////////////////////
// --- functions ---
////////////////////////////////////////////////////////////////////////////////////
/// Show the [HistogramComponent]
///
///   * **show_update** - Function to display state updates
///   * _return_ - View for ccd_histogram
#[component]
pub fn CcdHistogram(
    /// Function to display state updates
    show_update: WriteSignal<String>,
) -> impl IntoView {
    pub const SELF_CLASS: &str = "plus-ch; ccd-section";
    crate::log_component!("`CcdHistogram`");
    // α <fn ccd_histogram>

    use crate::utils::block_time::BlockTime;
    use crate::CssClasses;
    use crate::HistogramComponent;
    use crate::SliderWithNumericInput;
    use leptos::create_rw_signal;
    use leptos::create_signal;
    use leptos::IntoAttribute;
    use leptos::MaybeSignal;
    use leptos::Signal;
    use leptos::SignalGet;
    use leptos::SignalGetUntracked;
    use leptos::SignalSet;
    use leptos::SignalUpdate;
    use leptos::SignalWith;
    use plus_lookup::CurrencyValue;
    use plus_modeled::Currency;
    use plus_utils::HistogramEntry;

    let label_maker = |v: f64| {
        CurrencyValue::new(Currency::Usd, v.round())
            .to_string()
            .into_view()
    };

    let (add_entries_read, add_entries_write) = create_signal(1);
    let (reset_read, reset_write) = create_signal(());
    let (delay_millis_read, delay_millis_write) = create_signal(0.0);

    let normal_label = move || {
        format!(
            "Normal N(10%, 20%) {}",
            add_entries_read.get() * 1000
        )
    };
    let lognormal_label = move || {
        format!(
            "Lognormal N(10%, 20%) {}",
            add_entries_read.get() * 1000
        )
    };

    let normal_entries = move || {
        let delay_millis = delay_millis_read.get_untracked();
        let _timing = BlockTime::new(&format!("Adding normal points delay -> {delay_millis}"));
        add_entries_read.track();
        let mut normal_generator = NormalSeries::new(
            NormalBasedDist::NormalDist {
                distribution: Normal::new(0.1, 0.2).unwrap(),
            },
            rand::thread_rng(),
        );
        (0..1_000)
            .map(move |i| {
                HistogramEntry::new(
                    i,
                    100_000.0 * normal_generator.next_value(delay_millis_read.get_untracked()),
                )
            })
            .collect::<Vec<_>>()
    };

    let lognormal_entries = move || {
        let delay_millis = delay_millis_read.get_untracked();
        let _timing = BlockTime::new(&format!("Adding lognormal points delay -> {delay_millis}"));
        add_entries_read.track();
        let mut lognormal_generator = NormalSeries::new(
            NormalBasedDist::LognormalDist {
                distribution: LogNormal::new(0.1, 0.2).unwrap(),
            },
            rand::thread_rng(),
        );
        (0..1_000)
            .map(move |i| {
                HistogramEntry::new(
                    i,
                    100_000.0 * lognormal_generator.next_value(delay_millis_read.get_untracked()),
                )
            })
            .collect::<Vec<_>>()
    };

    let normal_id_rw = create_rw_signal(0u32);
    let lognormal_id_rw = create_rw_signal(0u32);

    let views = move || {
        reset_read.track();
        add_entries_write.set(1);

        view! {
            <div class=CssClasses::ChHists.as_str()>
                <div class=CssClasses::ChNormal.as_str()>
                    <HistogramComponent
                        plot_label=MaybeSignal::Dynamic(Signal::derive(normal_label))
                        entries=MaybeSignal::Dynamic(Signal::derive(normal_entries))
                        legend_label_maker=label_maker
                        table_label_maker=label_maker
                        selected_id=Some(normal_id_rw)
                    />
                </div>

                <div class=CssClasses::ChLognormal.as_str()>
                    <HistogramComponent
                        plot_label=MaybeSignal::Dynamic(Signal::derive(lognormal_label))
                        entries=MaybeSignal::Dynamic(Signal::derive(lognormal_entries))
                        legend_label_maker=label_maker
                        table_label_maker=label_maker
                        selected_id=Some(lognormal_id_rw)
                    />
                </div>
            </div>
        }
    };

    // ω <fn ccd_histogram>
    view! {
        <div class=SELF_CLASS>
            // α <plus-ch-view>

            <SliderWithNumericInput
                updatable=Updatable::new(
                    0.0,
                    move |new_value| {
                        delay_millis_write.set(*new_value);
                        show_update.set(format!("Point fn delay -> {new_value:?}"));
                    },
                )

                label=MaybeSignal::Static("Delay MS".into())
                slider_id="ccd-sldr-delay".into()
                range=0.0..=5.0
                step=1.0
            />

            <button on:click=move |_| {
                add_entries_write.update(|u| *u += 1)
            }>"Add 1,000 Points"</button>

            <button on:click=move |_| reset_write.set(())>"Reset To 1,000"</button>

            {views}

        // ω <plus-ch-view>
        </div>
    }
}

////////////////////////////////////////////////////////////////////////////////////
// --- type impls ---
////////////////////////////////////////////////////////////////////////////////////
impl NormalSeries {
    /// Return the next value
    ///
    ///   * **delay_millis** - Approximate time to take on this function
    ///   * _return_ - Next value
    pub fn next_value(&mut self, delay_millis: f64) -> f64 {
        // α <fn NormalSeries::next_value>

        cfg_if::cfg_if! {
            if #[cfg(any(feature = "csr", feature = "hydrate"))] {
                let start_time = instant::now();
                let mut elapsed = 0.0;

                use rand_distr::Uniform;

                let mut loop_count = 0u64;
                while elapsed < delay_millis {
                    let mut n = 0;
                    while n < 150 {
                        n = self.thread_rng.sample(Uniform::new(1, 500));
                    }
                    elapsed = instant::now() - start_time;
                    loop_count += 1;
                }

                tracing::debug!("Simulated delay -> {elapsed:?} in {loop_count} iterations");
            }
        }

        match &self.distribution {
            NormalBasedDist::NormalDist { distribution } => {
                distribution.sample(&mut self.thread_rng)
            }
            NormalBasedDist::LognormalDist { distribution } => {
                distribution.sample(&mut self.thread_rng)
            }
        }

        // ω <fn NormalSeries::next_value>
    }

    /// Initializer
    ///
    ///   * **distribution** - Type of distribution
    ///   * **thread_rng** - Random number generator
    ///   * _return_ - The constructed instance
    pub fn new(distribution: NormalBasedDist, thread_rng: ThreadRng) -> Self {
        // α <new initialization>
        // ω <new initialization>
        Self {
            distribution,
            thread_rng,
        }
    }
}

/// Unit tests for `ccd_histogram`
#[cfg(test)]
pub mod unit_tests {

    /// Test type NormalSeries
    mod test_normal_series {
        ////////////////////////////////////////////////////////////////////////////////////
        // --- module uses ---
        ////////////////////////////////////////////////////////////////////////////////////
        use test_log::test;

        ////////////////////////////////////////////////////////////////////////////////////
        // --- functions ---
        ////////////////////////////////////////////////////////////////////////////////////
        #[test]
        fn next_value() {
            // α <fn test NormalSeries::next_value>
            todo!("Test next_value")
            // ω <fn test NormalSeries::next_value>
        }

        // α <mod-def test_normal_series>
        // ω <mod-def test_normal_series>
    }

    // α <mod-def unit_tests>
    // ω <mod-def unit_tests>
}

// α <mod-def ccd_histogram>
// ω <mod-def ccd_histogram>
