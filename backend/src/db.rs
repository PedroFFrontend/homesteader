use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct SensorData {
    id: i32,
    src_timestamp: i64,
    cpu_temp: f64,
    cpu_volt: f64,
}

use sqlx::postgres::PgPoolOptions;
use tokio::time::sleep;
use std::env;
use std::time::Duration;

pub async fn init_pg_pool() -> Result<sqlx::PgPool, sqlx::Error> {
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



    // let pool = PgPoolOptions::new()
    //     .max_connections(5) // Set the maximum number of connections
    //     .connect(&database_url) // Connect to the database
    //     .await?;
    
    Err(sqlx::Error::PoolTimedOut)
}


pub async fn create_table(pool: &PgPool) -> Result<(), sqlx::Error> {
    const CREATE_TABLE_QUERY : &str = r#"
        CREATE TABLE IF NOT EXISTS sensor_data (
            id SERIAL PRIMARY KEY,
            dst_timestamp BIGINT DEFAULT EXTRACT(EPOCH FROM NOW())::BIGINT,
            src_timestamp BIGINT NOT NULL,
            cpu_temp REAL NOT NULL,
            cpu_volt REAL NOT NULL
        );
    "#;

    sqlx::query(CREATE_TABLE_QUERY)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn fetch_all_data(pool: &PgPool) -> Result<Vec<SensorData>, sqlx::Error> {
    const FETCH_QUERY: &str = "SELECT id, src_timestamp, cpu_temp, cpu_volt FROM sensor_data;";
    let rows = sqlx::query_as::<_, SensorData>(FETCH_QUERY)
        .fetch_all(pool)
        .await?;

    Ok(rows)
}

pub async fn add_entry(pool: &PgPool, src_timestamp: i64, cpu_temp: f64, cpu_volt: f64) -> Result<i32, sqlx::Error> {
    const INSERT_QUERY: &str = r#"
        INSERT INTO sensor_data (src_timestamp, cpu_temp, cpu_volt)
        VALUES ($1, $2, $3)
        RETURNING id;
    "#;

    let id: (i32,) = sqlx::query_as(INSERT_QUERY)
        .bind(src_timestamp)
        .bind(cpu_temp)
        .bind(cpu_volt)
        .fetch_one(pool)
        .await?;

    Ok(id.0)
}
