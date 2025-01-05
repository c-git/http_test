use actix_web::web::ServiceConfig;
use shuttle_actix_web::ShuttleActixWeb;
use tracing_subscriber::{
    fmt::{self, format::FmtSpan},
    prelude::*,
    EnvFilter,
};

#[shuttle_runtime::main]
async fn main() -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    tracing_subscriber::registry()
        .with(fmt::layer().with_span_events(FmtSpan::NEW))
        // .with(fmt::layer().with_span_events(FmtSpan::ACTIVE))
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")))
        .init();

    Ok(http_test::setup_closure().into())
}
