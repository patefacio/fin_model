////////////////////////////////////////////////////////////////////////////////////
// --- module uses ---
////////////////////////////////////////////////////////////////////////////////////
use crate::MeasuredStats;

////////////////////////////////////////////////////////////////////////////////////
// --- structs ---
////////////////////////////////////////////////////////////////////////////////////
/// Tracks small set of statistics in one pass
#[derive(Debug, Clone)]
pub struct IncrementalStats {
    /// Count of values
    count: usize,
    /// Mean of values before last push
    prior_mean: f64,
    /// Mean of values
    mean: f64,
    /// Sum of squared diff of values
    sum_squared_diff: f64,
    /// Min of values
    min: f64,
    /// Max of values
    max: f64,
    /// Vector of values if tracking median is required
    values: Option<Vec<f64>>,
    /// Median, only on call to `finalize_median`.
    pub median: Option<f64>,
}

////////////////////////////////////////////////////////////////////////////////////
// --- type impls ---
////////////////////////////////////////////////////////////////////////////////////
impl IncrementalStats {
    /// New [IncrementalStats] with values having been pushed.
    ///
    ///   * **values** - Values to push into new [IncrementalStats]
    ///   * **track_median** - If true will track median.
    ///   * _return_ - [IncrementalStats] tracking instance.
    #[inline]
    pub fn from_values(values: &[f64], track_median: bool) -> IncrementalStats {
        // α <fn IncrementalStats::from_values>
        let mut result = if track_median {
            IncrementalStats::new(values.len())
        } else {
            IncrementalStats::default()
        };

        result.push_values(values);
        result
        // ω <fn IncrementalStats::from_values>
    }

    /// Push value into stats
    ///
    ///   * **value** - Value to push into stats
    #[inline]
    pub fn push_value(&mut self, value: f64) {
        // α <fn IncrementalStats::push_value>

        let prior_n = self.count as f64;
        let n = prior_n + 1.0;
        self.count += 1;

        let delta = value - self.mean;
        let delta_n = delta / n;
        self.prior_mean = self.mean;
        self.mean += delta_n;
        self.sum_squared_diff += delta * delta_n * prior_n;
        if value < self.min {
            self.min = value;
        }
        if value > self.max {
            self.max = value;
        }

        if let Some(values) = self.values.as_mut() {
            values.push(value);
        }

        // ω <fn IncrementalStats::push_value>
    }

    /// Push value into stats
    ///
    ///   * **values** - Values to push into stats
    #[inline]
    pub fn push_values(&mut self, values: &[f64]) {
        // α <fn IncrementalStats::push_values>

        for &value in values {
            self.push_value(value);
        }

        // ω <fn IncrementalStats::push_values>
    }

    /// Count of items pushed
    ///
    ///   * _return_ - Count of items pushed.
    #[inline]
    pub fn count(&self) -> usize {
        // α <fn IncrementalStats::count>
        self.count
        // ω <fn IncrementalStats::count>
    }

    /// Arithmetic mean
    ///
    ///   * _return_ - Mean of pushed data.
    #[inline]
    pub fn mean(&self) -> Option<f64> {
        // α <fn IncrementalStats::mean>

        if self.count > 0 {
            Some(self.mean)
        } else {
            None
        }

        // ω <fn IncrementalStats::mean>
    }

    /// Min of values pushed
    ///
    ///   * _return_ - Min of values pushed.
    #[inline]
    pub fn min(&self) -> Option<f64> {
        // α <fn IncrementalStats::min>

        if self.count > 0 {
            Some(self.min)
        } else {
            None
        }

        // ω <fn IncrementalStats::min>
    }

    /// Max of values pushed
    ///
    ///   * _return_ - Max of pushed data.
    #[inline]
    pub fn max(&self) -> Option<f64> {
        // α <fn IncrementalStats::max>

        if self.count > 0 {
            Some(self.max)
        } else {
            None
        }

        // ω <fn IncrementalStats::max>
    }

    /// Variance of values pushed
    ///
    ///   * _return_ - Variance of pushed data.
    #[inline]
    pub fn variance(&self) -> Option<f64> {
        // α <fn IncrementalStats::variance>

        if self.count > 1 {
            Some(self.sum_squared_diff / (self.count as f64 - 1.0))
        } else {
            None
        }

        // ω <fn IncrementalStats::variance>
    }

    /// Standard deviation of values pushed
    ///
    ///   * _return_ - Standard deviation of pushed data.
    #[inline]
    pub fn std_dev(&self) -> Option<f64> {
        // α <fn IncrementalStats::std_dev>

        if self.count > 1 {
            self.variance().map(|var| var.sqrt())
        } else {
            None
        }

        // ω <fn IncrementalStats::std_dev>
    }

