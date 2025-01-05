#![warn(clippy::all, rust_2018_idioms, unused_crate_dependencies)]

#[cfg(not(target_arch = "wasm32"))]
mod suppress_used_in_bin {
    use tokio as _;
    use tracing_subscriber as _;
}

mod app;
pub use app::ui_request_test;
pub use app::TestApp;
