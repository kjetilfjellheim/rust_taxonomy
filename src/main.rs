extern crate diesel;

use crate::taxonomy::{find_taxonomies, find_taxonomy, find_taxonomy_hierarchy};
use crate::taxonomy::{get_connection_pool_status, init_db};
use actix_web::{middleware::Logger, web, App, HttpServer};
use actix_web_prom::{PrometheusMetrics, PrometheusMetricsBuilder};
use core::time::Duration;
use dotenv;
use log4rs;
use prometheus::IntGauge;
use std::env;
use std::thread;

mod taxonomy;

/// Property in .env for loggin configuration.
const LOG_FILE_PROP: &str = "LOG_FILE";
/// Property in .env for server host.
const SERVER_HOST: &str = "SERVER_HOST";
/// Property in .env for server port.
const SERVER_PORT: &str = "SERVER_PORT";

///
/// Initalize and start the api.
///
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
    // Initialize custom metrics
    let max_connections_gauge =
        IntGauge::new("max_connections", "Connection pool maximum").unwrap();
    let idle_connections_gauge = IntGauge::new("idle_connections", "Connection pool idle").unwrap();
    //Register custom prometheus metrics
    register_promethius_metrics(&prometheus, &max_connections_gauge);
    register_promethius_metrics(&prometheus, &idle_connections_gauge);
    // Start to gather db metrics
    gather_db_metrics(max_connections_gauge, idle_connections_gauge);
    // Initalize route
    let server = HttpServer::new(move || {
        App::new()
            .wrap(prometheus.clone())
            .wrap(Logger::default())
            .configure(init_routes)
    });
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

///
/// Initialize service routes.
///
fn init_routes(config: &mut web::ServiceConfig) {
    config.service(find_taxonomy).service(find_taxonomies).service(find_taxonomy_hierarchy);
}

///
/// Register prometheus metrics
///
fn register_promethius_metrics(
    prometheus_metrics: &PrometheusMetrics,
    gauge: &IntGauge,
) {
    prometheus_metrics
        .registry
        .register(Box::new(gauge.clone()))
        .unwrap();
}

///
/// Start monitoring process of db connections.
///
fn gather_db_metrics(
    max_connections_gauge: IntGauge,
    idle_connections_gauge: IntGauge,
) {
    thread::spawn(move || loop {
        let pool_state = get_connection_pool_status();
        max_connections_gauge.set(pool_state.0 as i64);
        idle_connections_gauge.set(pool_state.1 as i64);
        thread::sleep(Duration::from_secs(1));
    });
}
