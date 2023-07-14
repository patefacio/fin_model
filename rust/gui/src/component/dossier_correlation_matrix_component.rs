//! Module for dossier_correlation_matrix_component leptos function/component

////////////////////////////////////////////////////////////////////////////////////
// --- module uses ---
////////////////////////////////////////////////////////////////////////////////////
use crate::Updatable;
#[allow(unused_imports)]
use leptos::log;
use leptos::{component, view, IntoView, Scope};
#[allow(unused_imports)]
use leptos_dom::console_log;
use plus_modeled::DossierCorrelationMatrix;

////////////////////////////////////////////////////////////////////////////////////
// --- functions ---
////////////////////////////////////////////////////////////////////////////////////
/// Models a correlation matrix where all pair-wise indices are represented as keys.
///
///   * **cx** - Context
///   * **updatable** - The [DossierCorrelationMatrix] being edited
///   * _return_ - View for dossier_correlation_matrix_component
#[component]
pub fn DossierCorrelationMatrixComponent(
    /// Context
    cx: Scope,
    /// The [DossierCorrelationMatrix] being edited
    updatable: Updatable<DossierCorrelationMatrix>,
) -> impl IntoView {
    // α <fn dossier_correlation_matrix_component>

    use leptos::create_signal;
    use leptos::For;
    use leptos::IntoAttribute;
    use leptos::SignalGet;
    use plus_modeled::core::dossier_item_index::ItemIndex;

    //let index_pairs = updatable.value.mappings.iter();

    let (indices, _set_indices) = create_signal(cx, updatable.value.mappings);
    console_log(&format!("MATRIX WORKING"));

    view! { cx,
        <For
            each=move || indices.get()
            key=|entry| { format!("{entry:?}") }
            view=|cx, element| {
                let row_index = element
                    .row_index
                    .as_ref()
                    .map(|row_index| match row_index.item_index {
                        Some(ItemIndex::WorthIndex(w)) => w.to_string(),
                        Some(ItemIndex::FlowIndex(f)) => f.to_string(),
                        Some(ItemIndex::HoldingIndex(dhi)) => dhi.holding_index.unwrap().to_string(),
                        None => "Oops".to_string(),
                    })
                    .unwrap();
                let column_index = element
                    .column_index
                    .as_ref()
                    .map(|column_index| match column_index.item_index {
                        Some(ItemIndex::WorthIndex(w)) => w.to_string(),
                        Some(ItemIndex::FlowIndex(f)) => f.to_string(),
                        Some(ItemIndex::HoldingIndex(dhi)) => dhi.holding_index.unwrap().to_string(),
                        None => "Oops".to_string(),
                    })
                    .unwrap();
                let correlation = element.correlation;
                view! { cx,
                    <div style="display: inline-flex">
                        <div inner_html=format!("row: {row_index} [ ")></div>
                        <div inner_html=format!(" ] column: {column_index}, ")></div>
                        <div inner_html=format!("{correlation}")></div>
                    </div>
                    <hr/>
                }
            }
        />
        <h5>"The matrix"</h5>
    }

    // ω <fn dossier_correlation_matrix_component>
}

// α <mod-def dossier_correlation_matrix_component>

pub fn set_matrix_correlation(
    matrix: &mut DossierCorrelationMatrix,
    index: (u32, u32),
    correlation: f64,
) -> f64 {
    use plus_modeled::core::dossier_item_index::ItemIndex;
    use plus_modeled::core::DossierHoldingIndex;
    use plus_modeled::DossierCorrelationEntry;
    use plus_modeled::DossierItemIndex;

    for i in matrix.mappings.iter_mut() {
        let row_index = i
            .row_index
            .as_ref()
            .map(|row_index| match row_index.item_index {
                Some(ItemIndex::WorthIndex(w)) => w,
                Some(ItemIndex::FlowIndex(f)) => f,
                Some(ItemIndex::HoldingIndex(dhi)) => dhi.holding_index.unwrap(),
                None => 0,
            });
        let column_index =
            i.column_index
                .as_ref()
                .map(|column_index| match column_index.item_index {
                    Some(ItemIndex::WorthIndex(w)) => w,
                    Some(ItemIndex::FlowIndex(f)) => f,
                    Some(ItemIndex::HoldingIndex(dhi)) => dhi.holding_index.unwrap(),
                    None => 0,
                });

        if row_index.unwrap() == index.0 && column_index.unwrap() == index.1 {
            i.correlation = correlation;
            return i.correlation;
        }
    }
    let make_cor_entry = |row_holding_id, column_holding_id, correlation| DossierCorrelationEntry {
        row_index: Some(DossierItemIndex {
            item_index: Some(ItemIndex::HoldingIndex(DossierHoldingIndex {
                holding_index: Some(row_holding_id),
                ..Default::default()
            })),
        }),
        column_index: Some(DossierItemIndex {
            item_index: Some(ItemIndex::HoldingIndex(DossierHoldingIndex {
                holding_index: Some(column_holding_id),
                ..Default::default()
            })),
        }),
        correlation: correlation,
    };
    matrix
        .mappings
        .push(make_cor_entry(index.0, index.1, correlation));
    return correlation;
}

