//! Support for tracking pearson coefficient (a.k.a correlation). The term _incremental_
//! means the algorithm does not require all the data in memory at once. Rather, it
//! supports adding the data one point at a time and keeping a running tally of all
//! inputs to the correlation calculation so it can be produced on demand.

////////////////////////////////////////////////////////////////////////////////////
// --- module uses ---
////////////////////////////////////////////////////////////////////////////////////
use crate::MeasuredStats;
use ndarray::Array1;

////////////////////////////////////////////////////////////////////////////////////
// --- structs ---
////////////////////////////////////////////////////////////////////////////////////
/// Incrementally tracks pearson coefficient (i.e. correlation) and in process means and std_devs.
/// Implements efficient incremental calculation of correlation. That is, it avoids creating
/// an Nx50,000 matrix to calculate a coefficient matrix for a 50,000 iterated simulation.
/// The idea is rather than collect all data points in a big matrix, just keep summary information
/// as it progresses and provide the means, std_devs, and correlations when finished.
///
/// StdDev uses <https://math.stackexchange.com/questions/102978/incremental-computation-of-standard-deviation>
/// Correlations uses <https://stats.stackexchange.com/questions/410468/online-update-of-pearson-coefficient>
///
/// Also for triangular storage this is useful:
/// <https://www.geeksforgeeks.org/efficient-method-to-store-a-lower-triangular-matrix-using-row-major-mapping/>
#[derive(Debug, Clone, Default, PartialEq)]
pub struct IncrementalPearson {
    /// One row for each pair of indices in the mix
    pearson_rows: Vec<PearsonRowEntry>,
    /// Triangular summary data.
    pearson_triangular: Vec<PearsonTriangularEntry>,
    /// Number of rows tracked/added
    rows_tracked: usize,
}

/// Row indexed summary data.
/// Default provided to set `min` to max possible and `max` to min possible.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PearsonRowEntry {
    /// Incremental mean on last row addition
    pub prior_mean: f64,
    /// Incremental mean
    pub mean: f64,
    /// Minimum value
    pub min: f64,
    /// Maximum value
    pub max: f64,
    /// Running variance
    pub variance: f64,
}

/// Calculates pearson coefficient (i.e. correlation).
/// Tracks a single entry in triangular matrix.
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct PearsonTriangularEntry {
    /// Covariance
    pub cov: f64,
}

////////////////////////////////////////////////////////////////////////////////////
// --- type impls ---
////////////////////////////////////////////////////////////////////////////////////
impl IncrementalPearson {
    /// Create new [IncrementalPearson].
    ///
    ///   * **row_dim** - Length defining number of rows in a matrix and number of entries in lower triangular matrix.
    ///   * _return_ - New properly sized [IncrementalPearson]
    pub fn new(row_dim: usize) -> IncrementalPearson {
        // α <fn IncrementalPearson::new>

        IncrementalPearson {
            pearson_rows: vec![PearsonRowEntry::default(); row_dim],
            pearson_triangular: vec![
                PearsonTriangularEntry::default();
                (row_dim * (row_dim + 1)) / 2
            ],
            rows_tracked: 0,
        }

        // ω <fn IncrementalPearson::new>
    }

    /// Update data with new values.
    ///
    ///   * **values** - One set of returns in the series to add to summary stats.
    pub fn track_row(&mut self, values: &Array1<f64>) {
        // α <fn IncrementalPearson::track_row>

        debug_assert!(values.len() == self.pearson_rows.len());

        let prior_n = self.rows_tracked as f64;
        let n = prior_n + 1.0;

        for (i, &x) in values.iter().enumerate() {
            let row = &mut self.pearson_rows[i];
            let delta = x - row.mean;
            let delta_n = delta / n;
            row.prior_mean = row.mean;
            row.mean += delta_n;
            row.variance += delta * delta_n * prior_n;
            if row.max < x {
                row.max = x;
            }
            if row.min > x {
                row.min = x;
            }
        }

        for (i, x) in values.iter().enumerate() {
            for (j, y) in values.iter().enumerate() {
                if i >= j {
                    // Indexing [i,j] as triangular is [i*(i-1)/2+j-1]
                    let index = (i + 1) * i / 2 + j;
                    let triangular_entry = &mut self.pearson_triangular[index];
                    let x_mean = &self.pearson_rows[i].prior_mean;
                    let y_mean = &self.pearson_rows[j].prior_mean;
                    triangular_entry.cov += (x_mean - x) * (y_mean - y) * prior_n / n;
                } else {
                    break;
                }
            }
        }

        self.rows_tracked += 1;

        // ω <fn IncrementalPearson::track_row>
    }