    /// Create a new instance of [IncrementalStats]
    ///
    ///   * **median_capacity** - If non-0 will track median and initialize vector with specified capacity.
    ///   * _return_ - A new [IncrementalStats] instance.
    pub fn new(median_capacity: usize) -> IncrementalStats {
        // α <fn IncrementalStats::new>

        if median_capacity > 0 {
            IncrementalStats {
                values: Some(Vec::with_capacity(median_capacity)),
                ..Default::default()
            }
        } else {
            IncrementalStats::default()
        }

        // ω <fn IncrementalStats::new>
    }

    /// All data has been pushed and this will sort the values making _median_ available.
    /// **Note** This clears the values after saving the median to save memory.
    ///
    ///   * **keep_data_points** - If set and medians on dossier items are tracked, this will preserve values.
    /// Normally when `track_median` is set all values are saved in a vector until
    /// a call to `finalize_median` called. At the end of that function it deletes
    /// those values to save memory...unless this value is set to true.
    pub fn finalize_median(&mut self, keep_data_points: bool) {
        // α <fn IncrementalStats::finalize_median>

        if let Some(values) = self.values.as_mut() {
            if values.len() > 0 {
                values.sort_by(|a, b| a.partial_cmp(b).unwrap());
                self.median = values.get((values.len() - 1) / 2).copied();
                tracing::debug!(
                    "Sorted {} values to get to median -> {}",
                    values.len(),
                    self.get_measured_stats()
                );
            }
        }

        if !keep_data_points {
            self.values = None
        }

        // ω <fn IncrementalStats::finalize_median>
    }

    /// The bundle of stats.
    ///
    ///   * _return_ - The bundle of stats so far.
    #[inline]
    pub fn get_measured_stats(&self) -> MeasuredStats {
        // α <fn IncrementalStats::get_measured_stats>

        if self.count > 0 {
            MeasuredStats {
                count: self.count,
                min: Some(self.min),
                max: Some(self.max),
                mean: self.mean(),
                median: self.median,
                std_dev: self.std_dev(),
            }
        } else {
            MeasuredStats::default()
        }

        // ω <fn IncrementalStats::get_measured_stats>
    }
}

/// Accessors for [IncrementalStats] fields
impl IncrementalStats {
    #[inline]
    pub fn get_count(&self) -> usize {
        self.count
    }

    #[inline]
    pub fn get_values(&self) -> &Option<Vec<f64>> {
        &self.values
    }
}

////////////////////////////////////////////////////////////////////////////////////
// --- trait impls ---
////////////////////////////////////////////////////////////////////////////////////
impl Default for IncrementalStats {
    /// A trait for giving a type a useful default value.
    ///
    ///   * _return_ - The new default instance
    fn default() -> Self {
        // α <fn Default::default for IncrementalStats>

        IncrementalStats {
            count: 0,
            prior_mean: 0.0,
            mean: 0.0,
            sum_squared_diff: 0.0,
            min: std::f64::MAX,
            max: std::f64::MIN,
            values: None,
            median: None,
        }

        // ω <fn Default::default for IncrementalStats>
    }
}

/// Unit tests for `incremental_stats`
#[cfg(test)]
pub mod unit_tests {

    /// Test type IncrementalStats
    mod test_incremental_stats {
        ////////////////////////////////////////////////////////////////////////////////////
        // --- module uses ---
        ////////////////////////////////////////////////////////////////////////////////////
        use test_log::test;

        ////////////////////////////////////////////////////////////////////////////////////
        // --- functions ---
        ////////////////////////////////////////////////////////////////////////////////////
        #[test]
        fn from_values() {
            // α <fn test IncrementalStats::from_values>

            use approx::assert_relative_eq;

            let stats = IncrementalStats::from_values(&[1.0, 2.0, 3.0], false);
            assert_eq!(3, stats.count());
            assert_eq!(Some(1.0), stats.min());
            assert_eq!(Some(3.0), stats.max());
            assert_eq!(Some(2.0), stats.mean());
            assert_relative_eq!(1.0, stats.std_dev().unwrap());
            assert_relative_eq!(1.0, stats.variance().unwrap());

            let stats = IncrementalStats::from_values(
                &[10.32, 10.22, 9.31, 9.93, 10.1, 7.93, 11.23],
                false,
            );

            assert_relative_eq!(9.862857143f64, stats.mean().unwrap(), epsilon = 0.00001);
            assert_relative_eq!(1.025340826, stats.std_dev().unwrap(), epsilon = 0.00001);

            // ω <fn test IncrementalStats::from_values>
        }

