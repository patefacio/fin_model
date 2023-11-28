//! Support for plotting a histogram

////////////////////////////////////////////////////////////////////////////////////
// --- module uses ---
////////////////////////////////////////////////////////////////////////////////////
use crate::IncrementalStats;
use crate::SvgArea;
use crate::SvgDim;
use crate::SvgPoint;
use std::collections::HashMap;
use std::ops::RangeInclusive;

////////////////////////////////////////////////////////////////////////////////////
// --- enums ---
////////////////////////////////////////////////////////////////////////////////////
/// Enumerates the quartiles, min, max, and mean
#[derive(Debug, Copy, Clone)]
pub enum DescriptivePoint {
    /// Minimum value point
    Min,
    /// First quartile point
    Q1,
    /// Second quartile point (median)
    Q2,
    /// Third quartile point
    Q3,
    /// Maximum value point
    Max,
    /// Point closest to the arithmetic mean
    Mean,
}

////////////////////////////////////////////////////////////////////////////////////
// --- structs ---
////////////////////////////////////////////////////////////////////////////////////
/// An histogram value to be incorporated into a histogram
#[derive(Debug, Clone)]
pub struct HistogramEntry {
    /// Id of the value
    pub id: u32,
    /// Value to be plotted
    pub value: f64,
}

/// Defines how an svg is broken up
#[derive(Debug, Clone, Default)]
pub struct HistogramSpans {
    /// Width of x-axis at the bottom
    pub axis_line_width_pct: f64,
    /// Width of grid lines
    pub grid_line_width_pct: f64,
    /// Width of vertical mean line
    pub mean_line_width_pct: f64,
    /// Width of vertical median line
    pub median_line_width_pct: f64,
    /// Padding on left and right of chart.
    /// The axis goes from `pad_x` to `dim_x - pad_x`  
    pub pad_x_pct: f64,
    /// Extra padding inside the axis before leftmost/rightmost point is drawn
    pub inner_pad_pct: f64,
    /// Padding on top and bottom of chart
    pub pad_y_pct: f64,
    /// Maximum radius of circle
    pub max_circle_r_pct: f64,
    /// x position of left edge of first bin
    pub first_bin_x_start_pct: f64,
    /// Width of area dedicated to the plot starting at `first_bin_x_start`
    pub plot_width_pct: f64,
    /// y value for base of plot
    pub plot_base_y_pct: f64,
    /// Height of area dedicated to the plot starting just above the x axis
    pub plot_height_pct: f64,
}

/// Details of the plot point, including its value and location
#[derive(Debug, Clone, Default)]
pub struct PlotPoint {
    /// Numeric id of the value - for matching up specific points with styles
    pub id: u32,
    /// Value of the point
    pub value: f64,
    /// Row the point is in
    pub row_index: u32,
    /// Bin the point is in
    pub bin_index: u32,
    /// Percentile of this point among all points
    pub percentile: f64,
    /// Location of the point - updated when points added
    pub location: SvgPoint,
}

/// The plot is a collection of `num_bins` bins of [PlotPoint] entries.
/// A bin is a vector of points aligned vertically. Similarly a a row in the plot
/// can be collected into a vector of points.
#[derive(Debug, Clone)]
pub struct PointVector {
    /// The indices of the plot points
    pub plot_points: Vec<usize>,
}

/// Generates an svg histogram plot of a series data.
/// `value_range` is the range of values `(minimum value, maximum value)`.
/// Each [PlotPoint] contains the _normalized value_ (i.e. the value scaled to the
/// `value_range`), the percentile of the point, and optionally a label.
#[derive(Debug, Clone, Default)]
pub struct HistogramPlot {
    /// Span configuration of the svg
    pub histogram_spans: HistogramSpans,
    /// Range of values (min to max)
    /// This dictates the `value_factor` and `bin_width`. Each time a batch of values
    /// are added the min or max may change, requiring `numb_bins` and `value_factor`
    /// to be reevaluated.
    pub value_range: Option<RangeInclusive<f64>>,
    /// Multiplicative factor that scales values proportionally from range to 100
    pub value_factor: f64,
    /// Radius of circle which may vary as max/min value vary
    pub circle_r_pct: f64,
    /// Number of bins to display data in
    pub num_bins: u32,
    /// Width of bin
    pub bin_width_pct: f64,
    /// Width of bin, recalculated when values added
    pub bin_width: f64,
    /// Elements comprising the x-axis and grid lines
    pub axis_and_grid_elements: Vec<String>,
    /// The points on the plot, sorted by value
    pub sorted_points: Vec<PlotPoint>,
    /// The vector of bins from left to right
    pub bins: Vec<PointVector>,
    /// The vector of rows from base to top of plot
    pub rows: Vec<PointVector>,
    /// Maps the provided id to its index in the vector
    pub id_to_index: HashMap<u32, usize>,
    /// Incremental stats of values
    pub incremental_stats: IncrementalStats,
}