    /// Get the incremental coefficient (correlation).
    ///
    ///   * **index** - The 2D index to desired correlation.
    ///   * _return_ - The incremental correlation.
    pub fn get_pearson_coefficient(&self, index: (usize, usize)) -> Option<f64> {
        // α <fn IncrementalPearson::get_pearson_coefficient>

        if self.rows_tracked > 1 {
            let (i, j) = if index.0 > index.1 {
                (index.0, index.1)
            } else {
                (index.1, index.0)
            };
            let triangular_index = (i + 1) * i / 2 + j;
            let triangular_entry = self.pearson_triangular[triangular_index];
            let t = self.get_std_dev(i).expect("std dev present")
                * self.get_std_dev(j).expect("std dev present");
            if t > 0.0 {
                Some(triangular_entry.cov / ((self.rows_tracked as f64 - 1.0) * t))
            } else {
                None
            }
        } else {
            None
        }

        // ω <fn IncrementalPearson::get_pearson_coefficient>
    }

    /// Get the mean of indexed item.
    ///
    ///   * **index** - The index of desired mean.
    ///   * _return_ - Incremental mean of indexed item.
    pub fn get_mean(&self, index: usize) -> Option<f64> {
        // α <fn IncrementalPearson::get_mean>
        if self.rows_tracked > 0 {
            Some(self.pearson_rows[index].mean)
        } else {
            None
        }
        // ω <fn IncrementalPearson::get_mean>
    }

    /// Get the standard deviation of indexed item.
    ///
    ///   * **index** - The index of desired std dev.
    ///   * _return_ - Incremental standard deviation of indexed item.
    pub fn get_std_dev(&self, index: usize) -> Option<f64> {
        // α <fn IncrementalPearson::get_std_dev>

        let n = self.rows_tracked as f64;
        if n > 1.0 {
            Some((self.pearson_rows[index].variance / (n - 1.0)).sqrt())
        } else {
            None
        }
        // ω <fn IncrementalPearson::get_std_dev>
    }

    /// Get the estimated normal indexed item.
    ///
    ///   * **index** - The index of desired mean.
    ///   * _return_ - Incremental mean and std_dev of indexed item.
    pub fn get_est_normal(&self, index: usize) -> Option<(f64, f64)> {
        // α <fn IncrementalPearson::get_est_normal>

        let n = self.rows_tracked;

        if n > 1 {
            let row_entry = self.pearson_rows[index];
            Some((
                row_entry.mean,
                self.get_std_dev(index).expect("n > 1, std_dev present"),
            ))
        } else {
            None
        }

        // ω <fn IncrementalPearson::get_est_normal>
    }

    /// Get the bundle of measured stats.
    ///
    ///   * **index** - The index of desired stats.
    ///   * _return_ - Measured stats.
    pub fn get_measured_stats(&self, index: usize) -> Option<MeasuredStats> {
        // α <fn IncrementalPearson::get_measured_stats>

        if self.rows_tracked > 0 {
            let row_entry = self.pearson_rows[index];
            let est_normal = self.get_est_normal(index);

            Some(MeasuredStats {
                count: self.rows_tracked,
                min: Some(row_entry.min),
                max: Some(row_entry.max),
                mean: est_normal.map(|ns| ns.0),
                median: None,
                std_dev: est_normal.map(|ns| ns.1),
            })
        } else {
            None
        }

        // ω <fn IncrementalPearson::get_measured_stats>
    }
}

