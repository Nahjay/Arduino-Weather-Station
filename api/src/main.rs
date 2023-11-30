/* Create the API for my Arduino Weather Station that will store information at Various Endpoints */
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use simplelog::{CombinedLogger, TermLogger, WriteLogger};
use std::fs::File;

// Create a Struct to hold the response data

// Create a struct to hold the response data
#[derive(Serialize)]
pub struct Response {
    pub message: String,
}
async fn main() -> std::io::Result<()> {
    /* Instantiate Logger */
    match CombinedLogger::init(vec![
        TermLogger::new(
            log::LevelFilter::Debug,
            simplelog::Config::default(),
            simplelog::TerminalMode::Mixed,
            simplelog::ColorChoice::Auto,
        ),
        WriteLogger::new(
            log::LevelFilter::Debug,
            simplelog::Config::default(),
            File::create("gps_rs.log").unwrap(),
        ),
    ]) {
        Ok(_) => debug!("Logger initialized"),
        Err(e) => debug!("Logger failed to initialize: {}", e),
    }

    // Create the Server and bind it to port 8084
    HttpServer::new(|| {
        App::new()
            // .route("/", web::get().to(HttpResponse::Ok))
            .service(healthcheck)
            .service(index)
            .service(date_time)
            .service(utc_time)
            .service(altitude)
            .service(lat_long)
            .service(all_serial_data)
            .default_service(web::route().to(not_found))
    })
    .bind(("127.0.0.1", 8084))?
    .run()
    .await
}