////////////////////////////////////////////////////////////////////////////////////
// --- type impls ---
////////////////////////////////////////////////////////////////////////////////////
impl HistogramEntry {
    /// Initializer
    ///
    ///   * **id** - Id of the value
    ///   * **value** - Value to be plotted
    ///   * _return_ - The constructed instance
    pub fn new(id: u32, value: f64) -> Self {
        // α <new initialization>
        // ω <new initialization>
        Self { id, value }
    }
}

impl HistogramSpans {
    /// Basic configuration
    ///
    ///   * _return_ - The constructed instance
    pub fn with_defaults() -> Self {
        let axis_line_width_pct = 0.20;
        let grid_line_width_pct = 0.5;
        let mean_line_width_pct = 0.5;
        let median_line_width_pct = 0.5;
        let pad_x_pct = 1.0;
        let inner_pad_pct = 1.5;
        let pad_y_pct = 2.0;
        let max_circle_r_pct = 10.0;
        // α <with_defaults initialization>
        let first_bin_x_start_pct = pad_x_pct + inner_pad_pct;
        // Entire width, less left side up to start of bin 0, less right padding
        let plot_width_pct = 100.0 - first_bin_x_start_pct - pad_x_pct - inner_pad_pct;
        let plot_base_y_pct = 100.0 - pad_y_pct - axis_line_width_pct;
        let plot_height_pct = plot_base_y_pct - pad_y_pct - inner_pad_pct;
        // ω <with_defaults initialization>
        Self {
            axis_line_width_pct,
            grid_line_width_pct,
            mean_line_width_pct,
            median_line_width_pct,
            pad_x_pct,
            inner_pad_pct,
            pad_y_pct,
            max_circle_r_pct,
            first_bin_x_start_pct,
            plot_width_pct,
            plot_base_y_pct,
            plot_height_pct,
        }
    }
}

impl PlotPoint {
    /// Initializer
    ///
    ///   * **id** - Numeric id of the value - for matching up specific points with styles
    ///   * **value** - Value of the point
    ///   * **row_index** - Row the point is in
    ///   * **bin_index** - Bin the point is in
    ///   * **percentile** - Percentile of this point among all points
    ///   * **location** - Location of the point - updated when points added
    ///   * _return_ - The constructed instance
    pub fn new(
        id: u32,
        value: f64,
        row_index: u32,
        bin_index: u32,
        percentile: f64,
        location: SvgPoint,
    ) -> Self {
        // α <new initialization>
        // ω <new initialization>
        Self {
            id,
            value,
            row_index,
            bin_index,
            percentile,
            location,
        }
    }
}

