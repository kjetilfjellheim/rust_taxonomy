extern crate diesel;

use crate::taxonomy::init_db;
use crate::taxonomy::{get_specific_tsn, list_tsn};
use actix_web::{middleware::Logger, web, App, HttpServer};
use dotenv;
use log4rs;
use std::env;
use actix_web_prom::{PrometheusMetricsBuilder};

mod taxonomy;

const LOG_FILE_PROP: &str = "LOG_FILE";
const SERVER_HOST: &str = "SERVER_HOST";
const SERVER_PORT: &str = "SERVER_PORT";

/**
 * Initalize and start the api.
 */
#[actix_rt::main]
async fn main() -> Result<(), std::io::Error> {
    // Initialize the environment variables.
    match dotenv::dotenv() {
        Ok(_val) => {}
        Err(err) => panic!("Failed to initalizing environment: {}", err),
    }
    // Initialize the logging environment
    match env::var(LOG_FILE_PROP) {
        Ok(val) => {
            log4rs::init_file(val, Default::default()).expect("Failed to initialize logging.");
        }
        Err(err) => panic!("Failed to initalizing logging: {}", err),
    }
    // Initialize the database connecrtion pool.
    init_db();
    // Initalize prometheus metrics
    let prometheus = PrometheusMetricsBuilder::new("api")
        .endpoint("/metrics/prometheus")
        .build()
        .unwrap();
    // Initalize route
    let server = HttpServer::new(move || App::new().wrap(prometheus.clone()).wrap(Logger::default()).configure(init_routes));
    // Get server host from environment.
    let server_host = match env::var(SERVER_HOST) {
        Ok(val) => val,
        Err(err) => panic!("Missing server host: {}", err),
    };
    // Get server port from environment.
    let server_port = match env::var(SERVER_PORT) {
        Ok(val) => val.parse::<u16>().expect("SERVER_PORT must be integer."),
        Err(err) => panic!("Missing server port: {}", err),
    };
    // Bind the server to listen.
    let bind_result = server.bind((server_host, server_port));
    // Start server
    match bind_result {
        Ok(addr) => addr.run().await,
        Err(err) => {
            panic!("Failed to setup server: {}", err)
        }
    }
}

/**
 * Initialize service routes.
 */
pub fn init_routes(config: &mut web::ServiceConfig) {
    config.service(get_specific_tsn).service(list_tsn);
}