pub fn get_matrix_correlation(matrix: &DossierCorrelationMatrix, index: (u32, u32)) -> f64 {
    use plus_modeled::core::dossier_item_index::ItemIndex;
    //use plus_modeled::core::DossierHoldingIndex;
    //use plus_modeled::DossierCorrelationEntry;
    //use plus_modeled::DossierItemIndex;

    for i in matrix.mappings.iter() {
        let row_index = i
            .row_index
            .as_ref()
            .map(|row_index| match row_index.item_index {
                Some(ItemIndex::WorthIndex(w)) => w,
                Some(ItemIndex::FlowIndex(f)) => f,
                Some(ItemIndex::HoldingIndex(dhi)) => dhi.holding_index.unwrap(),
                None => 0,
            });
        let column_index =
            i.column_index
                .as_ref()
                .map(|column_index| match column_index.item_index {
                    Some(ItemIndex::WorthIndex(w)) => w,
                    Some(ItemIndex::FlowIndex(f)) => f,
                    Some(ItemIndex::HoldingIndex(dhi)) => dhi.holding_index.unwrap(),
                    None => 0,
                });

        if row_index.unwrap() == index.0 && column_index.unwrap() == index.1 {
            return i.correlation;
        }
    } /*
      let make_cor_entry = |row_holding_id, column_holding_id, correlation| DossierCorrelationEntry {
          row_index: Some(DossierItemIndex {
              item_index: Some(ItemIndex::HoldingIndex(DossierHoldingIndex {
                  holding_index: Some(row_holding_id),
                  ..Default::default()
              })),
          }),
          column_index: Some(DossierItemIndex {
              item_index: Some(ItemIndex::HoldingIndex(DossierHoldingIndex {
                  holding_index: Some(column_holding_id),
                  ..Default::default()
              })),
          }),
          correlation: correlation,
      };
      matrix.mappings.push(make_cor_entry(index.0, index.1, 0.0));
      */
    return 0.0;
}

#[component]
pub fn DisplayEntireMatrix(
    cx: Scope,
    updatable: Updatable<DossierCorrelationMatrix>,
) -> impl IntoView {
    use leptos::store_value;
    use plus_modeled::core::dossier_item_index::ItemIndex;
    //use plus_modeled::core::DossierHoldingIndex;
    //use plus_modeled::DossierCorrelationEntry;
    //use plus_modeled::DossierItemIndex;

    use leptos::create_signal;
    use leptos::For;
    use leptos::IntoAttribute;
    use leptos::SignalGet;

    let mut rows = vec![];
    let mut cols = vec![];

    let updatable = store_value(cx, updatable);
    updatable.with_value(|updatable| {
        for i in updatable.value.mappings.iter() {
            let row_index = i
                .row_index
                .as_ref()
                .map(|row_index| match row_index.item_index {
                    Some(ItemIndex::WorthIndex(w)) => w,
                    Some(ItemIndex::FlowIndex(f)) => f,
                    Some(ItemIndex::HoldingIndex(dhi)) => dhi.holding_index.unwrap(),
                    None => 0,
                })
                .unwrap();
            let column_index = i
                .column_index
                .as_ref()
                .map(|column_index| match column_index.item_index {
                    Some(ItemIndex::WorthIndex(w)) => w,
                    Some(ItemIndex::FlowIndex(f)) => f,
                    Some(ItemIndex::HoldingIndex(dhi)) => dhi.holding_index.unwrap(),
                    None => 0,
                })
                .unwrap();

            rows.push(row_index);
            cols.push(column_index);
        }
    });

    rows.sort();
    rows.dedup();
    cols.sort();
    cols.dedup();

    let (row_indices, _set_indices) = create_signal(cx, rows.clone());

    console_log(&format!("DISPLAY WORKING"));

    view! { cx,
        <div inner_html=format!("_ {cols:?}")></div>
        <For
            each=move || row_indices.get()
            key=|entry| { format!("{entry:?}") }
            view=move |cx, r_element| {
                let row_index = rows.get(r_element as usize).unwrap();
                let (col_indices, _set_indices) = create_signal(cx, cols.clone());
                view! { cx,
                    <div inner_html=format!("R({row_index:?})")></div>
                    <For
                        each=move || col_indices.get()
                        key=|entry| { format!("{entry:?}") }
                        view=move |cx, c_element| {
                            let column_index = updatable
                                .with_value(|updatable| {
                                    get_matrix_correlation(&updatable.value, (r_element, c_element))
                                });
                            view! { cx,
                                <div style="display: inline-flex">
                                    <div inner_html=format!("C({column_index})")></div>
                                </div>
                            }
                        }
                    />
                }
            }
        />
    }
}

// ω <mod-def dossier_correlation_matrix_component>
