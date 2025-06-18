mod configs;
mod routes;
mod controllers;
mod models;

use axum::Router;
use dotenv::dotenv;
use configs::load_config;
use tokio::net::TcpListener;
use axum::serve;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let config = load_config().await?;
    println!("✅ Connected to MongoDB Atlas");
    println!("🚀 Server running on http://{}", config.server_address);

    let app: Router = routes::route(config.arc_client.clone());

    println!("🟡 Binding to address...");
    let listener = TcpListener::bind(&config.server_address).await?;
    println!("🟢 Bound to address, starting server...");
    serve(listener, app).await?;

    Ok(())
}
