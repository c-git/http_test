#![warn(clippy::all, rust_2018_idioms, unused_crate_dependencies)]

use env_logger as _; // Used in binary

mod app;
pub use app::ui_request_test;
pub use app::TestApp;
