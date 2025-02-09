use warp::Filter;

#[tokio::main]
async fn main() {
    let port: u16 = std::env::var("PORT")
        .unwrap_or_else(|_| "25565".to_string())
        .parse()
        .expect("Invalid PORT");

    println!("Starting server on port: {}", port);

    let route = warp::path("play").map(|| warp::reply::html("Classic Minecraft JS Server Running!"));
     
    warp::serve(route).run(([0, 0, 0, 0], port)).await;
}
