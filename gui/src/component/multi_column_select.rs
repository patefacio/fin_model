//! TODO: Document Module(multi_column_select)

////////////////////////////////////////////////////////////////////////////////////
// --- enums ---
////////////////////////////////////////////////////////////////////////////////////
/// The initial value, which can be either a selection from the options or
/// if a default value is not appropriate a _placeholder_ to show in the label
/// until the value is selected.
pub enum InitialValue {
  /// Displayed on the main select button until a selection is made.
  Placeholder(String),
  /// Specifies the initial selection that may be changed by the component.
  SelectionIndex(usize)
}

/// SelectOptions can be displayed left-to-right or top-to-bottom.
#[derive(Debug, Copy, Clone)]
pub enum Direction {
  /// SelectOptions fill in values row by row.
  LeftToRight,
  /// SelectOptions fill in values column by column.
  TopToBottom
}

/// Enumerates the types of options that have labels appear in the select menu.
/// 
/// - _Label_ is a simple display showing its value.
/// - _KeyLabel_ contains a _key_ and _label_ where both are shown in the menu and
///   the _key_ of the final selection  is displayed in the button that opens the menu.
///    
#[derive(Debug, Clone)]
pub enum SelectOption {
  /// Just a label to show in both the selections and the button.
  Label(String),
  /// Provides a _key_ (typically shorter value/mnemonic) and _label_
  KeyLabel {
    /// The key
    key: String,
    /// The label
    label: String
  }
}

////////////////////////////////////////////////////////////////////////////////////
// --- structs ---
////////////////////////////////////////////////////////////////////////////////////
/// Used to calculate index values for movement within the menu grid.
#[derive(Debug, Copy, Clone)]
pub struct Indexer {
  /// Number of items.
  pub item_count: usize,
  /// Number of rows.
  pub row_count: usize,
  /// Number of columns.
  pub column_count: usize,
  /// Direction of option placement. Since options are stored in a vector but
  /// displayed in a 2D grid, the translation between scalar index into vector
  /// and 2D index of grid will differ and the functions account for that.
  pub direction: Direction
}

////////////////////////////////////////////////////////////////////////////////////
// --- functions ---
////////////////////////////////////////////////////////////////////////////////////
/// A component that supports a drop down selection which can span multiple columns.
/// The purpose is to be able to better style large selection lists that otherwise would
/// be very long vertical lists. The features include:
/// - Navigating the selections with left/right and up/down arrow keys
/// - Tab support for each entry
/// - Focus _auto-selects_ the item so tabbing and navigating to selection makes it the
///   current selection
/// - Accept the selection on Enter, Space
/// - No-Op on Escape
/// 
/// This uses a grid for styling.
/// 
///   * **cx** - Context
///   * **on_select** - Called when selection changes
///   * _return_ - View for multi_column_select
pub fn MultiColumnSelect<F>(
  /// Context
  cx: Scope,
  /// Called when selection changes
  on_select: F
) -> IntoView
where
  F: Fn(String) + 'static {
  // α <fn multi_column_select>
  todo!("Implement `multi_column_select`")
  // ω <fn multi_column_select>
}

////////////////////////////////////////////////////////////////////////////////////
// --- type impls ---
////////////////////////////////////////////////////////////////////////////////////
impl Indexer {
  /// Create new indexer.
  /// 
  ///   * **item_count** - Number of items.
  ///   * **column_count** - Number of columns.
  ///   * **direction** - Selections go across or down.
  ///   * _return_ - New Indexer
  pub fn new(
    item_count: usize,
    column_count: usize,
    direction: Direction
  ) -> Self {
    // α <fn Indexer::new>
    todo!("Implement `new`")
    // ω <fn Indexer::new>
  }
  
  /// Given a flat index identifying an option in the vector, gets the corresponding
  /// (row, column) index for the grid.
  /// 
  ///   * **flat_index** - Index of selection.
  ///   * _return_ - The 2D index in grid.
  #[inline]
  pub fn flat_index_to_two_d(
    &self,
    flat_index: usize
  ) -> (usize, usize) {
    // α <fn Indexer::flat_index_to_two_d>
    todo!("Implement `flat_index_to_two_d`")
    // ω <fn Indexer::flat_index_to_two_d>
  }
  
  /// Given 2D grid index identifying an option gets the corresponding flat_index into the vector.
  /// 
  ///   * **row** - Row in grid.
  ///   * **column** - Column in grid.
  ///   * _return_ - The flat index of item.
  #[inline]
  pub fn two_d_to_flat_index(
    &self,
    row: usize,
    column: usize
  ) -> usize {
    // α <fn Indexer::two_d_to_flat_index>
    todo!("Implement `two_d_to_flat_index`")
    // ω <fn Indexer::two_d_to_flat_index>
  }
  
