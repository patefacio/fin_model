//! Module for normal_spec_component leptos function/component

////////////////////////////////////////////////////////////////////////////////////
// --- module uses ---
////////////////////////////////////////////////////////////////////////////////////
use crate::component::numeric_input::{Modification, NumericInput};
use crate::Updatable;
use leptos::{component, view, IntoView, Scope};
#[allow(unused_imports)]
use leptos_dom::console_log;
use plus_modeled::core::NormalSpec;

////////////////////////////////////////////////////////////////////////////////////
// --- functions ---
////////////////////////////////////////////////////////////////////////////////////
/// Models a normal specification -> N(mu, sigma).
///
///   * **cx** - Context
///   * **updatable** - The normal spec being edited
///   * _return_ - View for normal_spec_component
#[component]
pub fn NormalSpecComponent(
    /// Context
    cx: Scope,
    /// The normal spec being edited
    updatable: Updatable<Option<NormalSpec>>,
) -> impl IntoView {
    // α <fn normal_spec_component>

    use std::cell::RefCell;
    use std::rc::Rc;
    use leptos::IntoAttribute;

    let updatable = Rc::new(RefCell::new(updatable));

    let initial_mean = updatable
        .as_ref()
        .borrow()
        .value
        .as_ref()
        .map(|option_of_normal_spec| option_of_normal_spec.mean);

    let initial_std_dev = updatable
        .as_ref()
        .borrow()
        .value
        .as_ref()
        .map(|normal_spec| normal_spec.std_dev);

    let ns = if let Some(normal_spec) = updatable.as_ref().borrow().value.as_ref() {
        *normal_spec
    } else {
        NormalSpec {
            mean: 0.1,
            std_dev: 0.2,
        }
    };

    //use crate::utils::
    //let plot_options = ReturnDistPlotOptions

    let updatable_for_mean = Rc::clone(&updatable);
    let mean_updatable = Updatable::new(initial_mean, move |mean| {
        if let Some(mean) = mean.clone() {
            updatable_for_mean
                .as_ref()
                .borrow_mut()
                .update_and_then_signal(|normal_spec| {
                    let inner_value = if let Some(normal_spec) = normal_spec {
                        normal_spec.mean = mean;
                        *normal_spec
                    } else {
                        NormalSpec { mean, std_dev: 0.0 }
                    };

                    *normal_spec = Some(inner_value)
                });
        }
    });

    let std_dev_updatable = Updatable::new(initial_std_dev, move |std_dev| {
        if let Some(std_dev) = std_dev.clone() {
            updatable
                .as_ref()
                .borrow_mut()
                .update_and_then_signal(|normal_spec| {
                    let inner_value = if let Some(normal_spec) = normal_spec {
                        normal_spec.std_dev = std_dev;
                        *normal_spec
                    } else {
                        NormalSpec { mean: 0.0, std_dev }
                    };

                    *normal_spec = Some(inner_value)
                });
        }
    });

    view! {
    cx,

        <fieldset class="nsg">
        <legend>"Normal Growth"</legend>
        <div class="form">


            <div style="display: inline-flex" >
                "N("
                    <NumericInput
                        placeholder=Some("mean".to_string())
                        modification=Some(Modification::PrefixAndSuffix{
                            prefix: "μ=".into(),
                            suffix: "%".into()
                        })
                        non_negative=true
                        updatable=mean_updatable
                    />
                ", "
                    <NumericInput
                        placeholder=Some("std. dev".to_string())
                        modification=Some(Modification::PrefixAndSuffix{
                            prefix: "σ=".into(),
                            suffix: "%".into()
                        })
                        non_negative=true
                        updatable=std_dev_updatable
                    />
                ")"
            </div>
        </div>
        <div inner_html=foo() />

        </fieldset>
    }

    // ω <fn normal_spec_component>
}

// α <mod-def normal_spec_component>

fn foo() -> String {
    use plotters::prelude::*;
    let mut plot_buff = String::with_capacity(2 ^ 12);

    {
        let root_area = SVGBackend::with_string(&mut plot_buff, (1024, 768)).into_drawing_area();
        root_area.fill(&WHITE).unwrap();
        let root_area = root_area.titled("Image Title", ("sans-serif", 40)).unwrap();
        let (upper, lower) = root_area.split_vertically(512);
        let x_axis = (-3.4f32..3.4).step(0.1);

        let mut cc = ChartBuilder::on(&upper)
            .margin(5)
            .set_all_label_area_size(50)
            .caption("Sine and Cosine", ("sans-serif", 40))
            .build_cartesian_2d(-3.4f32..3.4, -1.2f32..1.2f32)
            .unwrap();

        cc.configure_mesh()
            .x_labels(20)
            .y_labels(10)
            .disable_mesh()
            .x_label_formatter(&|v| format!("{:.1}", v))
            .y_label_formatter(&|v| format!("{:.1}", v))
            .draw()
            .unwrap();

        cc.draw_series(LineSeries::new(x_axis.values().map(|x| (x, x.sin())), &RED))
            .unwrap()
            .label("Sine")
            .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));

        cc.draw_series(LineSeries::new(
            x_axis.values().map(|x| (x, x.cos())),
            &BLUE,
        ))
        .unwrap()
        .label("Cosine")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &BLUE));

        cc.configure_series_labels()
            .border_style(&BLACK)
            .draw()
            .unwrap();

        /*
        // It's possible to use a existing pointing element
         cc.draw_series(PointSeries::<_, _, Circle<_>>::new(
            (-3.0f32..2.1f32).step(1.0).values().map(|x| (x, x.sin())),
            5,
            Into::<ShapeStyle>::into(&RGBColor(255,0,0)).filled(),
        ))?;*/

        // Otherwise you can use a function to construct your pointing element yourself
        cc.draw_series(PointSeries::of_element(
            (-3.0f32..2.1f32).step(1.0).values().map(|x| (x, x.sin())),
            5,
            ShapeStyle::from(&RED).filled(),
            &|coord, size, style| {
                EmptyElement::at(coord)
                    + Circle::new((0, 0), size, style)
                    + Text::new(format!("{:?}", coord), (0, 15), ("sans-serif", 15))
            },
        ))
        .unwrap();

        let drawing_areas = lower.split_evenly((1, 2));

        for (drawing_area, idx) in drawing_areas.iter().zip(1..) {
            let mut cc = ChartBuilder::on(&drawing_area)
                .x_label_area_size(30)
                .y_label_area_size(30)
                .margin_right(20)
                .caption(format!("y = x^{}", 1 + 2 * idx), ("sans-serif", 40))
                .build_cartesian_2d(-1f32..1f32, -1f32..1f32)
                .unwrap();
            cc.configure_mesh()
                .x_labels(5)
                .y_labels(3)
                .max_light_lines(4)
                .draw()
                .unwrap();

            cc.draw_series(LineSeries::new(
                (-1f32..1f32)
                    .step(0.01)
                    .values()
                    .map(|x| (x, x.powf(idx as f32 * 2.0 + 1.0))),
                &BLUE,
            ))
            .unwrap();
        }

        root_area.present().unwrap();
    }

    plot_buff
}

// ω <mod-def normal_spec_component>
