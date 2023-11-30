/* Create the API for my Arduino Weather Station that will store information at Various Endpoints */
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use log::{debug, error, warn};
use serde::Serialize;
use simplelog::{CombinedLogger, TermLogger, WriteLogger};
use std::fs::File;

// Create a struct to hold the response data
#[derive(Serialize)]
pub struct Response {
    pub message: String,
}

// Creates a handler function that responds if the endpoint is not found in the server
async fn not_found() -> Result<HttpResponse, actix_web::Error> {
    let response = Response {
        message: "Resource not found".to_string(),
    };
    Ok(HttpResponse::NotFound().json(response))
}

// Creates a handler function for the /health endpoint
#[get("/health")]
async fn healthcheck() -> impl Responder {
    let response = Response {
        message: "Everything is working fine".to_string(),
    };
    HttpResponse::Ok().json(response)
}
#[get("/")]
async fn index() -> impl Responder {
    let response = Response {
        message: "Root endpoint".to_string(),
    };
    HttpResponse::Ok().json(response)
}

#[get("/weather")]
async fn weather() -> impl Responder {
    let response = Response {
        message: " All Weather Information endpoint".to_string(),
    };
    HttpResponse::Ok().json(response)
}

#[actix_web::main]
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
            .default_service(web::route().to(not_found))
    })
    .bind(("127.0.0.1", 8084))?
    .run()
    .await
}
