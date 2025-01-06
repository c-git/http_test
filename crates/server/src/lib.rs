use actix_cors::Cors;
use actix_files::Files;
use actix_web::{
    web::{self, ServiceConfig},
    App, HttpRequest, HttpResponse, HttpServer,
};
use tracing::error;
use tracing_actix_web::TracingLogger;

/// This function is called once per worker
fn modify_service_config(cfg: &mut ServiceConfig) {
    cfg.service(Files::new("/", "dist").index_file("index.html"));
    cfg.default_service(web::route().to(not_found));
}

pub struct CustomShuttleService {}

#[shuttle_runtime::async_trait]
impl shuttle_runtime::Service for CustomShuttleService {
    async fn bind(mut self, addr: std::net::SocketAddr) -> Result<(), shuttle_runtime::Error> {
        run_server(addr).await?;
        Ok(())
    }
}

async fn run_server(addr: std::net::SocketAddr) -> std::io::Result<()> {
    let server = HttpServer::new(|| {
        App::new()
            .wrap(Cors::permissive())
            .wrap(TracingLogger::default())
            .configure(setup_closure())
    })
    .bind(addr)?
    .run();
    match tokio::spawn(server).await {
        Ok(server_outcome) => match server_outcome {
            Ok(()) => {}
            Err(err_msg) => error!(?err_msg, "server returned with error"),
        },
        Err(err_msg) => error!(?err_msg, "server task panicked"),
    };
    Ok(())
}

/// This function is called once and returns a closure that is called once per worker
pub fn setup_closure() -> impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static {
    // Code that should run exactly once

    // Closure that is returned
    |cfg: &mut ServiceConfig| {
        modify_service_config(cfg);
    }
}

#[tracing::instrument(name = "DEFAULT NOT FOUND HANDLER", level = "error")]
pub async fn not_found(req: HttpRequest) -> HttpResponse {
    HttpResponse::NotFound().body("404 - Not found\n")
}
