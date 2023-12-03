mod blockchain;
mod config;
mod crypto;
mod errors;
mod handlers;
mod models;
mod routes;

#[tokio::main]
async fn main() {
    println!("DID Server is starting...");

    // Load server configuration
    let config = config::Config::new();

    // Set up routes using warp
    let routes = routes::configure_routes();

    // Start the server
    warp::serve(routes).run(([127, 0, 0, 1], 8080)).await;
}
