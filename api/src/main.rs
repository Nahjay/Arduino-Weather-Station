/* Create the API for my Arduino Weather Station that will store information at Various Endpoints */
use actix_cors::Cors;
use actix_files::Files;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use log::{debug, error};
use rusqlite::{params, Connection, Result};
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
    data: String,
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
        Some(data) => {
            let mut formatted_data = Vec::new();
            for line in data.data.lines() {
                let parts: Vec<&str> = line.split(":").collect();
                if parts.len() == 2 {
                    let key = parts[0].trim();
                    let value = parts[1].trim();
                    formatted_data.push(format!("{}: {}", key, value));
                }
            }

            // Respond with the formatted data
            HttpResponse::Ok().json(formatted_data)
        }
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

    // Initialize the database if not already done
    if let Err(err) = initialize_database() {
        error!("Failed to initialize the database: {}", err);
        return HttpResponse::InternalServerError().finish();
    }

    // Establish a connection to the SQLite database
    let conn = Connection::open("../db/weather_data.db");
    let conn = match conn {
        Ok(conn) => conn,
        Err(err) => {
            error!("Failed to open database connection: {}", err);
            return HttpResponse::InternalServerError().finish();
        }
    };

    // Insert data into the table (adjust the SQL statement according to your table structure)
    let sql = "INSERT INTO weather_data (data) VALUES (?)";
    if let Err(err) = conn.execute(sql, params![data.0.data]) {
        error!("Failed to insert data into the database: {}", err);
        return HttpResponse::InternalServerError().finish();
    }

    // Parse the JSON data
    for line in data.0.data.lines() {
        // Store in key-value pairs
        let parts: Vec<&str> = line.split(":").collect();
        if parts.len() == 2 {
            let key = parts[0].trim();
            let value = parts[1].trim();

            // Now you can do something with the key and value
            println!("Key: {}, Value: {}", key, value);
        }
    }

    // Add a debug statement to check if the data is being stored
    debug!("Weather data stored: {:?}", data);

    HttpResponse::Ok().json(Response {
        message: "Weather data stored successfully".to_string(),
    })
}

fn initialize_database() -> Result<()> {
    // Create the database and create a new database if it does not exist
    let conn = Connection::open("../db/weather_data.db")?;

    // Read my init db file and execute the SQL statements
    let init_db_file = std::fs::read_to_string("../db/init_db.sql").expect("Unable to read file");
    println!("init_db_file: {}", init_db_file);
    conn.execute_batch(&init_db_file)?;

    Ok(())
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
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header();

        App::new()
            .wrap(cors)
            .app_data(web::Data::new(app_state.clone()))
            .service(healthcheck)
            .service(weather)
            .service(temperature)
            .service(humidity)
            .service(pressure)
            .service(altitude)
            .service(light)
            .service(time)
            .service(post_weather)
            .default_service(web::route().to(not_found))
            // Serve static files from the static directory
            .service(Files::new("/", "../web_interface").index_file("static/index.html"))
    })
    .bind(("192.168.0.105", 8084))?
    .run()
    .await
}
