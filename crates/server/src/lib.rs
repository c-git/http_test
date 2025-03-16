use std::collections::HashMap;

use actix_cors::Cors;
use actix_files::Files;
use actix_web::{
    web::{self, scope, ServiceConfig},
    App, HttpRequest, HttpResponse, HttpServer, ResponseError,
};
use routes::{cookie_expire, cookie_set, cookie_show};
use thiserror::Error;
use tracing::{error, instrument};
use tracing_actix_web::TracingLogger;

mod routes;

#[derive(Error, Debug)]
#[error(transparent)]
pub struct HandlerError(#[from] anyhow::Error);
pub type Result<T, E = HandlerError> = core::result::Result<T, E>;

/// This function is called once per worker
fn modify_service_config(cfg: &mut ServiceConfig) {
    cfg.service(scope("/echo").default_service(web::route().to(echo_handler)))
        .service(scope("/echo_raw").default_service(web::route().to(echo_raw_handler)))
        .service(
            scope("/cookies")
                .route("/", web::get().to(cookie_show))
                .route("/delete/{name}", web::get().to(cookie_expire))
                .route("/set/{name}/{value}", web::get().to(cookie_set)),
        )
        .service(Files::new("/", "dist").index_file("index.html"))
        .default_service(web::route().to(not_found));
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

impl ResponseError for HandlerError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        actix_web::http::StatusCode::IM_A_TEAPOT
    }
}

#[instrument]
pub async fn echo_raw_handler(req: HttpRequest, bytes: web::Bytes) -> HttpResponse {
    HttpResponse::Ok().body(format!(
        "\
ECHO RAW RESPONSE

-- req --
{req:#?}
--------------------------------------------------------

-- bytes --
{bytes:#?}"
    ))
}

#[instrument]
pub async fn echo_handler(
    req: HttpRequest,
    json: Option<web::Json<serde_json::Value>>,
    form: Option<web::Form<HashMap<String, String>>>,
) -> HttpResponse {
    HttpResponse::Ok().body(format!(
        "\
ECHO RESPONSE

-- req --
{req:#?}
--------------------------------------------------------

-- form --
{form:#?}
--------------------------------------------------------

-- json --
{json:#?}

"
    ))
}
