extern crate diesel;

use dotenv::dotenv;
use actix_web::{App, web, HttpServer};

use crate::taxonomy::{ init_db };
use crate::taxonomy::list_longnames;

mod taxonomy;

#[actix_rt::main]
async fn main() -> Result<(), std::io::Error> {
    dotenv().ok();
    init_db();
    let server = HttpServer::new(|| App::new().configure(init_routes));
    let bind_result = server.bind(("127.0.0.1", 8080));
    match bind_result {
        Ok(addr) => { addr.run().await }
        Err(err) => { panic!("Failed to setup server: {}", err) }
    }
}

pub fn init_routes(config: &mut web::ServiceConfig) {
    config.service(list_longnames);
}
