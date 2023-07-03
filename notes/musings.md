

### Memory Leaks



### Guidelines

Do as much work as possible in pure rust. This makes testing simpler.


If closures are passed in (i.e. Updatable) be sure to move them into either a
signal (`create_signal`, `create_rw_signal`) or the store (`store_value`).
Because those calls all accept a _Scope_ (`cx`), those values are pushed into a
dictionary and the data is cleaned up when the component goes away. Without
moving that data into something like that