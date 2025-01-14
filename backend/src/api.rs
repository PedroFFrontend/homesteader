use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use sqlx::PgPool;

use crate::db;

  
const ADDRESS : &str = "0.0.0.0:8080";

async fn get_data(pool: web::Data<PgPool>) -> impl Responder {
    println!("get_data called!");
    match db::fetch_all_data(pool.get_ref()).await {
        Ok(data) => HttpResponse::Ok().json(data),
        Err(e) => {
            eprintln!("Internal server error: {e}");
            HttpResponse::InternalServerError().finish()
        },
    }
}
pub async fn start_task(pool: PgPool) {
    println!("Starting HTTP server at {ADDRESS}...");
    match HttpServer::new( move || {
        App::new()
            .app_data(web::Data::new(pool.clone())) // Share the pool across handlers
            .route("/", web::get().to(get_data)) // Define the route to get data
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