/// Accessors for [IncrementalPearson] fields
impl IncrementalPearson {
    #[inline]
    pub fn get_pearson_rows(&self) -> &Vec<PearsonRowEntry> {
        &self.pearson_rows
    }

    #[inline]
    pub fn get_pearson_triangular(&self) -> &Vec<PearsonTriangularEntry> {
        &self.pearson_triangular
    }

    #[inline]
    pub fn get_rows_tracked(&self) -> usize {
        self.rows_tracked
    }
}

////////////////////////////////////////////////////////////////////////////////////
// --- trait impls ---
////////////////////////////////////////////////////////////////////////////////////
impl Default for PearsonRowEntry {
    /// A trait for giving a type a useful default value.
    ///
    ///   * _return_ - The new default instance
    fn default() -> Self {
        // α <fn Default::default for PearsonRowEntry>

        PearsonRowEntry {
            prior_mean: 0.0,
            mean: 0.0,
            min: f64::MAX,
            max: f64::MIN,
            variance: 0.0,
        }

        // ω <fn Default::default for PearsonRowEntry>
    }
}

/// Unit tests for `incremental_pearson`
#[cfg(test)]
pub mod unit_tests {

    /// Test type IncrementalPearson
    mod test_incremental_pearson {
        ////////////////////////////////////////////////////////////////////////////////////
        // --- module uses ---
        ////////////////////////////////////////////////////////////////////////////////////
        use test_log::test;

        ////////////////////////////////////////////////////////////////////////////////////
        // --- functions ---
        ////////////////////////////////////////////////////////////////////////////////////
        #[test]
        fn track_row_basic_stats() {
            // α <fn test IncrementalPearson::track_row_basic_stats>

            use approx::assert_relative_eq;
            use ndarray::arr1;

            let data = [
                [10.32, 103.0, 97.0],
                [10.22, 105.0, 95.3],
                [9.31, 96.0, 102.0],
                [9.93, 101.0, 98.5],
                [10.1, 100.5, 100.0],
                [7.93, 82.0, 115.0],
                [11.23, 108.0, 90.3],
            ];

            let mut incremental = IncrementalPearson::new(3);
            for row in data.iter() {
                incremental.track_row(&arr1(row))
            }

            assert_relative_eq!(
                9.862857143f64,
                incremental.get_mean(0).unwrap(),
                epsilon = 0.00001
            );
            assert_relative_eq!(
                99.35714286,
                incremental.get_mean(1).unwrap(),
                epsilon = 0.00001
            );
            assert_relative_eq!(
                99.72857143,
                incremental.get_mean(2).unwrap(),
                epsilon = 0.00001
            );

            assert_relative_eq!(
                1.025340826,
                incremental.get_est_normal(0).unwrap().1,
                epsilon = 0.00001
            );
            assert_relative_eq!(
                8.527965201,
                incremental.get_est_normal(1).unwrap().1,
                epsilon = 0.00001
            );
            assert_relative_eq!(
                7.701885978,
                incremental.get_est_normal(2).unwrap().1,
                epsilon = 0.00001
            );

            assert_relative_eq!(
                1.0,
                incremental.get_pearson_coefficient((0, 0)).unwrap(),
                epsilon = 0.00001
            );

            assert_relative_eq!(
                0.977478679,
                incremental.get_pearson_coefficient((0, 1)).unwrap(),
                epsilon = 0.00001
            );

            assert_relative_eq!(
                -0.9917098187,
                incremental.get_pearson_coefficient((1, 2)).unwrap(),
                epsilon = 0.00001
            );

            // ω <fn test IncrementalPearson::track_row_basic_stats>
        }

        // α <mod-def test_incremental_pearson>
        use super::*;
        // ω <mod-def test_incremental_pearson>
    }

    // α <mod-def unit_tests>
    use super::*;
    // ω <mod-def unit_tests>
}

// α <mod-def incremental_pearson>
// ω <mod-def incremental_pearson>
