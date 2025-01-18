use std::time::Duration;

use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, PgPool};


#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct SensorData {
    id: i32,
    src_timestamp: i64,
    cpu_temp: f64,
    cpu_volt: f64,
}



pub async fn create_table(pool: &PgPool) -> Result<(), sqlx::Error> {
    println!("Creating 'sensors' table if it doesn't exist");
    const CREATE_TABLE_QUERY : &str = r#"
        CREATE TABLE IF NOT EXISTS sensors (
            id SERIAL PRIMARY KEY,
            dst_timestamp BIGINT DEFAULT EXTRACT(EPOCH FROM NOW())::BIGINT,
            src_timestamp BIGINT NOT NULL,
            cpu_temp FLOAT8 NOT NULL,
            cpu_volt FLOAT8 NOT NULL
        );
    "#;
    sqlx::query(CREATE_TABLE_QUERY)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn get_all_data(pool: &PgPool) -> Result<Vec<SensorData>, sqlx::Error> {
    println!("Fetching all data");
     const FETCH_QUERY: &str = r#"
        SELECT * 
        FROM sensors
        ORDER BY id;
     "#;
    match sqlx::query_as::<_, SensorData>(FETCH_QUERY)
        .fetch_all(pool)
        .await {
            Ok(rows) => {
                let length = rows.len();
                println!("Fetched all {length} rows");
                Ok(rows)},
            Err(e) => {
                eprintln!("Error fetching all data: {e}");
                Err(e)
            }
        }
}

pub async fn add_entry(pool: &PgPool, src_timestamp: i64, cpu_temp: f64, cpu_volt: f64) {
    let mut tx = pool.begin().await.expect("pool begin failed");

    println!("Adding {src_timestamp} {cpu_temp} {cpu_volt}");
    const INSERT_QUERY: &str = r#"
        INSERT INTO sensors (src_timestamp, cpu_temp, cpu_volt)
        VALUES ($1, $2, $3)
        RETURNING id;
    "#;

    sqlx::query(INSERT_QUERY)
        .bind(src_timestamp)
        .bind(cpu_temp)
        .bind(cpu_volt)
        .execute(&mut tx)
        .await.expect("add_entry failed");

    tx.commit().await.expect("commit failed");
}
