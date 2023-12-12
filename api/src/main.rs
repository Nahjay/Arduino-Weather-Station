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

// Create structs to hold the response data
#[derive(Serialize)]
pub struct Response {
    pub message: String,
}
#[derive(Serialize, Deserialize)]
pub struct Temperature {
    pub temperature: String,
}

#[derive(Serialize, Deserialize)]
pub struct Humidity {
    pub humidity: String,
}

#[derive(Serialize, Deserialize)]
pub struct Pressure {
    pub pressure: String,
}

#[derive(Serialize, Deserialize)]
pub struct Altitude {
    pub altitude: String,
}

#[derive(Serialize, Deserialize)]
pub struct Light {
    pub light: String,
}

#[derive(Serialize, Deserialize)]
pub struct Time {
    pub time: String,
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
                let parts: Vec<&str> = line.splitn(2, ':').map(str::trim).collect();
                if parts.len() == 2 {
                    let key = parts[0].trim();
                    let value = parts[1].trim();
                    formatted_data.push(format!("{}: {}", key, value));
                } else if line.contains("Pressure") {
                    let pressure_parts: Vec<&str> = line.split('=').map(str::trim).collect();
                    if pressure_parts.len() == 2 {
                        let pressure_key = pressure_parts[0];
                        let pressure_value = pressure_parts[1];
                        formatted_data.push(format!("{}: {}", pressure_key, pressure_value))
                    }
                }
            }

            // Remove the last 12 elements from the vector
            formatted_data.truncate(formatted_data.len() - 12);

            // Respond with the formatted data
            HttpResponse::Ok().json(formatted_data)
        }
        None => HttpResponse::NotFound().json(Response {
            message: "Weather data not available".to_string(),
        }),
    }
}

#[get("/weather/temperature")]
async fn temperature(state: web::Data<Arc<AppState>>) -> impl Responder {
    let app_state = state.weather_data.lock().unwrap();
    match &*app_state {
        Some(data) => {
            for line in data.data.lines() {
                let parts: Vec<&str> = line.split(":").collect();
                if parts.len() == 2 {
                    let key = parts[0].trim();
                    let value = parts[1].trim();
                    if key == "Temperature" {
                        // Respond with the temperature value
                        return HttpResponse::Ok().json(Temperature {
                            temperature: value.to_string(),
                        });
                    }
                }
            }

            // If "Temperature" information is not found
            HttpResponse::NotFound().json(Response {
                message: "Temperature information not available".to_string(),
            })
        }
        None => HttpResponse::NotFound().json(Response {
            message: "Weather data not available".to_string(),
        }),
    }
}

#[get("/weather/humidity")]
async fn humidity(state: web::Data<Arc<AppState>>) -> impl Responder {
    let app_state = state.weather_data.lock().unwrap();
    match &*app_state {
        Some(data) => {
            for line in data.data.lines() {
                let parts: Vec<&str> = line.split(":").collect();
                if parts.len() == 2 {
                    let key = parts[0].trim();
                    let value = parts[1].trim();
                    if key == "Humidity" {
                        // Respond with the humidity value
                        return HttpResponse::Ok().json(Humidity {
                            humidity: value.to_string(),
                        });
                    }
                }
            }

            // If "Humidity" information is not found
            HttpResponse::NotFound().json(Response {
                message: "Humidity information not available".to_string(),
            })
        }
        None => HttpResponse::NotFound().json(Response {
            message: "Weather data not available".to_string(),
        }),
    }
}

#[get("/weather/pressure")]
async fn pressure(state: web::Data<Arc<AppState>>) -> impl Responder {
    let app_state = state.weather_data.lock().unwrap();
    match &*app_state {
        Some(data) => {
            for line in data.data.lines() {
                if line.contains("Pressure") {
                    let pressure_parts: Vec<&str> = line.split('=').map(str::trim).collect();
                    if pressure_parts.len() == 2 {
                        let pressure_value = pressure_parts[1];

                        // Respond with the pressure value
                        return HttpResponse::Ok().json(Pressure {
                            pressure: pressure_value.to_string(),
                        });
                    }
                }
            }
            // If "Pressure" information is not found
            HttpResponse::NotFound().json(Response {
                message: "Pressure information not available".to_string(),
            })
        }
        None => HttpResponse::NotFound().json(Response {
            message: "Weather data not available".to_string(),
        }),
    }
}

#[get("/weather/altitude")]
async fn altitude(state: web::Data<Arc<AppState>>) -> impl Responder {
    let app_state = state.weather_data.lock().unwrap();
    match &*app_state {
        Some(data) => {
            for line in data.data.lines() {
                let parts: Vec<&str> = line.split(":").collect();
                if parts.len() == 2 {
                    let key = parts[0].trim();
                    let value = parts[1].trim();
                    if key == "Altitude" {
                        // Respond with the altitude value
                        return HttpResponse::Ok().json(Altitude {
                            altitude: value.to_string(),
                        });
                    }
                }
            }

            // If "Altitude" information is not found
            HttpResponse::NotFound().json(Response {
                message: "Altitude information not available".to_string(),
            })
        }
        None => HttpResponse::NotFound().json(Response {
            message: "Weather data not available".to_string(),
        }),
    }
}

#[get("/weather/light")]
async fn light(state: web::Data<Arc<AppState>>) -> impl Responder {
    let app_state = state.weather_data.lock().unwrap();
    match &*app_state {
        Some(data) => {
            for line in data.data.lines() {
                let parts: Vec<&str> = line.split(":").collect();
                if parts.len() == 2 {
                    let key = parts[0].trim();
                    let value = parts[1].trim();
                    if key == "Light" {
                        // Respond with the light value
                        return HttpResponse::Ok().json(Light {
                            light: value.to_string(),
                        });
                    }
                }
            }

            // If "Light" information is not found
            HttpResponse::NotFound().json(Response {
                message: "Light information not available".to_string(),
            })
        }
        None => HttpResponse::NotFound().json(Response {
            message: "Weather data not available".to_string(),
        }),
    }
}

#[get("/weather/time")]
async fn time(state: web::Data<Arc<AppState>>) -> impl Responder {
    let app_state = state.weather_data.lock().unwrap();
    match &*app_state {
        Some(data) => {
            for line in data.data.lines() {
                let parts: Vec<&str> = line.splitn(2, ':').map(str::trim).collect();
                if parts.len() == 2 {
                    let key = parts[0].trim();
                    let value = parts[1].trim();
                    if key == "Time" {
                        // Respond with the time value
                        return HttpResponse::Ok().json(Time {
                            time: value.to_string(),
                        });
                    }
                }
            }

            // If "Time" information is not found
            HttpResponse::NotFound().json(Response {
                message: "Time information not available".to_string(),
            })
        }
        None => HttpResponse::NotFound().json(Response {
            message: "Weather data not available".to_string(),
        }),
    }
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

    // Insert data into the table
    let sql = "INSERT INTO weather_data (data) VALUES (?)";
    if let Err(err) = conn.execute(sql, params![data.0.data]) {
        error!("Failed to insert data into the database: {}", err);
        return HttpResponse::InternalServerError().finish();
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
