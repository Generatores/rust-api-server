pub mod config;
pub mod models;
mod routes;

use config::db_connection::MongoRepo;
use routes::data_routes::get_data;

use actix_web::{middleware::Logger, web::Data, App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    let db = MongoRepo::init().await;
    let db_data = Data::new(db);

    HttpServer::new(move || {
        let logger = Logger::default();
        App::new()
            .app_data(db_data.clone())
            .wrap(logger)
            .service(get_data)
    })
    .bind(("127.0.0.1", 3030))?
    .run()
    .await
}
