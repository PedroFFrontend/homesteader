use paho_mqtt::{self as mqtt, Message};
use paho_mqtt::properties;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use tokio::{task::{self}, time::timeout};
use uuid::Uuid;
use std::time::Duration;
use crate::db;

#[derive(Serialize, Deserialize, Debug)]
struct SensorMessage {
    src_timestamp: i64,
    cpu_temp: String,
    cpu_volt: String
}

pub fn create_mqtt_client() -> mqtt::Client {
    let create_opts = mqtt::CreateOptionsBuilder::new()
        .server_uri("mosquitto:1883")
        .client_id("backend")
        .mqtt_version(mqtt::MQTT_VERSION_5)
        .finalize();

    let client = mqtt::Client::new(create_opts).unwrap();
    let conn_opts = mqtt::ConnectOptionsBuilder::new_v5()
        .keep_alive_interval(Duration::from_secs(20))
        .finalize();

    client.connect(conn_opts).unwrap();
    client
}


pub fn publish_to_topic(client: &mqtt::Client, _: &sqlx::Pool<sqlx::Postgres>, topic: &str, message:&str) {
    let msg: Message = Message::new(topic, message, mqtt::QOS_1);
    match client.publish(msg) {
        Ok(_) => (),
        Err(e) => eprintln!("Failed to publish to topic {topic}: {e}")
    }
}

pub fn request(client: &mqtt::Client, _: &sqlx::Pool<sqlx::Postgres>, topic: &str, payload: &str) -> String{
    let response_topic = format!("response/{}", Uuid::new_v4());
    let correlation_id = Uuid::new_v4().to_string();

    // Subscribe to the response topic
    client.subscribe(&response_topic, 1).expect("Error subscribing");

    // Start consuming messages
    let rx = client.start_consuming();
    let props = mqtt::properties![
        mqtt::PropertyCode::ResponseTopic => response_topic.to_string(),
        mqtt::PropertyCode::CorrelationData => correlation_id.to_string(),
    ];

    // Publish the request
     let message = mqtt::MessageBuilder::new()        
        .topic(topic)
        .payload(payload)
        .qos(1)
        .properties(props)
        .finalize();
    println!("sending: {:?}", message.properties());
    client.publish(message).expect("Error publishing request");

    // Wait for the response
    for msg in rx.iter() {
        if let Some(msg) = msg {
            if msg.topic() == response_topic {
                if let Some(corr_data) = msg.properties().get_binary(mqtt::PropertyCode::CorrelationData) {
                    if corr_data == correlation_id.as_bytes() {
                        println!("Received response: {}", msg.payload_str());
                        return msg.payload_str().to_lowercase();
                     }
                }
            }
        }
    }
    return "timeout".to_string();
}

pub async fn subscribe_to_topic(client: &mqtt::Client, pool: &sqlx::Pool<sqlx::Postgres>, topic: &str)  {
    println!("Subscribing to topic: {}",topic);
    let rx = client.start_consuming();
    client.subscribe(topic, 1).expect("subscribe failed");

    for msg in rx {
        let pool_clone = pool.clone();
        task::spawn(async move {
            match msg {
                Some(msg) => {
                    println!("Received: {}", msg.payload_str());
                    let parsed: SensorMessage = match serde_json::from_str(&msg.payload_str().replace("'", "\"")) {
                        Ok(parsed) => parsed,
                        Err(e) => {
                            eprintln!("Error parsing JSON: {}", e);
                            return;
                        }
                    };
                    println!("{:?}", parsed);
                    let src_timestamp = parsed.src_timestamp;
                    let cpu_temp: f64 = parsed.cpu_temp.parse().expect("CPU temp not parseable to float");
                    let cpu_volt: f64 = parsed.cpu_volt.parse().expect("CPU voltage not parseable to float");
                    db::sensors::add_entry(&pool_clone, src_timestamp, cpu_temp, cpu_volt).await;
                }
                None => {
                    println!("Lost connection");
                }
            }
        });
    }
}


async fn process_message(msg: mqtt::Message, pool: PgPool) {
    println!("Received: {}", msg.payload_str());
    
    let parsed: SensorMessage = match serde_json::from_str(&msg.payload_str().replace("'", "\"")) {
        Ok(parsed) => parsed,
        Err(e) => {
            eprintln!("Error parsing JSON: {}", e);
            return;
        }
    };

    let src_timestamp = parsed.src_timestamp;
    let cpu_temp: f64 = parsed.cpu_temp.parse().expect("CPU temp not parseable to float");
    let cpu_volt: f64 = parsed.cpu_volt.parse().expect("CPU voltage not parseable to float");

    // Use a timeout to prevent long-running transactions
    match timeout(Duration::from_secs(5), add_entry_with_transaction(&pool, src_timestamp, cpu_temp, cpu_volt)).await {
        Ok(result) => {
            if let Err(e) = result {
                eprintln!("Failed to add entry to database: {}", e);
            }
        },
        Err(_) => eprintln!("Database operation timed out"),
    }
}

async fn add_entry_with_transaction(pool: &PgPool, src_timestamp: i64, cpu_temp: f64, cpu_volt: f64) -> Result<(), sqlx::Error> {
    let mut tx = pool.begin().await?;

    sqlx::query(
        "INSERT INTO sensors (timestamp, cpu_temp, cpu_volt) VALUES ($1, $2, $3)",
    ).bind(src_timestamp)
    .bind(cpu_temp)
    .bind(cpu_volt)
    .execute(&mut tx)
    .await?;

    tx.commit().await?;
    Ok(())
}