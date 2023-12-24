# Tokio

Tokio is an asynchronous runtime for the Rust programming language. It provides the building blocks needed for writing networking applications.
Tokio provides a few major components:

- Multiple variations of the runtime for executing asynchronous code. Everything from a multi-threaded, work-stealing runtime to a light-weight, single-threaded runtime.
- An asynchronous version of the standard library.
- A large ecosystem of libraries.

## Key links

- [Tokio]( https://tokio.rs/ )
- [Tokio glossary]( https://tokio.rs/tokio/glossary )
- [Tokio tutorial]( https://tokio.rs/tokio/tutorial )
- [Tokio examples]( https://github.com/tokio-rs/tokio/tree/master/examples )
- [Tokio mini-Redis example]( https://github.com/tokio-rs/mini-redis )
- Template for a tokio-rs app with logging & command line argument parser: [rust-tokio-template]( https://github.com/Finomnis/rust-tokio-template/tree/main )

## Graceful shutdown

Example from [tokio_graceful_shutdown]( https://docs.rs/tokio-graceful-shutdown/latest/tokio_graceful_shutdown/ ):

```rust,ignore
use tokio_graceful_shutdown::{SubsystemBuilder, SubsystemHandle, Toplevel};
use tokio::time::{sleep, Duration};

type Result = Result<(), Box<dyn Error>>;

async fn countdown() {
    for i in (1..=5).rev() {
        tracing::info!("Shutting down in: {}", i);
        sleep(Duration::from_millis(1000)).await;
    }
}

async fn countdown_subsystem(subsys: SubsystemHandle) -> Result {
    tokio::select! {
        _ = subsys.on_shutdown_requested() => {
            tracing::info!("Countdown cancelled.");
        },
        _ = countdown() => {
            subsys.request_shutdown();
        }
    };

    Ok(())
}

#[tokio::main]
async fn main() -> Result {
    // Init logging
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::TRACE)
        .init();

    // Setup and execute subsystem tree
    Toplevel::new(|s| async move {
        s.start(SubsystemBuilder::new("Countdown", countdown_subsystem));
    })
    .catch_signals()  // signals the Toplevel object to listen for SIGINT/SIGTERM/Ctrl+C
    .handle_shutdown_requests(Duration::from_millis(1000))  // collects all the return values of the subsystems, determines the global error state
    .await
    .map_err(Into::into)
}
```