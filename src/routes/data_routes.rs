use actix_web::{
  get,
  post,
  put,
  error::ResponseError,
  web::Path,
  web::Json,
  web::Data,
  HttpResponse,
  http::{header::ContentType, StatusCode}
};

#[get("/data")]
pub async fn get_datas() -> Json<String> {
  return Json("endpoint reached".to_string());
}