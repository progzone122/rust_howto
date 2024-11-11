# Alternatives

[![cat-development-tools::debugging][cat-development-tools::debugging-badge]][cat-development-tools::debugging]{{hi:Debugging}}

## Older alternatives to `tracing`

Tracing is now the "go-to" crate for logging.

### `log`

[![log][c-log-badge]][c-log]{{hi:log}}

`log` is an older and simpler crate if your needs are simple and you are not using any async code.

### `slog`

[![slog][c-slog-badge]][c-slog]{{hi:slog}}
[![slog-crates.io][c-slog-crates.io-badge]][c-slog-crates.io]
[![slog-github][c-slog-github-badge]][c-slog-github]
[![slog-lib.rs][c-slog-lib.rs-badge]][c-slog-lib.rs]
[![cat-development-tools::debugging][cat-development-tools::debugging-badge]][cat-development-tools::debugging]{{hi:Debugging}}

Structured, extensible, composable logging.

`slog` remains a stable, featureful and battle-tested library, used in many important projects.

### `log4rs`

[![log4rs][c-log4rs-badge]][c-log4rs]{{hi:log4rs}}
[![log4rs-crates.io][c-log4rs-crates.io-badge]][c-log4rs-crates.io]
[![log4rs-github][c-log4rs-github-badge]][c-log4rs-github]
[![log4rs-lib.rs][c-log4rs-lib.rs-badge]][c-log4rs-lib.rs]

A highly configurable multi-output logging implementation for the log facade.

```rust
use log::{error, info, warn};
use log4rs;

fn main() {
    log4rs::init_file("config/log4rs.yaml", Default::default()).unwrap();

    info!("booting up");

    // ...
}
```

### env_logger

[![env_logger][c-env_logger-badge]][c-env_logger]{{hi:env_logger}}
[![env_logger-crates.io][c-env_logger-crates.io-badge]][c-env_logger-crates.io]
[![env_logger-github][c-env_logger-github-badge]][c-env_logger-github]
[![env_logger-lib.rs][c-env_logger-lib.rs-badge]][c-env_logger-lib.rs]
[![cat-development-tools::debugging][cat-development-tools::debugging-badge]][cat-development-tools::debugging]{{hi:Debugging}}

A logging implementation for log which is configured via an environment variable. `env_logger` makes sense when used in executables (binary projects). Libraries should use the log crate instead.

```rust
use log::info;

fn main() {
    env_logger::init();

    info!("starting up");

    // ...
}
```

## Other frameworks

### OpenTelemetry

[OpenTelemetry Rust documentation][opentelemetry-rust]⮳

## OpenObserve

[openobserve][openobserve-github]{{hi:openobserve}}⮳ (written in Rust) is a petabyte-scale Elasticsearch/Splunk/Datadog alternative for logs, metrics, traces, RUM, error tracking, and session replay.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
TODO write / organize together with the old log content?
incorporate into SUMMARY, etc
</div>