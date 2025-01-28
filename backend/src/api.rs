use actix_web::{get, http, post, web::{self, Json}, App, HttpResponse, HttpServer, Responder};
use actix_cors::Cors;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use crate::{db::{self, users::AddUserBody}, mqtt};
mod weather;

const ADDRESS : &str = "0.0.0.0:8080";

#[derive(Deserialize)]
struct GetWeatherQuery {
    lat: f64,
    lon: f64
}

#[get("/sensors")]
async fn get_data(pool: web::Data<PgPool>) -> impl Responder {
    let data = match db::sensors::get_all_data(pool.get_ref()).await {
        Ok(data) => HttpResponse::Ok().json(data),
        Err(e) => {
            eprintln!("Internal server error: {e}");
            HttpResponse::InternalServerError().finish()
        },
    };
    data
}

#[get("/actuator/state")]
async fn get_actuator_state(pool: web::Data<PgPool>, client:web::Data<paho_mqtt::Client>) -> impl Responder {
    return mqtt::request(&client,&pool, "homestead/actuator/get_state","");
}

#[derive(Serialize, Deserialize)]
struct SetActuatorStatePayload {
    new_state: bool
}

#[post("/actuator/state")]
async fn set_actuator_state(pool: web::Data<PgPool>, client:web::Data<paho_mqtt::Client>, body: Json<SetActuatorStatePayload>) -> impl Responder {
    let new_state = match body.new_state {
        true => "true",
        false => "false"
    };
    return mqtt::request(&client,&pool, "homestead/actuator/set_state", new_state);
}

#[post("/add_user")]
async fn add_user(pool: web::Data<PgPool>,body: Json<AddUserBody>) -> impl Responder {
     match db::users::add_user(pool.get_ref(), body.lat,body.lon,body.zone).await {
        Ok(data) => HttpResponse::Ok().json(data),
        Err(e) => {
            eprintln!("Internal server error: {e}");
            HttpResponse::InternalServerError().finish()
        },
    }
}

#[get("/health")]
async fn health_check(_: web::Data<PgPool>) -> impl Responder {
    HttpResponse::Ok().body("Database is responsive")
}

#[get("/health/db")]
async fn health_check_db(pool: web::Data<PgPool>) -> impl Responder {
    match sqlx::query("SELECT 1").fetch_one(pool.get_ref()).await {
        Ok(_) => HttpResponse::Ok().body("Database is responsive"),
        Err(e) => {
            eprintln!("Database health check failed: {}", e);
            HttpResponse::InternalServerError().body("Database is not responsive")
        }
    }
}

#[get("/weather/current")]
async fn get_current_weather(_: web::Data<PgPool>, query: web::Query<GetWeatherQuery>) -> impl Responder {
    match weather::fetch_current(query.lat, query.lon).await {
        Ok(response) => HttpResponse::Ok().body(serde_json::to_string(&response).unwrap()),
        Err(_) => HttpResponse::BadRequest().body(())
    }
}

#[get("/weather/forecast")]
async fn get_forecast_weather(_: web::Data<PgPool>, query: web::Query<GetWeatherQuery>) -> impl Responder {
    match weather::fetch_forecast(query.lat, query.lon).await {
        Ok(response) => HttpResponse::Ok().body(serde_json::to_string(&response).unwrap()),
        Err(e) => match e {
            weather::models::FetchForecastWeatherError::JsonParseError => HttpResponse::InternalServerError().body(()),
            weather::models::FetchForecastWeatherError::RequestError => HttpResponse::BadRequest().body(()),
            weather::models::FetchForecastWeatherError::BadStatusError(e) => HttpResponse::BadRequest().body(()),
        }
    }
}


pub async fn start(pool: PgPool, client: paho_mqtt::Client) {
    println!("Starting HTTP server at {ADDRESS}...");
    match HttpServer::new( move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:3000")
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
            .allowed_header(http::header::CONTENT_TYPE)
            .max_age(3600);

        App::new()
            .wrap(cors)
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(client.clone())) // Share the pool across handlers
            // Share the pool across handlers
            .service(get_data)
            .service(health_check)
            .service(health_check_db)
            .service(add_user)
            .service(get_current_weather)
            .service(get_forecast_weather)
            .service(get_actuator_state)
            .service(set_actuator_state)

    })
    .bind(ADDRESS)
    .expect("Failed to bind server")
    .run()
    .await
    {
        Ok(()) => {println!("Completed HTTP server task");}
        Err(e) => {eprintln!("Server failed: {e}");}
    }
}