use actix_web::{get, http, post, web::{self, Json}, App, HttpResponse, HttpServer, Responder};
use actix_cors::Cors;
use sqlx::PgPool;

use crate::db::{self, users::AddUserBody};

const ADDRESS : &str = "0.0.0.0:8080";

#[get("/")]
async fn get_data(pool: web::Data<PgPool>) -> impl Responder {
    println!("get_data called!");
    let data = match db::sensors::get_all_data(pool.get_ref()).await {
        Ok(data) => HttpResponse::Ok().json(data),
        Err(e) => {
            eprintln!("Internal server error: {e}");
            HttpResponse::InternalServerError().finish()
        },
    };
    println!("get_data finished");
    data
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

pub async fn start(pool: PgPool) {
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
            .app_data(web::Data::new(pool.clone())) // Share the pool across handlers
            .service(get_data)
            .service(health_check)
            .service(health_check_db)

            .service(add_user)
            // .route("/", web::get().to(get_data))
            //  .route("/add_user", web::post().to(add_user)) 

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