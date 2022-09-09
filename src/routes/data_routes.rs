use actix_web::{
    error::ResponseError,
    get,
    http::{header::ContentType, StatusCode},
    post, put,
    web::Data,
    web::Json,
    web::Path,
    HttpResponse, Responder, Result,
};
use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
struct Response {
    results: String,
}

#[get("/data")]
pub async fn get_datas() -> Result<impl Responder> {
    let my_response_object = Response {
        results: "endpoint reached...".to_string(),
    };

    return Ok(Json(my_response_object));
}
