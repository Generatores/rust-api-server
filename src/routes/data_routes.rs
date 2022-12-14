use crate::config::db_connection::MongoRepo;

use actix_web::{
    get,
    web::{Data, Path},
    HttpResponse,
};

#[get("/data/{id}")]
pub async fn get_data(db: Data<MongoRepo>, path: Path<String>) -> HttpResponse {
    let id = path.into_inner();
    if id.is_empty() {
        return HttpResponse::BadRequest().body("invalid ID");
    }
    let function_model = db.get_data(&id).await;
    match function_model {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[get("/data")]
pub async fn get_all_data(db: Data<MongoRepo>) -> HttpResponse {
    let datas = db.get_all_data().await;
    match datas {
        Ok(datas) => HttpResponse::Ok().json(datas),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
