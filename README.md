# unwrap-log

Non-panicking alternatives to `Option` and `Result` unwrapping, which log at warn level.

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://github.com/ryanavella/unwrap-log-rs/blob/master/LICENSE-MIT) [![License: Unlicense](https://img.shields.io/badge/license-Unlicense-blue.svg)](https://github.com/ryanavella/unwrap-log-rs/blob/master/LICENSE-UNLICENSE) [![crates.io](https://img.shields.io/crates/v/unwrap-log.svg?colorB=319e8c)](https://crates.io/crates/unwrap-log) [![docs.rs](https://img.shields.io/badge/docs.rs-unwrap--log-yellowgreen)](https://docs.rs/unwrap-log)

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
