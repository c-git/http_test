//! Based on https://httpbin.org/#/Cookies
use actix_web::{
    cookie::Cookie,
    http::header::LOCATION,
    web::{Json, Path, Query},
    HttpRequest, HttpResponse,
};
use anyhow::Context;
use serde::Deserialize;
use tracing::instrument;

#[derive(Deserialize)]
pub struct QueryData {
    stay: Option<String>,
}

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
pub async fn cookie_set(
    path: Path<(String, String)>,
    Query(QueryData { stay }): Query<QueryData>,
) -> HttpResponse {
    let (name, value) = path.into_inner();
    let cookie = Cookie::build(&name, &value).path("/").finish();
    if stay.is_none() {
        HttpResponse::SeeOther()
            .insert_header((LOCATION, "/cookies/"))
            .cookie(cookie)
            .finish()
    } else {
        HttpResponse::Ok()
            .cookie(cookie)
            .body(format!("set cookie: {name} = {value}"))
    }
}

#[instrument]
pub async fn cookie_expire(
    path: Path<String>,
    Query(QueryData { stay }): Query<QueryData>,
) -> HttpResponse {
    let name = path.into_inner();
    let mut cookie = Cookie::build(&name, "").path("/").finish();
    cookie.make_removal();
    if stay.is_none() {
        HttpResponse::SeeOther()
            .insert_header((LOCATION, "/cookies/"))
            .cookie(cookie)
            .finish()
    } else {
        HttpResponse::Ok()
            .cookie(cookie)
            .body(format!("removed cookie: {name}"))
    }
}
