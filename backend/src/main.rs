mod api;
mod mqtt;
mod db;
use tokio;

#[tokio::main]
async fn main() {

   let pool = match db::initialize_db().await {
    Ok(pool) => pool,
    Err(e) => {
            eprintln!("Failed to initialize database: {:?}",e);    
            return;
        }
    };

    let client = mqtt::create_mqtt_client();

    let mqtt_task: tokio::task::JoinHandle<()> = tokio::spawn({
        let pool_clone = pool.clone(); 
        let client_clone: paho_mqtt::Client = client.clone();
        async move {
            mqtt::subscribe_to_topic(&client_clone, &pool_clone, "homestead/sensor").await 
        }
    });

    let server_task = tokio::spawn( {
         async move {
            api::start(pool, client).await
        }
    });
    
    println!("Starting Server and MQTT backend tasks...");
    let (mqtt_task_result, server_task_result) = tokio::join!(mqtt_task, server_task);

    if let Err(e) = server_task_result {
        eprintln!("Server task failed: {}", e);
    }

    match mqtt_task_result {
        Ok(()) => println!("MQTT task completed successfully"),
        Err(e) => eprintln!("MQTT task panicked: {}", e),
    }
 
}