impl HistogramPlot {
    /// Augment current data with more values in the form of pair (id, value)
    ///
    ///   * **sorted_values** - Values to add
    pub fn add_sorted_values<'a>(
        &mut self,
        sorted_values: impl Iterator<Item = &'a HistogramEntry> + Clone,
    ) {
        // α <fn HistogramPlot::add_sorted_values>
        debug_assert!(
            sorted_values
                .clone()
                .is_sorted_by(|a, b| a.value.partial_cmp(&b.value)),
            "New values added must be sorted"
        );

        // Clone the current range or initialize
        let (mut current_start, mut current_end) = self
            .value_range
            .as_ref()
            .map(|r| (*r.start(), *r.end()))
            .unwrap_or_else(|| (f64::MAX, f64::MIN));

        for (i, &HistogramEntry { id, value }) in sorted_values.clone().enumerate() {
            self.id_to_index.insert(id, i);
            self.incremental_stats.push_value(value);
            current_start = current_start.min(value);
            current_end = current_end.max(value);
            self.sorted_points.push(PlotPoint {
                id,
                value,
                ..Default::default()
            });
        }
        self.update_range_items(current_start..=current_end);
        self.place_points();

        tracing::debug!("Added values: {self:#?}");

        // ω <fn HistogramPlot::add_sorted_values>
    }

    /// Get point based on description
    ///
    ///   * **descriptive_point** - Quartile break point to get
    ///   * _return_ - Corresponding position and point
    pub fn get_descriptive_point(&self, descriptive_point: DescriptivePoint) -> (u32, &PlotPoint) {
        // α <fn HistogramPlot::get_descriptive_point>

        let len = self.sorted_points.len();
        let quartile_span = len / 4;

        let position = match descriptive_point {
            DescriptivePoint::Min => 0,
            DescriptivePoint::Q1 => quartile_span,
            DescriptivePoint::Q2 => quartile_span * 2,
            DescriptivePoint::Q3 => quartile_span * 3,
            DescriptivePoint::Max => len - 1,
            DescriptivePoint::Mean => {
                let mean = self.incremental_stats.mean().unwrap();
                match self
                    .sorted_points
                    .binary_search_by(|plot_point| plot_point.value.partial_cmp(&mean).unwrap())
                {
                    Ok(i) => i,
                    Err(i) => i,
                }
            }
        };

        (position as u32, &self.sorted_points[position])

        // ω <fn HistogramPlot::get_descriptive_point>
    }

    /// Get two shading areas, one for the row of the selected item and one for the column(bin).
    /// The idea is to show the selection by styling a background row and column rectangle that
    /// intersect at the selected point.
    ///
    ///   * **id** - Id of the point to _select_.
    ///   * _return_ - The (x shading area, y shading area) tuple
    pub fn get_selector_areas(&mut self, id: u32) -> (SvgArea, SvgArea) {
        // α <fn HistogramPlot::get_selector_areas>
        let plot_point_index = self.id_to_index.get(&id).unwrap();
        let plot_point = self.sorted_points.get(*plot_point_index).unwrap();
        let row_index = plot_point.row_index;
        let bin_index = plot_point.bin_index;
        let x_rect = SvgArea::new(
            SvgPoint::new(
                self.histogram_spans.first_bin_x_start_pct,
                row_index as f64 * self.circle_r_pct * 2.0,
            ),
            SvgDim::new(self.histogram_spans.plot_width_pct, self.circle_r_pct * 2.0),
        );
        let y_rect = SvgArea::new(
            SvgPoint::new(
                self.histogram_spans.first_bin_x_start_pct
                    + bin_index as f64 * self.circle_r_pct * 2.0,
                0.0,
            ),
            SvgDim::new(self.bin_width_pct, self.circle_r_pct * 2.0),
        );
        (x_rect, y_rect)

        // ω <fn HistogramPlot::get_selector_areas>
    }

    /// Get a pointer to the selected point.
    /// The element is a group with a marker for the arrowhead and a line drawn from upper
    /// right to the point.
    ///
    ///   * _return_ - Group defining a pointer pointing at the selected point
    pub fn get_pointer_element(&self) -> String {
        // α <fn HistogramPlot::get_pointer_element>

        let radius = self.circle_r_pct;
        let neg_radius = -radius;
        let diameter = radius * 2.0;
        // Define the arrow head as (diameter, diameter, diameter*sqrt(2.0))
        // Pointing at the center of the circle. The transform will back the entire pointer away from the center
        let arrow_head_dim = 2.0 * diameter;
        let neg_arrow_head_dim = -arrow_head_dim;

        let arrow_length = 8.0 * radius;
        let neg_arrow_length = -arrow_length;
        let stroke_width = self.circle_r_pct;

        format!(
            r#"
<path d="M0,{neg_arrow_head_dim} L{arrow_head_dim},0 L0,0 z" fill="black" stroke-width="{stroke_width}"/>
<line x1="{arrow_length}" y1="{neg_arrow_length}" x2="{radius}" y2="{neg_radius}" stroke="black" stroke-width="{stroke_width}"/>
        "#
        )

        // ω <fn HistogramPlot::get_pointer_element>
    }

    /// Get a translation of the pointer so it points to specified plot point of `id`.
    ///
    ///   * **id** - Id of the point to _select_.
    ///   * _return_ - The point to translate the pointer to
    pub fn get_pointer_translation(&self, id: u32) -> SvgPoint {
        // α <fn HistogramPlot::get_pointer_translation>

        let plot_point_index = self.id_to_index.get(&id).unwrap();
        let mut location = self.sorted_points.get(*plot_point_index).unwrap().location;
        let shift = self.circle_r_pct * 0.65;
        location.x += shift;
        location.y -= shift;

        location

        // ω <fn HistogramPlot::get_pointer_translation>
    }

    /// Add the x axis
    pub fn add_axis(&mut self) {
        // α <fn HistogramPlot::add_axis>
        todo!("Implement `add_axis`")
        // ω <fn HistogramPlot::add_axis>
    }

    /// Whenever range is expanded the `bin_width` and `value_factor` need to be
    /// reevaluated. Width of bin is defined by `max_value - min_value/num_bins`.
    /// `value_factor` is `100.0 / (max_value - min_value)`.
    ///
    ///
    ///   * **new_range** - Updated range
    #[inline]
    pub fn update_range_items(&mut self, new_range: RangeInclusive<f64>) {
        // α <fn HistogramPlot::update_range_items>

        if Some(&new_range) != self.value_range.as_ref() {
            self.value_range = Some(new_range.clone());
            let value_span = (*new_range.end() - *new_range.start())
                / (self.histogram_spans.plot_width_pct / 100.0);
            self.value_factor = 100.0 / value_span;
            // Expand the width by the inner pad on left and right
            self.bin_width_pct = self.histogram_spans.plot_width_pct / self.num_bins as f64;
            self.bin_width = self.bin_width_pct / self.value_factor;
        }

        // ω <fn HistogramPlot::update_range_items>
    }

    /// The range items have been updated and the points are ready to be placed in the bins.
    /// Updates all points with new position and inserts their index into the bin.
    pub fn place_points(&mut self) {
        // α <fn HistogramPlot::place_points>

        let num_points = self.sorted_points.len();

        for bin in self.bins.iter_mut() {
            bin.plot_points.clear();
        }

        for row in self.rows.iter_mut() {
            row.plot_points.clear();
        }

        let mut max_points_in_any_bin = 0;
        let value_range = self.value_range.as_ref().unwrap();
        let max_bin_index = (self.num_bins - 1) as usize;
        let mut row_index = 0;
        let mut prior_bin_index = None;

        // Iterate over points assigning x position, percentile and place in proper bin
        for (i, plot_point) in self.sorted_points.iter_mut().enumerate() {
            let mut bin_index =
                ((plot_point.value - *value_range.start()) / self.bin_width) as usize;

            // If this point advances to next bin, reset the row index
            if let Some(prior_bin_index) = prior_bin_index {
                if prior_bin_index != bin_index {
                    row_index = 0;
                }
            }

            tracing::debug!(
                "Getting bin @ {bin_index} from value({}) with bin_width({}) from ({}) bins and range({:?})",
                plot_point.value,
                self.bin_width,
                self.bins.len(),
                self.value_range
            );

            // Floating math above can lead to bin_index of just beyond end of vec so protect
            debug_assert!(bin_index <= self.num_bins as usize);
            bin_index = bin_index.min(max_bin_index);
            plot_point.percentile = i as f64 / num_points as f64;
            plot_point.location = SvgPoint::new(0.0, 0.0);
            let bin = self.bins.get_mut(bin_index).unwrap();
            bin.plot_points.push(i);
            if row_index == self.rows.len() {
                self.rows.push(PointVector {
                    plot_points: Vec::new(),
                });
            }

            let row = self.rows.get_mut(row_index).unwrap();
            plot_point.row_index = row_index as u32;
            plot_point.bin_index = bin_index as u32;
            row.plot_points.push(i);

            max_points_in_any_bin = max_points_in_any_bin.max(bin.plot_points.len());
            prior_bin_index = Some(bin_index);
            row_index += 1;
        }

        let diameter = self
            .histogram_spans
            .max_circle_r_pct
            .min(self.histogram_spans.plot_height_pct / max_points_in_any_bin as f64)
            .min(self.bin_width_pct - 0.001);

        self.circle_r_pct = diameter / 2.0;
        let plot_base_y_pct = self.histogram_spans.plot_base_y_pct;
        let half_bin_width = self.bin_width_pct / 2.0;

        // Iterate over points again and assign cy for each point
        for (bin_index, bin) in self.bins.iter().enumerate() {
            let mut cy = plot_base_y_pct - self.circle_r_pct;
            for &point_index in bin.plot_points.iter() {
                let location = &mut self.sorted_points[point_index].location;
                location.y = cy;
                location.x = (bin_index as f64 * self.bin_width_pct + half_bin_width)
                    + self.histogram_spans.first_bin_x_start_pct;

                cy -= diameter;
            }
        }

        // ω <fn HistogramPlot::place_points>
    }

    /// Initializer
    ///
    ///   * **histogram_spans** - Span configuration of the svg
    ///   * **num_bins** - Number of bins to display data in
    ///   * _return_ - The constructed instance
    pub fn new(histogram_spans: HistogramSpans, num_bins: u32) -> Self {
        // α <fn HistogramPlot::new>

        Self {
            histogram_spans,
            num_bins,
            bins: (0..num_bins)
                .map(|_| PointVector {
                    plot_points: Vec::default(),
                })
                .collect(),
            ..Default::default()
        }
        // ω <fn HistogramPlot::new>
    }
}

