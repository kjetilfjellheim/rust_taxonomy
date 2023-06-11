extern crate diesel;

use crate::taxonomy::init_db;
use crate::taxonomy::{ get_specific_tsn, list_tsn };
use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use log4rs;
mod taxonomy;

///
/// Initalize and start the api.
///
#[actix_rt::main]
async fn main() -> Result<(), std::io::Error> {
    log4rs::init_file("resources/logging.yaml", Default::default()).unwrap();
    dotenv().ok();
    init_db();
    let server = HttpServer::new(|| App::new().configure(init_routes));
    let bind_result = server.bind(("127.0.0.1", 8080));
    match bind_result {
        Ok(addr) => addr.run().await,
        Err(err) => {
            panic!("Failed to setup server: {}", err)
        }
    }
}

///
/// Initialize all api routes.
///
pub fn init_routes(config: &mut web::ServiceConfig) {
    config.service(get_specific_tsn).service(list_tsn);
}
