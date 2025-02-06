use warp::Filter;

const CACHE_CONTROL: &str = "Cache-Control";
const ACCESS_CONTROL: &str = "Access-Control-Allow-Origin";

#[tokio::main]
async fn main() {
    // Define the directory to serve files from with conditional caching
    let dir = warp::fs::dir("../target/dx/ore-app/release/web/public")
        .and(warp::path::full())
        .map(|reply, path: warp::path::FullPath| {
            let path_str = path.as_str();
            let reply = if should_cache(path_str) {
                warp::reply::with_header(reply, CACHE_CONTROL, "no-cache, must-revalidate")
            } else {
                warp::reply::with_header(reply, CACHE_CONTROL, "public, max-age=31536000, immutable")
            };
            warp::reply::with_header(reply, ACCESS_CONTROL, "*")
        });

    // Route to handle unknown paths
    let index = warp::path::end()
        .and(warp::fs::file("../target/dx/ore-app/release/web/public/index.html"))
        .map(|reply| {
            let reply = warp::reply::with_header(reply, CACHE_CONTROL, "public, max-age=31536000, immutable");
            warp::reply::with_header(reply, ACCESS_CONTROL, "*")
        });

    // Route to handle any other paths (fallback to index.html)
    let fallback = warp::any()
        .map(|| warp::reply::html(include_str!("../../target/dx/ore-app/release/web/public/index.html")))
        .map(|reply| {
            let reply = warp::reply::with_header(reply, CACHE_CONTROL, "public, max-age=31536000, immutable");
            warp::reply::with_header(reply, ACCESS_CONTROL, "*")
        });

    // Combine routes
    let routes = dir.or(index).or(fallback).with(warp::log("ore-app"));

    // Start the warp server
    warp::serve(routes).run(([0, 0, 0, 0], 8080)).await;
}

fn should_cache(path: &str) -> bool {
    // Don't cache .wasm or .js files
    if path.ends_with(".wasm") || path.ends_with(".js") {
        return false;
    }
    // Cache other static assets
    path.contains("assets/") || 
    path.ends_with(".css") ||
    path.ends_with(".png") ||
    path.ends_with(".jpg") ||
    path.ends_with(".jpeg") ||
    path.ends_with(".gif") ||
    path.ends_with(".webp") ||
    path.ends_with(".otf")
}
