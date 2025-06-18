use mongodb::{Client, options::ClientOptions};
use std::sync::Arc;
use dotenv::dotenv;
use mongodb::bson::doc;

pub struct AppConfig {
    pub server_address: String,
    pub arc_client: Arc<Client>,
}

pub async fn load_config() -> Result<AppConfig, Box<dyn std::error::Error>> {
    dotenv().ok();

    // Load MongoDB Atlas connection
    let mongodb_uri = std::env::var("MONGOSTRING")
        .expect("MONGOSTRING must be set in .env file");
    
    let client_options = ClientOptions::parse(&mongodb_uri).await?;
    let client = Client::with_options(client_options)?;

    // Test connection
    client
        .database("admin")
        .run_command(doc! {"ping": 1}, None)
        .await?;
    println!("✅ Connected to MongoDB Atlas");

    Ok(AppConfig {
        server_address: std::env::var("SERVERADDRESS")?,
        arc_client: Arc::new(client),
    })
}