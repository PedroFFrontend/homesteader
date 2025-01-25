use sqlx::{Pool, Postgres};
use sqlx::postgres::PgPoolOptions;
use tokio::time::sleep;
use std::env;
use std::time::Duration;

pub mod sensors;
pub mod users;

#[derive(Debug, Clone)]
pub struct DbInitializationError;


async fn connect_to_db() -> Result<sqlx::PgPool, sqlx::Error> {
    // Load database URL from environment variable
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    println!("Initializing db at {database_url}");
    // Create a new PgPool with specified options


    let mut retries = 5;
    while retries > 0 {
        match PgPoolOptions::new()
            .max_connections(5)
            .connect(&database_url)
            .await 
        {
            Ok(pool) => {
                println!("Connected to the database!");
                // Your application logic here...
                return Ok(pool);
            }
            Err(e) => {
                println!("Failed to connect: {e}\n Retrying...");
                retries -= 1;
                sleep(Duration::from_secs(2)).await; // Wait before retrying
            }
        }
    }
    Err(sqlx::Error::PoolTimedOut)
}

pub async fn initialize_db() -> Result<Pool<Postgres>,DbInitializationError>{
    let pool = match connect_to_db().await {
        Ok(pool) => pool,
        Err(e) => {
            eprintln!("Failed to initialize database pool: {}", e);
            return Err(DbInitializationError); // Exit early or handle accordingly
        }
    };

    match users::create_table(&pool).await {
        Ok(_) => (),
        Err(e) => {
            eprintln!("Failed to create users table: {}", e);
            return Err(DbInitializationError); // Exit early or handle accordingly
        }
    }

    match sensors::create_table(&pool).await {
        Ok(_) => (),
        Err(e) => {
            eprintln!("Failed to create sensor table: {}", e);
            return Err(DbInitializationError); // Exit early or handle accordingly
        }
    }
    Ok(pool)
}





