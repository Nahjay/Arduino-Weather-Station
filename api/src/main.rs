/* Create the API for my Arduino Weather Station that will store information at Various Endpoints */
use actix_files::Files;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use log::{debug, error, warn};
use serde::{Deserialize, Serialize};
use simplelog::{CombinedLogger, TermLogger, WriteLogger};
use std::fs::File;
use std::sync::{Arc, Mutex};

// Create a struct to hold the response data
#[derive(Serialize)]
pub struct Response {
    pub message: String,
}

#[derive(Default)]
struct AppState {
    weather_data: Mutex<Option<WeatherData>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WeatherData {
    pub temperature: f32,
    pub humidity: f32,
    // Add other fields as needed
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
async fn weather(state: web::Data<Arc<AppState>>) -> impl Responder {
    let app_state = state.weather_data.lock().unwrap();
    match &*app_state {
        Some(data) => HttpResponse::Ok().json(data),
        None => HttpResponse::NotFound().json(Response {
            message: "Weather data not available".to_string(),
        }),
    }
}

#[get("/weather/temperature")]
async fn temperature() -> impl Responder {
    let response = Response {
        message: "Temperature endpoint".to_string(),
    };
    HttpResponse::Ok().json(response)
}

#[get("/weather/humidity")]
async fn humidity() -> impl Responder {
    let response = Response {
        message: "Humidity endpoint".to_string(),
    };
    HttpResponse::Ok().json(response)
}

#[get("/weather/pressure")]
async fn pressure() -> impl Responder {
    let response = Response {
        message: "Pressure endpoint".to_string(),
    };
    HttpResponse::Ok().json(response)
}

#[get("/weather/altitude")]
async fn altitude() -> impl Responder {
    let response = Response {
        message: "Altitude endpoint".to_string(),
    };
    HttpResponse::Ok().json(response)
}

#[get("/weather/light")]
async fn light() -> impl Responder {
    let response = Response {
        message: "Light endpoint".to_string(),
    };
    HttpResponse::Ok().json(response)
}

#[get("/weather/time")]
async fn time() -> impl Responder {
    let response = Response {
        message: "Time endpoint".to_string(),
    };
    HttpResponse::Ok().json(response)
}

#[post("/post_weather")]
async fn post_weather(
    data: web::Json<WeatherData>,
    state: web::Data<Arc<AppState>>,
) -> impl Responder {
    let mut app_state = state.weather_data.lock().unwrap();
    *app_state = Some(data.0.clone());

    // Add a debug statement to check if the data is being stored
    debug!("Weather data stored: {:?}", data);

    HttpResponse::Ok().json(Response {
        message: "Weather data stored successfully".to_string(),
    })
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
            File::create("weather_station_rs.log").unwrap(),
        ),
    ]) {
        Ok(_) => debug!("Logger initialized"),
        Err(e) => debug!("Logger failed to initialize: {}", e),
    }

    let app_state = Arc::new(AppState::default());

    // Create the Server and bind it to port 8084
    HttpServer::new(move || {
        App::new()
            // .route("/", web::get().to(HttpResponse::Ok))
            .app_data(web::Data::new(app_state.clone()))
            .service(healthcheck)
            .service(index)
            .service(weather)
            .service(temperature)
            .service(humidity)
            .service(pressure)
            .service(altitude)
            .service(light)
            .service(time)
            .service(post_weather)
            .default_service(web::route().to(not_found))
            .service(Files::new("/", "./static/").index_file("index.html"))
    })
    .bind(("127.0.0.1", 8084))?
    .run()
    .await
}
