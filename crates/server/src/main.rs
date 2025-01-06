use http_test_server::CustomShuttleService;
use tracing_subscriber::{
    fmt::{self, format::FmtSpan},
    prelude::*,
    EnvFilter,
};

#[shuttle_runtime::main]
async fn main() -> Result<CustomShuttleService, shuttle_runtime::Error> {
    tracing_subscriber::registry()
        .with(fmt::layer().with_span_events(FmtSpan::NEW))
        // .with(fmt::layer().with_span_events(FmtSpan::ACTIVE))
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")))
        .init();

    Ok(CustomShuttleService {})
}
