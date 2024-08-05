use warp::Filter;

#[tokio::main]
async fn main() {
    // Define the directory to serve files from
    let dir = warp::fs::dir("../dist");

    // Route to handle unknown paths
    let index = warp::path::end().and(warp::fs::file("../dist/index.html"));

    // Route to handle any other paths (fallback to index.html)
    let fallback = warp::any().map(|| warp::reply::html(include_str!("../../dist/index.html")));

    // Combine routes
    let routes = dir.or(index).or(fallback).with(warp::log("ore-app"));

    // Start the warp server
    warp::serve(routes).run(([0, 0, 0, 0], 8080)).await;
}
