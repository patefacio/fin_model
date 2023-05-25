//! TODO: Document Module(multi_column_select)

////////////////////////////////////////////////////////////////////////////////////
// --- module uses ---
////////////////////////////////////////////////////////////////////////////////////
use crate::utils::block_time::BlockTime;
use crate::utils::constants::{
    DOWN_KEY, ENTER_KEY, ESCAPE_KEY, LEFT_KEY, RIGHT_KEY, SPACE_KEY, TAB_KEY, UP_KEY,
};
use crate::utils::element_sugar::{element_from_event, find_element_up};
use crate::utils::html_tag::HtmlTag;
use leptos::{component, create_node_ref, create_rw_signal, store_value, tracing, view};
use leptos::{
    IntoAttribute, IntoClass, IntoView, Scope, SignalGet, SignalSet, SignalUpdate, WriteSignal,
};
use leptos_dom::console_log;
use leptos_dom::helpers::window_event_listener_untyped;
use leptos_dom::html::{Button, Div};
use std::rc::Rc;
use wasm_bindgen::JsCast;
use web_sys::{Element, Event, KeyboardEvent, MouseEvent};

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
    SelectionIndex(usize),
}

/// SelectOptions can be displayed left-to-right or top-to-bottom.
#[derive(Debug, Copy, Clone)]
pub enum Direction {
    /// SelectOptions fill in values row by row.
    LeftToRight,
    /// SelectOptions fill in values column by column.
    TopToBottom,
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
        label: String,
    },
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
    pub direction: Direction,
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
///   * **options** - The options to display
///   * **on_select** - Called when selection changes
///   * **initial_value** - Initial selection - if None assumes first option
///   * **direction** - Specifies whether items flows from top to bottom or left to right.
///   * **column_count** - Number of columns to display in the grid of options.
///   * _return_ - View for multi_column_select
#[component]
pub fn MultiColumnSelect<F>(
    /// Context
    cx: Scope,
    /// The options to display
    options: Vec<SelectOption>,
    /// Called when selection changes
    on_select: F,
    /// Initial selection - if None assumes first option
    #[prop(default=None)]
    initial_value: Option<InitialValue>,
    /// Specifies whether items flows from top to bottom or left to right.
    #[prop(default=Direction::LeftToRight)]
    direction: Direction,
    /// Number of columns to display in the grid of options.
    #[prop(default = 3)]
    column_count: usize,
) -> impl IntoView
where
    F: Fn(String) + 'static,
{
    // α <fn multi_column_select>
    let block_time = BlockTime::new("Creation of element");

    let indexer = Indexer::new(options.len(), column_count, direction);
    let mcs_grid_ref = create_node_ref::<Div>(cx);
    let main_button_ref = create_node_ref::<Button>(cx);
    let set_focus_main_button = move || {
        console_log("Focus to  main");
        main_button_ref
            .get()
            .expect("ref should be loaded")
            .focus()
            .expect("focus back to main menu");
        console_log("Focus should go to main button ref!!");
    };

    let menu_is_hidden = create_rw_signal(cx, true);
    let current_selected_index = create_rw_signal(
        cx,
        if let Some(InitialValue::SelectionIndex(i)) = initial_value {
            i
        } else {
            0
        },
    );

    let selection_vec = store_value(
        cx,
        Rc::new(
            (0..indexer.item_count)
                .map(|_| create_node_ref::<Button>(cx))
                .collect::<Vec<_>>(),
        ),
    );

    let initial_value = if let Some(InitialValue::Placeholder(placeholder)) = initial_value {
        placeholder
    } else {
        options[current_selected_index.get()]
            .main_button_label()
            .clone()
    };

    on_select(initial_value.clone());

    let main_button_label = create_rw_signal(cx, initial_value);
    let using_mouse = create_rw_signal(cx, false);

    let set_target_focus = move |flat_index: usize| {
        selection_vec.with_value(|selection_vec| {
            selection_vec
                .get(flat_index)
                .as_ref()
                .expect("Node exists")
                .get()
                .expect("ref should be loaded")
                .focus()
                .expect("focus set");
        });

        console_log(&format!("Done to manually set focus {flat_index}"));
    };

    let show_menu = move || {
        console_log("Showing Menu!!");
        menu_is_hidden.set(false);
        set_target_focus(current_selected_index.get());
    };

    let hide_menu = move || {
        console_log("Hiding Menu!!");
        menu_is_hidden.set(true);
    };

    fn get_selection(element: Element) -> Option<(usize, String)> {
        find_element_up(element, HtmlTag::Button).and_then(|element| {
            Some((
                element
                    .get_attribute("data-flat-index")
                    .expect("Selection has flat_index")
                    .parse()
                    .expect("An int"),
                element
                    .get_attribute("data-value")
                    .expect("Selection has value"),
            ))
        })
    }

    let set_selection = Rc::new(move |element: Element| {
        if let Some((flat_index, selected)) = get_selection(element) {
            console_log(&format!("Selected {flat_index} {selected}"));
            current_selected_index.set(flat_index);
            main_button_label.set(selected.clone());
            on_select(selected);
        }
    });

    let key_down_set_selection = set_selection.clone();
    let handle_key_down = Rc::new(move |ev: KeyboardEvent| {
        let _timing = BlockTime::new("key_down");
        let key_code = ev.key_code();

        match key_code {
            TAB_KEY => {
                using_mouse.set(false);
            }
            ENTER_KEY | SPACE_KEY | ESCAPE_KEY => {
                console_log("Enter|Space|Escape Key");
                if key_code != ESCAPE_KEY {
                    key_down_set_selection(element_from_event(&ev));
                }
                hide_menu();
                set_focus_main_button();
                ev.prevent_default();
            }
            LEFT_KEY | UP_KEY | RIGHT_KEY | DOWN_KEY => {
                if let Some((i, _)) = get_selection(element_from_event(&ev)) {
                    let target_i = indexer.key_move(i, key_code);
                    set_target_focus(target_i);
                    using_mouse.set(false);
                }
                console_log(&format!("Got one of the movement keys {key_code}"));
            }
            _ => console_log("Other key"),
        };
    });

    let click_set_selection = set_selection.clone();
    let handle_click = Rc::new(move |ev: MouseEvent| {
        console_log("Select button handle click");
        let _timing = BlockTime::new("click");
        click_set_selection(element_from_event(&ev));
        hide_menu();
        set_focus_main_button();
    });

    let handle_main_button_action = move || {
        let _timing = BlockTime::new("main button action");
        console_log(&format!("Main button action hidden({})", menu_is_hidden()));
        if menu_is_hidden() {
            show_menu();
        }
    };

    let handle_main_button_mousedown = move |ev: MouseEvent| {
        handle_main_button_action();
        // Following prevent subsequent global focus in from subsequently hiding when just shown
        // by stopping the default behavior
        ev.prevent_default();
    };

    let handle_main_button_key_activate = move |ev: KeyboardEvent| {
        let _timing = BlockTime::new("key_down");
        let key_code = ev.key_code();

        match key_code {
            ENTER_KEY | SPACE_KEY => {
                console_log("Enter|Space Key");
                handle_main_button_action();
                ev.prevent_default();
            }
            _ => console_log("Other key"),
        };
    };

    let handle_mouseover = move |ev: MouseEvent| {
        console_log("Got mouseover!");
        if let Some((index, _)) = get_selection(element_from_event(&ev)) {
            set_target_focus(index);
        }
    };

    let handle_mousemove = move |_| {
        using_mouse.set(true);
    };

    let mut cells = Vec::with_capacity(indexer.item_count);
    for row in 0..indexer.row_count {
        for column in 0..column_count {
            let flat_index = indexer.two_d_to_flat_index(row, column);
            let cell = if let Some(select_option) = options.get(flat_index) {
                let button_ref =
                    selection_vec.with_value(|selection_vec| selection_vec[flat_index]);
                let (value, button_content) = match select_option {
                    SelectOption::Label(label) => (label, view! {cx, <div class="mcs-label">{label}</div> }.into_view(cx)),
                    SelectOption::KeyLabel { key, label } => (
                        key,
                        view! {cx,
                            <div class="icon-label">
                                <div class="icon">{key}</div>
                                <div class="label">{label}</div>
                            </div>
                        }
                        .into_view(cx),
                    ),
                };

                let wrapped_click = handle_click.clone();
                let wrapped_handle_click = move |ev| wrapped_click(ev);
                let wrapped_handle_keydown = handle_key_down.clone();
                let wrapped_handle_keydown = move |ev| wrapped_handle_keydown(ev);

                view! { cx,
                <button
                    class="select-button"
                    on:click=wrapped_handle_click
                    on:mouseover=handle_mouseover
                    on:mousemove=handle_mousemove
                    on:keydown=wrapped_handle_keydown
                    class:using-mouse=using_mouse
                    data-flat-index=flat_index
                    data-value=value
                    node_ref=button_ref
                >{button_content}</button> }
                .into_view(cx)
            } else {
                view! { cx, <div></div> }.into_view(cx)
            };
            cells.push(cell);
        }
    }

    let handle_global_mousedown = move |ev: Event| {
        console_log("Global mousedown working !!");
        if !menu_is_hidden() {
            let container_div = mcs_grid_ref.get().expect("div");
            let target_element = element_from_event(&ev);
            let same_element = container_div.is_equal_node(Some(target_element.unchecked_ref()));

            console_log(&format!(
                "same({same_element}) target({:?}), container({:?})",
                target_element, *container_div
            ));
            if same_element || !container_div.contains(Some(&target_element)) {
                hide_menu()
            }
        }
    };

    #[cfg(not(feature = "ssr"))]
    window_event_listener_untyped("mousedown", handle_global_mousedown);

    // Important that this reacts to focusin and not focus or blur.
    // Focus and blur do not propagate!
    let handle_global_focusin = move |ev: Event| {
        if !menu_is_hidden() {
            let container_div = mcs_grid_ref.get().expect("div");
            let target_element = element_from_event(&ev);

            console_log(&format!("Global focus in !! {}", target_element.tag_name(),));

            let main_button_got_focus = main_button_ref
                .get()
                .expect("ref should be loaded!")
                .is_equal_node(Some(&target_element.unchecked_ref()));
            if main_button_got_focus || !container_div.contains(Some(&target_element)) {
                hide_menu()
            }
        }
    };

    #[cfg(not(feature = "ssr"))]
    window_event_listener_untyped("focusin", handle_global_focusin);

    view! { cx,
        <div class="mcs-grid"
            disabled=move || {!menu_is_hidden()}
            node_ref=mcs_grid_ref
        >
            <button
                on:mousedown=handle_main_button_mousedown
                on:keydown=handle_main_button_key_activate
                class="main-button"
                node_ref=main_button_ref
            >
            {main_button_label}
            </button>
            <div
                class="container"
                class:hidden={menu_is_hidden}
                // This sets the number of grid columns
                style=format!("grid-template-columns: {}", "1fr ".repeat(column_count))
            >

            {cells.into_view(cx)}
        </div>
        </div>
    }
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
    pub fn new(item_count: usize, column_count: usize, direction: Direction) -> Self {
        // α <fn Indexer::new>

        let mut row_count = item_count / column_count;
        if item_count % column_count > 0 {
            row_count += 1;
        }

        Indexer {
            item_count,
            row_count: row_count,
            column_count,
            direction,
        }

        // ω <fn Indexer::new>
    }

    /// Given a flat index identifying an option in the vector, gets the corresponding
    /// (row, column) index for the grid.
    ///
    ///   * **flat_index** - Index of selection.
    ///   * _return_ - The 2D index in grid.
    #[inline]
    pub fn flat_index_to_two_d(&self, flat_index: usize) -> (usize, usize) {
        // α <fn Indexer::flat_index_to_two_d>
        match self.direction {
            Direction::LeftToRight => (
                flat_index / self.column_count,
                flat_index % self.column_count,
            ),
            Direction::TopToBottom => (flat_index % self.row_count, flat_index / self.row_count),
        }
        // ω <fn Indexer::flat_index_to_two_d>
    }

    /// Given 2D grid index identifying an option gets the corresponding flat_index into the vector.
    ///
    ///   * **row** - Row in grid.
    ///   * **column** - Column in grid.
    ///   * _return_ - The flat index of item.
    #[inline]
    pub fn two_d_to_flat_index(&self, row: usize, column: usize) -> usize {
        // α <fn Indexer::two_d_to_flat_index>
        match self.direction {
            Direction::LeftToRight => row * self.column_count + column,
            Direction::TopToBottom => column * self.row_count + row,
        }
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
    pub fn wrap(n: isize, max_n: usize) -> usize {
        // α <fn Indexer::wrap>
        let r = n % max_n as isize;
        (if r < 0 { r + max_n as isize } else { r }) as usize
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
    pub fn nearby_row(&self, mut row: usize, column: usize, offset: isize) -> usize {
        // α <fn Indexer::nearby_row>

        loop {
            row = Self::wrap(row as isize + offset, self.row_count);
            if self.two_d_to_flat_index(row, column) < self.item_count {
                break;
            }
        }

        self.two_d_to_flat_index(row, column)

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
    pub fn nearby_column(&self, row: usize, mut column: usize, offset: isize) -> usize {
        // α <fn Indexer::nearby_column>

        loop {
            column = Self::wrap(column as isize + offset, self.column_count);
            if self.two_d_to_flat_index(row, column) < self.item_count {
                break;
            }
        }

        self.two_d_to_flat_index(row, column)

        // ω <fn Indexer::nearby_column>
    }

    /// Given _flat index_ of current position applies calculates new position
    /// after a move indicated by key (Up, Down, Left, Right).
    ///
    ///   * **current** - Flat index of current position.
    ///   * **key** - Value of key code.
    ///   * _return_ - New _flat index_.
    pub fn key_move(&self, current: usize, key: u32) -> usize {
        // α <fn Indexer::key_move>
        let (row, column) = self.flat_index_to_two_d(current);
        match key {
            UP_KEY => self.nearby_row(row, column, -1),
            DOWN_KEY => self.nearby_row(row, column, 1),
            LEFT_KEY => self.nearby_column(row, column, -1),
            RIGHT_KEY => self.nearby_column(row, column, 1),
            _ => current,
        }
        // ω <fn Indexer::key_move>
    }
}

impl SelectOption {
    /// The label to display on the main button showing the current selection.
    ///
    ///   * _return_ - The label for the main button
    #[inline]
    pub fn main_button_label(&self) -> &String {
        // α <fn SelectOption::main_button_label>
        match self {
            SelectOption::Label(label) => label,
            SelectOption::KeyLabel { key, label: _ } => key,
        }
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

            let ltr_indexer = Indexer::new(6*16, 16, Direction::LeftToRight); 
            assert_eq!((0, 15), ltr_indexer.flat_index_to_two_d(15));
            assert_eq!((1, 0), ltr_indexer.flat_index_to_two_d(16));

            let ttb_indexer = Indexer::new(6*16, 16, Direction::TopToBottom);
            assert_eq!((5, 0), ttb_indexer.flat_index_to_two_d(5));
            assert_eq!((0, 1), ttb_indexer.flat_index_to_two_d(6));

            // ω <fn test Indexer::flat_index_to_two_d>
        }

        #[test]
        fn two_d_to_flat_index() {
            // α <fn test Indexer::two_d_to_flat_index>

            let ltr_indexer = Indexer::new(6*16, 16, Direction::LeftToRight);
            assert_eq!(15, ltr_indexer.two_d_to_flat_index(0, 15));
            assert_eq!(16, ltr_indexer.two_d_to_flat_index(1, 0));

            let ttb_indexer = Indexer::new(6*16, 16, Direction::TopToBottom);
            assert_eq!(5, ttb_indexer.two_d_to_flat_index(5, 0));
            assert_eq!(6, ttb_indexer.two_d_to_flat_index(0, 1));

            // ω <fn test Indexer::two_d_to_flat_index>
        }

        #[test]
        fn wrap() {
            // α <fn test Indexer::wrap>

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
        use super::*;
        // ω <mod-def test_indexer>
    }

    // α <mod-def unit_tests>
    use super::*;
    // ω <mod-def unit_tests>
}

// α <mod-def multi_column_select>


// ω <mod-def multi_column_select>
