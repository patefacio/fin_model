//! Support for plotting a distribution

////////////////////////////////////////////////////////////////////////////////////
// --- module uses ---
////////////////////////////////////////////////////////////////////////////////////
use crate::IncrementalStats;
use crate::SvgPoint;
use std::ops::RangeInclusive;

////////////////////////////////////////////////////////////////////////////////////
// --- structs ---
////////////////////////////////////////////////////////////////////////////////////
/// Defines how an svg is broken up
#[derive(Debug, Clone, Default)]
pub struct DistributionSpans {
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
    /// Percentile of this point among all points
    pub percentile: f64,
    /// Location of the point - updated when points added
    pub location: SvgPoint,
}

/// The plot is a collection of `num_bins` bins of [PlotPoint] entries.
/// The bins contain the indices of the [PlotPoint] entries.
#[derive(Debug, Clone)]
pub struct PlotBin {
    /// The indices of the plot points
    pub plot_points: Vec<usize>,
}

/// Generates an svg distribution plot of a series data.
/// `value_range` is the range of values `(minimum value, maximum value)`.
/// Each [PlotPoint] contains the _normalized value_ (i.e. the value scaled to the
/// `value_range`), the percentile of the point, and optionally a label.
#[derive(Debug, Clone, Default)]
pub struct DistributionPlot {
    /// Span configuration of the svg
    pub distribution_spans: DistributionSpans,
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
    /// The points on the plot
    pub points: Vec<PlotPoint>,
    /// The vector of bins
    pub bins: Vec<PlotBin>,
    /// Incremental stats of values
    pub incremental_stats: IncrementalStats,
}

////////////////////////////////////////////////////////////////////////////////////
// --- type impls ---
////////////////////////////////////////////////////////////////////////////////////
impl DistributionSpans {
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
    ///   * **percentile** - Percentile of this point among all points
    ///   * **location** - Location of the point - updated when points added
    ///   * _return_ - The constructed instance
    pub fn new(id: u32, value: f64, percentile: f64, location: SvgPoint) -> Self {
        // α <new initialization>
        // ω <new initialization>
        Self {
            id,
            value,
            percentile,
            location,
        }
    }
}

