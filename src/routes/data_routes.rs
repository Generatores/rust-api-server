use actix_web::{
    error::ResponseError,
    get,
    http::{header::ContentType, StatusCode},
    post, put,
    web::Data,
    web::Json,
    web::Path,
    HttpResponse,
};
use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
struct Response {
    results: String,
}

#[get("/data")]
pub async fn get_datas() -> Json<Response> {
    return Json(Response {
        results: "endpoint reached".to_string(),
    });
}
