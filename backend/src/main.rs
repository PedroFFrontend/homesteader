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

    // // Create a separate connection pool for MQTT message processing
    // let mqtt_pool = match db::initialize_db().await {
    //     Ok(pool) => pool,
    //     Err(e) => {
    //         eprintln!("Failed to initialize MQTT database pool: {:?}", e);
    //         return;
    //     }
    // };

    // Test database connection after initialization
    if let Err(e) = sqlx::query("SELECT 1").fetch_one(&pool).await {
        eprintln!("Database connection test after initialization failed: {}", e);
        return;
    }

    let mqtt_task: tokio::task::JoinHandle<()> = tokio::spawn({
        let pool_clone = pool.clone(); 
        let client = mqtt::create_mqtt_client();
        async move {
            mqtt::subscribe_to_topic(&client, &pool_clone).await 
        }
    });

    let server_task = tokio::spawn( async {
        api::start(pool).await
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