impl DistributionPlot {
    /// Augment current data with more values in the form of pair (id, value)
    ///
    ///   * **sorted_values** - Values to add
    pub fn add_sorted_values<'a>(
        &mut self,
        sorted_values: impl Iterator<Item = &'a (u32, f64)> + Clone,
    ) {
        // α <fn DistributionPlot::add_sorted_values>
        debug_assert!(
            sorted_values
                .clone()
                .is_sorted_by(|a, b| a.1.partial_cmp(&b.1)),
            "New values added must be sorted"
        );

        // Clone the current range or initialize
        let (mut current_start, mut current_end) = self
            .value_range
            .as_ref()
            .map(|r| (*r.start(), *r.end()))
            .unwrap_or_else(|| (f64::MAX, f64::MIN));

        for &(id, value) in sorted_values.clone() {
            self.incremental_stats.push_value(value);
            current_start = current_start.min(value);
            current_end = current_end.max(value);
            self.points.push(PlotPoint {
                id,
                value,
                ..Default::default()
            });
        }
        self.update_range_items(current_start..=current_end);
        self.place_points();

        tracing::warn!("Added values: {self:#?}");

        // ω <fn DistributionPlot::add_sorted_values>
    }

    /// Add the x axis
    pub fn add_axis(&mut self) {
        // α <fn DistributionPlot::add_axis>
        todo!("Implement `add_axis`")
        // ω <fn DistributionPlot::add_axis>
    }

    /// Whenever range is expanded the `bin_width` and `value_factor` need to be
    /// reevaluated. Width of bin is defined by `max_value - min_value/num_bins`.
    /// `value_factor` is `100.0 / (max_value - min_value)`.
    ///
    ///
    ///   * **new_range** - Updated range
    #[inline]
    pub fn update_range_items(&mut self, new_range: RangeInclusive<f64>) {
        // α <fn DistributionPlot::update_range_items>

        if Some(&new_range) != self.value_range.as_ref() {
            self.value_range = Some(new_range.clone());
            let value_span = (*new_range.end() - *new_range.start())
                / (self.distribution_spans.plot_width_pct / 100.0);
            self.value_factor = 100.0 / value_span;
            // Expand the width by the inner pad on left and right
            self.bin_width_pct = self.distribution_spans.plot_width_pct / self.num_bins as f64;
            self.bin_width = self.bin_width_pct / self.value_factor;
        }

        // ω <fn DistributionPlot::update_range_items>
    }

    /// The range items have been updated and the points are ready to be placed in the bins.
    /// Updates all points with new position and inserts their index into the bin.
    pub fn place_points(&mut self) {
        // α <fn DistributionPlot::place_points>

        let num_points = self.points.len();

        for bin in self.bins.iter_mut() {
            bin.plot_points.clear();
        }

        let mut max_points_in_any_bin = 0;
        let value_range = self.value_range.as_ref().unwrap();
        let max_bin_index = (self.num_bins - 1) as usize;

        // Iterate over points assigning x position, percentile and place in proper bin
        for (i, plot_point) in self.points.iter_mut().enumerate() {
            let mut bin_index =
                ((plot_point.value - *value_range.start()) / self.bin_width) as usize;

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
            max_points_in_any_bin = max_points_in_any_bin.max(bin.plot_points.len());
        }

        let diameter = self
            .distribution_spans
            .max_circle_r_pct
            .min(self.distribution_spans.plot_height_pct / max_points_in_any_bin as f64)
            .min(self.bin_width_pct - 0.001);

        self.circle_r_pct = diameter / 2.0;
        let plot_base_y_pct = self.distribution_spans.plot_base_y_pct;
        let half_bin_width = self.bin_width_pct / 2.0;

        // Iterate over points again and assign cy for each point
        for (bin_index, bin) in self.bins.iter().enumerate() {
            let mut cy = plot_base_y_pct - self.circle_r_pct;
            for &point_index in bin.plot_points.iter() {
                let location = &mut self.points[point_index].location;
                location.y = cy;
                location.x = (bin_index as f64 * self.bin_width_pct + half_bin_width)
                    + self.distribution_spans.first_bin_x_start_pct;

                cy -= diameter;
            }
        }

        // ω <fn DistributionPlot::place_points>
    }

    /// Initializer
    ///
    ///   * **distribution_spans** - Span configuration of the svg
    ///   * **num_bins** - Number of bins to display data in
    ///   * _return_ - The constructed instance
    pub fn new(distribution_spans: DistributionSpans, num_bins: u32) -> Self {
        // α <fn DistributionPlot::new>

        Self {
            distribution_spans,
            num_bins,
            bins: (0..num_bins)
                .map(|_| PlotBin {
                    plot_points: Vec::default(),
                })
                .collect(),
            ..Default::default()
        }
        // ω <fn DistributionPlot::new>
    }
}

/// Unit tests for `distribution`
#[cfg(test)]
pub mod unit_tests {

    /// Test type DistributionPlot
    mod test_distribution_plot {
        ////////////////////////////////////////////////////////////////////////////////////
        // --- module uses ---
        ////////////////////////////////////////////////////////////////////////////////////
        use test_log::test;

        ////////////////////////////////////////////////////////////////////////////////////
        // --- functions ---
        ////////////////////////////////////////////////////////////////////////////////////
        #[test]
        fn add_sorted_values() {
            // α <fn test DistributionPlot::add_sorted_values>
            todo!("Test add_sorted_values")
            // ω <fn test DistributionPlot::add_sorted_values>
        }

        #[test]
        fn add_axis() {
            // α <fn test DistributionPlot::add_axis>
            todo!("Test add_axis")
            // ω <fn test DistributionPlot::add_axis>
        }

        #[test]
        fn update_range_items() {
            // α <fn test DistributionPlot::update_range_items>
            todo!("Test update_range_items")
            // ω <fn test DistributionPlot::update_range_items>
        }

        #[test]
        fn place_points() {
            // α <fn test DistributionPlot::place_points>
            todo!("Test place_points")
            // ω <fn test DistributionPlot::place_points>
        }

        // α <mod-def test_distribution_plot>
        // ω <mod-def test_distribution_plot>
    }

    // α <mod-def unit_tests>
    // ω <mod-def unit_tests>
}

// α <mod-def distribution>
// ω <mod-def distribution>
