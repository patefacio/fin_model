

### Memory Leaks



### Guidelines

Do as much work as possible in pure rust. This makes testing simpler.


If closures are passed in (i.e. Updatable) be sure to move them into either a
signal (`create_signal`, `create_rw_signal`) or the store (`store_value`).
Because those calls all accept a _Scope_ (`cx`), those values are pushed into a
dictionary and the data is cleaned up when the component goes away. Without
moving that data into something like that


#### Don't call parent closure while inside a {update} of {signal/store_value}

```
ENTER_KEY => {
                console_log(&format!("Numeric input enter key was pressed:1"));
                let input_ref = node_ref.get().expect("Input node");
                // This code goes through leptos borrowing
                numeric_input_data.update_value(|numeric_input_data| {
                    console_log(&format!("Numeric input enter key was pressed:2"));
                    if let Some(on_enter) = numeric_input_data.on_enter.as_mut() {
                        console_log(&format!("Numeric input enter key was pressed:3"));
 ===> OOOPS!!           on_enter(input_ref.value());
                        console_log(&format!("User done with on_enter"));
                        ev.prevent_default();
                    }
                })
            }
```
