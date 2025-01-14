mod api;
mod mqtt;
mod db;
use tokio;

#[tokio::main]
async fn main() {

    let pool = match db::init_pg_pool().await {
        Ok(pool) => pool,
        Err(e) => {
            eprintln!("Failed to initialize database pool: {}", e);
            return; // Exit early or handle accordingly
        }
    };
    match db::create_table(&pool).await {
        Ok(_) => (),
        Err(e) => {
            eprintln!("Failed to create database table: {}", e);
            return; // Exit early or handle accordingly
        }
    }

    let mqtt_task: tokio::task::JoinHandle<()> = tokio::spawn({
        let pool_clone = pool.clone(); // Clone here
        let client = mqtt::create_mqtt_client();
        async move {
            mqtt::subscribe_to_topic(client, pool_clone).await; // Use cloned pool
        }
    });

    let server_task = tokio::spawn( async {
        api::start_task(pool).await
    });
    
    let (mqtt_task_result, server_task_result) = tokio::join!(mqtt_task, server_task);
    if let Err(e) = server_task_result {
        eprintln!("Server task failed: {e}");
    }
    if let Err(e) = mqtt_task_result {
        eprintln!("MQTT task failed: {e}");
    }
}