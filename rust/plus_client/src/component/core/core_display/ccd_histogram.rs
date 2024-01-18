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
    let component_id = crate::component_id!("`CcdHistogram`");
    #[cfg(debug_assertions)]
    crate::log_component!(crate::COMPONENT_LOG_LEVEL, component_id);
    // α <fn ccd_histogram>

    use crate::utils::block_time::BlockTime;
    use crate::ClientCssClasses;
    use crate::HistogramComponent;
    use crate::SliderWithNumericInput;
    use leptos::create_local_resource;
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

    let normal_label = move || format!("Normal N(10%, 20%) {}", add_entries_read.get() * 1000);
    let lognormal_label =
        move || format!("Lognormal N(10%, 20%) {}", add_entries_read.get() * 1000);

    let normal_entries = move || async move {
        let delay_millis = delay_millis_read.get_untracked();
        let _timing = BlockTime::new(&format!("Adding lognormal points delay -> {delay_millis}"));
        let mut lognormal_entries = LognormalEntries::spawner().spawn("...");
        lognormal_entries.run(delay_millis).await
    };

    let lognormal_entries = move || async move {
        let delay_millis = delay_millis_read.get_untracked();
        let _timing = BlockTime::new(&format!("Adding lognormal points delay -> {delay_millis}"));
        let mut normal_entries = NormalEntries::spawner().spawn("...");
        normal_entries.run(delay_millis).await
    };

    let normal_id_rw = create_rw_signal(0u32);
    let lognormal_id_rw = create_rw_signal(0u32);

    let calc_resource = create_local_resource(
        move || add_entries_read.get(),
        move |read_count| async move { (normal_entries().await, lognormal_entries().await) },
    );

    let views = move || {
        reset_read.track();
        add_entries_write.set(1);
        if let Some((normal_entries, lognormal_entries)) = calc_resource.get() {
            view! {
                <div class=ClientCssClasses::ChHists.as_str()>
                    <div class=ClientCssClasses::ChNormal.as_str()>
                        <HistogramComponent
                            plot_label=MaybeSignal::Dynamic(Signal::derive(normal_label))
                            entries=MaybeSignal::Static(normal_entries)
                            legend_label_maker=label_maker
                            table_label_maker=label_maker
                            selected_id=Some(normal_id_rw)
                        />
                    </div>

                    <div class=ClientCssClasses::ChLognormal.as_str()>
                        <HistogramComponent
                            plot_label=MaybeSignal::Dynamic(Signal::derive(lognormal_label))
                            entries=MaybeSignal::Static(lognormal_entries)
                            legend_label_maker=label_maker
                            table_label_maker=label_maker
                            selected_id=Some(lognormal_id_rw)
                        />
                    </div>
                </div>
            }
            .into_view()
        } else {
            view! { <h3>"Running simulation..."</h3> }.into_view()
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
                use rand::Rng;
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
            } else {
                let _ = delay_millis;
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
        Self {
            distribution,
            thread_rng,
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////
// --- trait impls ---
////////////////////////////////////////////////////////////////////////////////////
impl Worker for NormalSeries {
    type Message = i32;

    type Input = i32;

    type Output = Vec<HistogramEntry>;
    /// Create the worker
    ///
    ///   * **scope** - The scope of the worker
    ///   * _return_ - The created worker
    fn create(scope: &WorkerScope<Self>) -> Self {
        // α <fn Worker::create for NormalSeries>
        todo!("Implement `create`")
        // ω <fn Worker::create for NormalSeries>
    }

    /// Worker receives an update
    ///
    ///   * **scope** - The scope of the worker
    ///   * **msg** - TODO Document Param(msg)
    fn update(&mut self, scope: &WorkerScope<Self>, msg: Self::Message) {
        // α <fn Worker::update for NormalSeries>
        todo!("Implement `update`")
        // ω <fn Worker::update for NormalSeries>
    }

    /// Receives an input from a connected bridge
    ///
    ///   * **scope** - The scope of the worker
    ///   * **msg** - TODO Document Param(msg)
    ///   * **id** - The handler id
    fn received(&mut self, scope: &WorkerScope<Self>, msg: Self::Input, id: HandlerId) {
        // α <fn Worker::received for NormalSeries>
        todo!("Implement `received`")
        // ω <fn Worker::received for NormalSeries>
    }
}

// α <mod-def ccd_histogram>

use crate::utils::block_time::BlockTime;
use gloo_worker::oneshot::oneshot;
use gloo_worker::Spawnable;
use gloo_worker::{HandlerId, Worker, WorkerScope};
pub use plus_utils::HistogramEntry;

#[oneshot]
async fn LognormalEntries(delay_millis: f64) -> Vec<HistogramEntry> {
    tracing::warn!("RUNNING LOGNORMAL ENTRIES delay({delay_millis})");
    let _timing = BlockTime::new(&format!("Adding lognormal points delay -> {delay_millis}"));
    let mut lognormal_generator = NormalSeries::new(
        NormalBasedDist::LognormalDist {
            distribution: LogNormal::new(0.1, 0.2).unwrap(),
        },
        rand::thread_rng(),
    );
    (0..1_000)
        .map(move |i| {
            HistogramEntry::new(i, 100_000.0 * lognormal_generator.next_value(delay_millis))
        })
        .collect::<Vec<_>>()
}

#[oneshot]
async fn NormalEntries(delay_millis: f64) -> Vec<HistogramEntry> {
    tracing::warn!("RUNNING NORMAL ENTRIES delay({delay_millis})");
    let _timing = BlockTime::new(&format!("Adding normal points delay -> {delay_millis}"));
    let mut normal_generator = NormalSeries::new(
        NormalBasedDist::NormalDist {
            distribution: Normal::new(0.1, 0.2).unwrap(),
        },
        rand::thread_rng(),
    );
    (0..1_000)
        .map(move |i| HistogramEntry::new(i, 100_000.0 * normal_generator.next_value(delay_millis)))
        .collect::<Vec<_>>()
}

// ω <mod-def ccd_histogram>
