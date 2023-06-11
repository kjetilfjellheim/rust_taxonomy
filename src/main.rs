extern crate diesel;

use crate::taxonomy::init_db;
use crate::taxonomy::{ list_longnames, get_longname };
use actix_web::{web, App, HttpServer};
use dotenv::dotenv;

mod taxonomy;

///
/// Initalize and start the api.
///
#[actix_rt::main]
async fn main() -> Result<(), std::io::Error> {
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
    config
        .service(list_longnames)
        .service(get_longname);

}
