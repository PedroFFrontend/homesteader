use paho_mqtt as mqtt;
use std::time::Duration;

use crate::db;

pub fn create_mqtt_client() -> mqtt::Client {
    let create_opts = mqtt::CreateOptionsBuilder::new()
        .server_uri("mosquitto:1883")
        .client_id("backend")
        .finalize();

    let client = mqtt::Client::new(create_opts).unwrap();
    let conn_opts = mqtt::ConnectOptionsBuilder::new()
        .keep_alive_interval(Duration::from_secs(20))
        .clean_session(true)
        .finalize();

    client.connect(conn_opts).unwrap();
    client
}


pub async fn subscribe_to_topic(client: &mqtt::Client, pool: &sqlx::Pool<sqlx::Postgres>) {
    println!("Subscribing to topic: 'homestead/cpu'");
    let rx = client.start_consuming();
    client.subscribe("homestead/cpu", 1).expect("subscribe failed");
    let pool_clone = pool.clone();
    tokio::spawn(async move {
        for msg in rx {
            match msg {
                Some(msg) => {
                    println!("Received: {}", msg);
                    msg.payload();
                    let src_timestamp = 10;
                    let cpu_temp= 1.0;
                    let cpu_volt= 1.0;
                    if let Err(e) = db::add_entry(&pool_clone, src_timestamp, cpu_temp, cpu_volt).await {
                        eprintln!("Failed to add entry to database: {}", e);
                    }
                }
                None => {
                    println!("Lost connection");
                    break;
                }
            }
        }
    }).await;
}