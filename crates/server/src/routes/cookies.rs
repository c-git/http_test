//! Based on https://httpbin.org/#/Cookies
use actix_web::{
    cookie::Cookie,
    http::header::LOCATION,
    web::{Json, Path},
    HttpRequest, HttpResponse,
};
use anyhow::Context;
use tracing::instrument;

#[instrument]
pub async fn cookie_show(req: HttpRequest) -> crate::Result<Json<Vec<(String, String)>>> {
    let mut result = vec![];
    let cookies = req.cookies().context("failed to access list of cookies")?;
    for cookie in cookies.iter() {
        result.push((cookie.name().to_string(), cookie.value().to_string()));
    }
    Ok(Json(result))
}

#[instrument]
pub async fn cookie_set(path: Path<(String, String)>) -> HttpResponse {
    let (name, value) = path.into_inner();
    let cookie = Cookie::build(name, value).path("/").finish();
    HttpResponse::SeeOther()
        .insert_header((LOCATION, "/cookies/"))
        .cookie(cookie)
        .finish()
}

#[instrument]
pub async fn cookie_expire(path: Path<String>) -> HttpResponse {
    let name = path.into_inner();
    let mut cookie = Cookie::build(name, "").path("/").finish();
    cookie.make_removal();
    HttpResponse::SeeOther()
        .insert_header((LOCATION, "/cookies/"))
        .cookie(cookie)
        .finish()
}
