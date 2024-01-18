//! Module for histogram_component leptos function/component

////////////////////////////////////////////////////////////////////////////////////
// --- module uses ---
////////////////////////////////////////////////////////////////////////////////////
use leptos::component;
use leptos::view;
#[allow(unused_imports)]
use leptos::IntoAttribute;
use leptos::IntoView;
use leptos::MaybeSignal;
use leptos::RwSignal;
use leptos::View;
use plus_utils::HistogramEntry;
use plus_utils::HistogramPlot;
use plus_utils::HistogramSpans;

////////////////////////////////////////////////////////////////////////////////////
// --- functions ---
////////////////////////////////////////////////////////////////////////////////////
/// Given a set of sorted floating point value, creates a
/// histogram plot.
///
///   * **plot_label** - Label for the plot
///   * **num_bins** - Number of bins to display in the histogram
///   * **entries** - Values **to add to the plot**
///   * **legend_label_maker** - Function to make labels for the specified values in legend
///   * **table_label_maker** - Function to make labels for the specified values in table
///   * **selected_id** - The id selected point to draw attention to - `None` means no highlighting
///   * **descriptive_points** - Additional _description points_ to add to the table.
/// The table captures basic statistic points (min, q1, q2,...).
/// This allows adding specific id points to the table where the
/// `u32` represents the id and the [String] is the text.
///   * _return_ - View for histogram_component
#[component]
pub fn HistogramComponent<LLM, TLM>(
    /// Label for the plot
    #[prop(default=MaybeSignal::Static(String::default()))]
    plot_label: MaybeSignal<String>,
    /// Number of bins to display in the histogram
    #[prop(default=MaybeSignal::Static(75))]
    num_bins: MaybeSignal<u32>,
    /// Values **to add to the plot**
    entries: MaybeSignal<Vec<HistogramEntry>>,
    /// Function to make labels for the specified values in legend
    legend_label_maker: LLM,
    /// Function to make labels for the specified values in table
    table_label_maker: TLM,
    /// The id selected point to draw attention to - `None` means no highlighting
    #[prop(default=None)]
    selected_id: Option<RwSignal<u32>>,
    /// Additional _description points_ to add to the table.
    /// The table captures basic statistic points (min, q1, q2,...).
    /// This allows adding specific id points to the table where the
    /// `u32` represents the id and the [String] is the text.
    #[prop(default=Vec::new())]
    descriptive_points: Vec<(u32, String)>,
) -> impl IntoView
where
    LLM: Fn(f64) -> View + Copy + 'static,
    TLM: Fn(f64) -> View + Copy + 'static,
{
    pub const SELF_CLASS: &str = "plus-hist";
    let component_id = crate::component_id!("`HistogramComponent`");
    #[cfg(debug_assertions)]
    crate::log_component!(crate::COMPONENT_LOG_LEVEL, component_id);
    // α <fn histogram_component>

    use crate::ClientCssClasses;
    use crate::SliderWithNumericInput;
    use crate::Updatable;
    use leptos::create_signal;
    use leptos::store_value;
    use leptos::IntoAttribute;
    use leptos::SignalGet;
    use leptos::SignalGetUntracked;
    use leptos::SignalSet;
    use leptos::SignalWith;

    let histogram_plot =
        HistogramPlot::new(HistogramSpans::with_defaults(), num_bins.get_untracked());

    let histogram_plot_stored_value = store_value(histogram_plot);
    let (histogram_updated_read, histogram_updated_write) = create_signal(());

    let histogram_plot_points = move || {
        histogram_plot_stored_value.update_value(|histogram_plot| {
            let mut sorted_entries = entries.get();
            sorted_entries.sort_by(|a, b| a.value.partial_cmp(&b.value).unwrap());
            histogram_plot.add_sorted_values(sorted_entries.iter());
        });
        let points = histogram_plot_stored_value.with_value(|histogram_plot| {
            let num_points = histogram_plot.sorted_points.len();
            let calculated_mean = histogram_plot
                .sorted_points
                .iter()
                .map(|p| p.value)
                .sum::<f64>()
                / num_points as f64;
            let mut median_found = false;
            let mut mean_found = false;

            histogram_plot
                .sorted_points
                .iter()
                .map(|plot_point| {
                    let percentile = (100.0 * plot_point.percentile) as u32;
                    let quartile = match percentile {
                        _ if plot_point.id == 0 => ClientCssClasses::MsscGmf.as_str(),
                        _ if !mean_found && plot_point.value > calculated_mean => {
                            mean_found = true;
                            ClientCssClasses::MsscMean.as_str()
                        }
                        percentile if percentile <= 25 => ClientCssClasses::MsscCircle1.as_str(),
                        percentile if percentile < 50 => ClientCssClasses::MsscCircle2.as_str(),
                        _ if !median_found => {
                            median_found = true;
                            ClientCssClasses::MsscMedian.as_str()
                        }
                        percentile if percentile <= 75 => ClientCssClasses::MsscCircle3.as_str(),
                        _ => ClientCssClasses::MsscCircle4.as_str(),
                    };

                    view! {
                        <circle
                            class=quartile
                            r=histogram_plot.circle_r_pct
                            cx=plot_point.location.x
                            cy=plot_point.location.y
                        ></circle>
                    }
                    .into_view()
                })
                .collect::<Vec<_>>()
        });

        histogram_updated_write.set(());

        points
    };

    let pointer_transform = move || {
        histogram_plot_stored_value.with_value(|histogram_plot| {
            selected_id.map(|selected_id| {
                let translate = histogram_plot.get_pointer_translation(selected_id.get());
                format!("translate({}, {})", translate.x, translate.y)
            })
        })
    };

    let selected_point_row = move || {
        selected_id.map(|selected_id| {
            histogram_plot_stored_value.with_value(|histogram_plot| {
                let &selected_point_index =
                    histogram_plot.id_to_index.get(&selected_id.get()).unwrap();
                let plot_point = histogram_plot
                    .sorted_points
                    .get(selected_point_index)
                    .unwrap();

                view! {
                    <tr class=ClientCssClasses::HistSelectedRow.as_str()>
                        <td>"Selected"</td>
                        <td class="numeric">{plot_point.id}</td>
                        <td class="numeric">{selected_point_index}</td>
                        <td class="numeric">{table_label_maker(plot_point.value)}</td>
                    </tr>
                }
            })
        })
    };

    let descriptive_table = {
        move || {
            histogram_updated_read.track();
            histogram_plot_stored_value.with_value({
                |histogram_plot| {
                    use plus_utils::DescriptivePoint;
                    let mut rows = vec![
                        (
                            "Min",
                            histogram_plot.get_descriptive_point(DescriptivePoint::Min),
                        ),
                        (
                            "Q1",
                            histogram_plot.get_descriptive_point(DescriptivePoint::Q1),
                        ),
                        (
                            "Q2",
                            histogram_plot.get_descriptive_point(DescriptivePoint::Q2),
                        ),
                        (
                            "Q3",
                            histogram_plot.get_descriptive_point(DescriptivePoint::Q3),
                        ),
                        (
                            "Max",
                            histogram_plot.get_descriptive_point(DescriptivePoint::Max),
                        ),
                        (
                            "Mean",
                            histogram_plot.get_descriptive_point(DescriptivePoint::Mean),
                        ),
                    ];

                    for (index, label) in descriptive_points.iter() {
                        let found = histogram_plot
                            .sorted_points
                            .iter()
                            .enumerate()
                            .find_map(|(position, plot_point)| {
                                if plot_point.id == *index {
                                    Some((position as u32, plot_point))
                                } else {
                                    None
                                }
                            })
                            .unwrap();

                        rows.push((&label, found));
                    }

                    rows.sort_by(|(_, (_, a)), (_, (_, b))| a.value.partial_cmp(&b.value).unwrap());

                    let rows = rows
                        .iter()
                        .map({
                            |(label, (position, plot_point))| {
                                view! {
                                    <tr>
                                        <td>{label.to_string()}</td>
                                        <td class="numeric">{plot_point.id}</td>
                                        <td class="numeric">{*position}</td>
                                        <td class="numeric">
                                            {table_label_maker(plot_point.value)}
                                        </td>
                                    </tr>
                                }
                                .into_view()
                            }
                        })
                        .collect::<Vec<_>>();

                    view! {
                        <table class="content-table">
                            <caption class="tbl-caption">"Descriptive Distribution Points"</caption>
                            <thead>
                                <tr>
                                    <th>"Point"</th>
                                    <th class="numeric">"Forecast Id"</th>
                                    <th class="numeric">"Position"</th>
                                    <th class="numeric">"Value"</th>
                                </tr>
                            </thead>
                            <tbody>{rows} {selected_point_row}</tbody>
                        </table>
                    }
                }
            })
        }
    };

    // ω <fn histogram_component>
    view! {
        <div class=SELF_CLASS>
            // α <plus-hist-view>

            <div class=ClientCssClasses::Title.as_str()>{plot_label}</div>

            <SliderWithNumericInput
                updatable=Updatable::new(
                    0.0,
                    move |new_ordinal_id| {
                        let position = *new_ordinal_id as usize;
                        if let Some(selected_id) = selected_id {
                            let id = histogram_plot_stored_value
                                .with_value(|histogram_plot| {
                                    histogram_plot.sorted_points[position].id
                                });
                            selected_id.set(id);
                        }
                        tracing::debug!("ordinal slider input updated -> {new_ordinal_id}");
                    },
                )

                label=MaybeSignal::Static("Position".into())
                slider_id="position-id".to_string()
                range=0.0..=999.0
                no_decimal=true
            />

            <div class=ClientCssClasses::HistPair.as_str()>
                <div>
                    <svg viewBox="0 0 100 100">
                        {histogram_plot_points}
                        <g
                            transform=pointer_transform
                            inner_html=histogram_plot_stored_value
                                .with_value(|histogram_plot| histogram_plot.get_pointer_element())
                        ></g>
                    </svg>
                </div>
                <div>{descriptive_table}</div>
            </div>

        // ω <plus-hist-view>
        </div>
    }
}

// α <mod-def histogram_component>
// ω <mod-def histogram_component>