/// Unit tests for `histogram`
#[cfg(test)]
pub mod unit_tests {

    /// Test type HistogramPlot
    mod test_histogram_plot {
        ////////////////////////////////////////////////////////////////////////////////////
        // --- module uses ---
        ////////////////////////////////////////////////////////////////////////////////////
        use test_log::test;

        ////////////////////////////////////////////////////////////////////////////////////
        // --- functions ---
        ////////////////////////////////////////////////////////////////////////////////////
        #[test]
        fn add_sorted_values() {
            // α <fn test HistogramPlot::add_sorted_values>
            todo!("Test add_sorted_values")
            // ω <fn test HistogramPlot::add_sorted_values>
        }

        #[test]
        fn get_descriptive_point() {
            // α <fn test HistogramPlot::get_descriptive_point>
            todo!("Test get_descriptive_point")
            // ω <fn test HistogramPlot::get_descriptive_point>
        }

        #[test]
        fn get_selector_areas() {
            // α <fn test HistogramPlot::get_selector_areas>
            todo!("Test get_selector_areas")
            // ω <fn test HistogramPlot::get_selector_areas>
        }

        #[test]
        fn get_pointer_element() {
            // α <fn test HistogramPlot::get_pointer_element>
            todo!("Test get_pointer_element")
            // ω <fn test HistogramPlot::get_pointer_element>
        }

        #[test]
        fn get_pointer_translation() {
            // α <fn test HistogramPlot::get_pointer_translation>
            todo!("Test get_pointer_translation")
            // ω <fn test HistogramPlot::get_pointer_translation>
        }

        #[test]
        fn add_axis() {
            // α <fn test HistogramPlot::add_axis>
            todo!("Test add_axis")
            // ω <fn test HistogramPlot::add_axis>
        }

        #[test]
        fn update_range_items() {
            // α <fn test HistogramPlot::update_range_items>
            todo!("Test update_range_items")
            // ω <fn test HistogramPlot::update_range_items>
        }

        #[test]
        fn place_points() {
            // α <fn test HistogramPlot::place_points>
            todo!("Test place_points")
            // ω <fn test HistogramPlot::place_points>
        }

        // α <mod-def test_histogram_plot>
        // ω <mod-def test_histogram_plot>
    }

    // α <mod-def unit_tests>
    // ω <mod-def unit_tests>
}

// α <mod-def histogram>
// ω <mod-def histogram>