  /// Basically a _mod_ operation that supports negative numbers.
  /// Wrap indexing often involves the mod operation which can provide
  /// index into an entry within a row (or column) by taking the remainder
  /// when dividing by number of rows (or columns). 
  /// [See](https://stackoverflow.com/questions/1907565/c-and-python-different-behaviour-of-the-modulo-operation)
  /// 
  ///   * **n** - The flat index to wrap to a row (or column)
  ///   * **max_n** - Number of elements in the row (or column)
  ///   * _return_ - The wrapped flat index
  #[inline]
  pub fn wrap(
    n: isize,
    max_n: usize
  ) -> usize {
    // α <fn Indexer::wrap>
    todo!("Implement `wrap`")
    // ω <fn Indexer::wrap>
  }
  
  /// Search for cell in following (or previous) row in the same column.
  /// If cell is empty keeps moving until not. This handles the 
  /// empty _cells_ at the end of the table. 
  /// 
  ///   * **row** - Row of cell to start move.
  ///   * **column** - Column of cell to start move.
  ///   * **offset** - Offset (negative implies left/previous).
  ///   * _return_ - The _flat index_ of the nearby cell.
  pub fn nearby_row(
    &self,
    mut row: usize,
    column: usize,
    offset: isize
  ) -> usize {
    // α <fn Indexer::nearby_row>
    todo!("Implement `nearby_row`")
    // ω <fn Indexer::nearby_row>
  }
  
  /// Search for cell in following (or previous) column in the same row.
  /// If cell is empty keeps moving until not. This handles the 
  /// empty _cells_ at the end of the table. 
  /// 
  ///   * **row** - Row of cell to start move.
  ///   * **column** - Column of cell to start move.
  ///   * **offset** - Offset (negative implies left/previous).
  ///   * _return_ - The _flat index_ of the nearby cell.
  pub fn nearby_column(
    &self,
    row: usize,
    mut column: usize,
    offset: isize
  ) -> usize {
    // α <fn Indexer::nearby_column>
    todo!("Implement `nearby_column`")
    // ω <fn Indexer::nearby_column>
  }
  
  /// Given _flat index_ of current position applies calculates new position
  /// after a move indicated by key (Up, Down, Left, Right).
  /// 
  ///   * **current** - Flat index of current position.
  ///   * **key** - Value of key code.
  ///   * _return_ - New _flat index_.
  pub fn key_move(
    &self,
    current: usize,
    key: u32
  ) -> usize {
    // α <fn Indexer::key_move>
    todo!("Implement `key_move`")
    // ω <fn Indexer::key_move>
  }
}

impl SelectOption {
  /// The label to display on the main button showing the current selection.
  /// 
  ///   * _return_ - The label for the main button
  #[inline]
  pub fn main_button_label(
    &self
  ) -> &String {
    // α <fn SelectOption::main_button_label>
    todo!("Implement `main_button_label`")
    // ω <fn SelectOption::main_button_label>
  }
}

/// Unit tests for `multi_column_select`
#[cfg(test)]
pub mod unit_tests {
  
  /// Test type Indexer
  mod test_indexer {
    ////////////////////////////////////////////////////////////////////////////////////
    // --- module uses ---
    ////////////////////////////////////////////////////////////////////////////////////
    use test_log::test;
    
    ////////////////////////////////////////////////////////////////////////////////////
    // --- functions ---
    ////////////////////////////////////////////////////////////////////////////////////
    #[test]
    fn new() {
      // α <fn test Indexer::new>
      todo!("Test new")
      // ω <fn test Indexer::new>
    }
    
    #[test]
    fn flat_index_to_two_d() {
      // α <fn test Indexer::flat_index_to_two_d>
      todo!("Test flat_index_to_two_d")
      // ω <fn test Indexer::flat_index_to_two_d>
    }
    
    #[test]
    fn two_d_to_flat_index() {
      // α <fn test Indexer::two_d_to_flat_index>
      todo!("Test two_d_to_flat_index")
      // ω <fn test Indexer::two_d_to_flat_index>
    }
    
    #[test]
    fn wrap() {
      // α <fn test Indexer::wrap>
      todo!("Test wrap")
      // ω <fn test Indexer::wrap>
    }
    
    #[test]
    fn nearby_row() {
      // α <fn test Indexer::nearby_row>
      todo!("Test nearby_row")
      // ω <fn test Indexer::nearby_row>
    }
    
    #[test]
    fn nearby_column() {
      // α <fn test Indexer::nearby_column>
      todo!("Test nearby_column")
      // ω <fn test Indexer::nearby_column>
    }
    
    #[test]
    fn key_move() {
      // α <fn test Indexer::key_move>
      todo!("Test key_move")
      // ω <fn test Indexer::key_move>
    }
    
    // α <mod-def test_indexer>
    // ω <mod-def test_indexer>
  }
  
  // α <mod-def unit_tests>
  // ω <mod-def unit_tests>
}

// α <mod-def multi_column_select>
// ω <mod-def multi_column_select>