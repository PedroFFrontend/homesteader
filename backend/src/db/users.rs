use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Serialize, Deserialize)]
pub struct AddUserBody {
    pub lat:f64, 
    pub lon:f64, 
    pub zone:i32
}

pub async fn create_table(pool: &PgPool) -> Result<(), sqlx::Error> {
    println!("Creating 'users' table if it doesn't exist");
    const CREATE_TABLE_QUERY : &str = r#"
        CREATE TABLE IF NOT EXISTS users (
            id SERIAL PRIMARY KEY,
            lat FLOAT8 NOT NULL,
            lon FLOAT8 NOT NULL,
            zone INT NOT NULL
        );
    "#;
    sqlx::query(CREATE_TABLE_QUERY)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn get_all_users(pool: &PgPool) -> Result<(), sqlx::Error> {
    println!("Creating 'users' table if it doesn't exist");
    const CREATE_TABLE_QUERY : &str = r#"
        SELECT * 
        FROM users
        ORDER BY id;
    "#;
    sqlx::query(CREATE_TABLE_QUERY)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn add_user(pool: &PgPool, lat:f64, lon: f64, zone: i32) -> Result<i32, sqlx::Error> {
    println!("Adding {lat} {lon} {zone}");
    const INSERT_QUERY: &str = r#"
        INSERT INTO users (lat, lon, zone)
        VALUES ($1, $2, $3)
        RETURNING id;
    "#;

    let id: (i32,) = sqlx::query_as(INSERT_QUERY)
        .bind(lat)
        .bind(lon)
        .bind(zone)
        .fetch_one(pool)
        .await.expect("add_user failed");

    Ok(id.0)
}
