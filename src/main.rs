use warp::Filter;

#[tokio::main]
async fn main() {
    let route = warp::path::end()
        .map(|| warp::reply::html("<h1>Classic Minecraft Server Running!</h1>"));

    let port: u16 = std::env::var("PORT")
        .unwrap_or_else(|_| "10000".to_string()) // Use Render's port
        .parse()
        .expect("Invalid PORT");

    println!("Starting server on port {}", port);
    warp::serve(route).run(([0, 0, 0, 0], port)).await;
}
