use warp::{Filter, Reply};
use std::path::Path;

#[tokio::main]
async fn main() {
    // Define the directory to serve files from with conditional caching
    let dir = warp::fs::dir("../dist")
        .and(warp::path::full())
        .map(|reply, path: warp::path::FullPath| {
            let path_str = path.as_str();
            if should_cache(path_str) {
                warp::reply::with_header(
                    reply,
                    "Cache-Control",
                    "public, max-age=31536000, immutable"
                )
            } else {
                warp::reply::with_header(
                    reply,
                    "Cache-Control",
                    "no-cache, must-revalidate"
                )
            }
        });

    // Route to handle unknown paths
    let index = warp::path::end()
        .and(warp::fs::file("../dist/index.html"))
        .map(|reply| {
            warp::reply::with_header(
                reply,
                "Cache-Control",
                "no-cache, must-revalidate"
            )
        });

    // Route to handle any other paths (fallback to index.html)
    let fallback = warp::any()
        .map(|| warp::reply::html(include_str!("../../dist/index.html")))
        .map(|reply| {
            warp::reply::with_header(
                reply,
                "Cache-Control",
                "no-cache, must-revalidate"
            )
        });

    // Combine routes
    let routes = dir.or(index).or(fallback).with(warp::log("ore-app"));

    // Start the warp server
    warp::serve(routes).run(([0, 0, 0, 0], 8080)).await;
}

fn should_cache(path: &str) -> bool {
    // Don't cache .wasm files
    if path.ends_with(".wasm") || path.contains("ORE.js") {
        return false;
    }
    // Cache other static assets
    path.contains("assets/") || 
    path.ends_with(".js") ||
    path.ends_with(".css") ||
    path.ends_with(".png") ||
    path.ends_with(".jpg") ||
    path.ends_with(".jpeg") ||
    path.ends_with(".gif") ||
    path.ends_with(".webp")
}