        #[test]
        fn push_value() {
            // α <fn test IncrementalStats::push_value>
            use approx::assert_relative_eq;

            let mut stats = IncrementalStats::default();
            stats.push_value(1.0);
            stats.push_value(2.0);
            stats.push_value(3.0);
            assert_eq!(3, stats.count());
            assert_eq!(Some(1.0), stats.min());
            assert_eq!(Some(3.0), stats.max());
            assert_eq!(Some(2.0), stats.mean());
            assert_relative_eq!(1.0, stats.std_dev().unwrap());
            assert_relative_eq!(1.0, stats.variance().unwrap());

            // ω <fn test IncrementalStats::push_value>
        }

        #[test]
        fn push_values() {
            // α <fn test IncrementalStats::push_values>

            let stats = IncrementalStats::from_values(&[1.0, 2.0, 3.0], false);
            assert_eq!(3, stats.count);

            // ω <fn test IncrementalStats::push_values>
        }

        #[test]
        fn count() {
            // α <fn test IncrementalStats::count>
            let stats = IncrementalStats::from_values(&[1.0, 2.0, 3.0], false);
            assert_eq!(3, stats.count);
            // ω <fn test IncrementalStats::count>
        }

        #[test]
        fn mean() {
            // α <fn test IncrementalStats::mean>
            let stats = IncrementalStats::from_values(&[1.0, 2.0, 3.0], false);
            assert_eq!(2.0, stats.mean().unwrap());
            // ω <fn test IncrementalStats::mean>
        }

        #[test]
        fn min() {
            // α <fn test IncrementalStats::min>
            let stats = IncrementalStats::from_values(&[1.0, 2.0, 3.0], false);
            assert_eq!(1.0, stats.min().unwrap());
            // ω <fn test IncrementalStats::min>
        }

        #[test]
        fn max() {
            // α <fn test IncrementalStats::max>
            let stats = IncrementalStats::from_values(&[1.0, 2.0, 3.0], false);
            assert_eq!(3.0, stats.max().unwrap());
            // ω <fn test IncrementalStats::max>
        }

        #[test]
        fn variance() {
            // α <fn test IncrementalStats::variance>
            let stats = IncrementalStats::from_values(&[1.0, 2.0, 3.0], false);
            assert_eq!(1.0, stats.variance().unwrap());
            // ω <fn test IncrementalStats::variance>
        }

        #[test]
        fn std_dev() {
            // α <fn test IncrementalStats::std_dev>
            let stats = IncrementalStats::from_values(&[1.0, 2.0, 3.0], false);
            assert_eq!(1.0, stats.std_dev().unwrap());
            // ω <fn test IncrementalStats::std_dev>
        }

        #[test]
        fn new() {
            // α <fn test IncrementalStats::new>

            assert_eq!(1024, IncrementalStats::new(1024).values.unwrap().capacity())

            // ω <fn test IncrementalStats::new>
        }

        #[test]
        fn finalize_median() {
            // α <fn test IncrementalStats::finalize_median>

            let mut stats = IncrementalStats::from_values(&[1.0, 2.0, 3.0], true);
            stats.finalize_median(true);
            assert_eq!(2.0, stats.median.unwrap());
            assert_eq!(vec![1.0, 2.0, 3.0], stats.values.unwrap());

            stats = IncrementalStats::from_values(&[1.0, 2.0, 3.0], true);
            stats.finalize_median(false);
            assert_eq!(2.0, stats.median.unwrap());
            assert_eq!(true, stats.values.is_none());

            // ω <fn test IncrementalStats::finalize_median>
        }

        #[test]
        fn get_measured_stats() {
            // α <fn test IncrementalStats::get_measured_stats>

            use approx::assert_relative_eq;
            let stats = IncrementalStats::from_values(&[1.0, 2.0, 3.0], false);
            let measured_stats = stats.get_measured_stats();

            assert_eq!(3, measured_stats.count);
            assert_relative_eq!(1.0, measured_stats.min.expect("testing"));
            assert_relative_eq!(3.0, measured_stats.max.expect("testing"));
            assert_relative_eq!(2.0, measured_stats.mean.expect("testing"));

            // ω <fn test IncrementalStats::get_measured_stats>
        }

        // α <mod-def test_incremental_stats>
        use super::*;
        // ω <mod-def test_incremental_stats>
    }

    // α <mod-def unit_tests>
    use super::*;
    // ω <mod-def unit_tests>
}

// α <mod-def incremental_stats>
// ω <mod-def incremental_stats>
