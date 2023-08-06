# unwrap-log

Non-panicking alternatives to `Option` and `Result` unwrapping, which log at warn level.

## Example

```rust
use unwrap_log::{OptionExt, ResultExt};
use env_logger::Builder;
use log::LevelFilter::Warn;

Builder::new().filter_level(Warn).init();

let x: i32 = None.unwrap_or_default_log();
assert_eq!(x, 0);

let y: i32 = Err("oops").unwrap_or_default_log();
assert_eq!(y, 0);
```

Output:
```
[1970-01-01T00:00:00Z WARN  my_crate] src\main.rs:8:23 encountered `None`
[1970-01-01T00:00:00Z WARN  my_crate] src\main.rs:11:30 encountered `Err("oops")`
```
