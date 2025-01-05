use actix_files::Files;
use actix_web::{
    middleware::Logger,
    web::{self, ServiceConfig},
    HttpRequest, HttpResponse,
};

/// This function is called once per worker
fn modify_service_config(cfg: &mut ServiceConfig) {
    // TODO 2: Disable CORS
    cfg.service(Files::new("/", "dist").index_file("index.html"));
    cfg.default_service(web::route().to(not_found).wrap(Logger::default()));
}

/// This function is called once and returns a closure that is called once per worker
pub fn setup_closure() -> impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static {
    // Code that should run exactly once

    // Closure that is returned
    |cfg: &mut ServiceConfig| {
        modify_service_config(cfg);
    }
}

#[tracing::instrument(name = "DEFAULT NOT FOUND HANDLER", ret, level = "error")]
pub async fn not_found(req: HttpRequest) -> HttpResponse {
    HttpResponse::NotFound().body("404 - Not found\n")
}
