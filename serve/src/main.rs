use warp::Filter;

#[tokio::main]
async fn main() {
    // Define the directory to serve files from with conditional caching
    let dir = warp::fs::dir("../target/dx/ore-app/release/web/public")
        .and(warp::path::full())
        .map(|reply, path: warp::path::FullPath| {
            let path_str = path.as_str();
            let reply = if should_cache(path_str) {
                cache_headers(reply)
            } else {
                cache_headers_must_revalidate(reply)
            };
            cors_headers(reply)
        });

    // Route to handle unknown paths
    let index = warp::path::end()
        .and(warp::fs::file("../target/dx/ore-app/release/web/public/index.html"))
        .map(|reply| {
            let reply = cache_headers_must_revalidate(reply);
            cors_headers(reply)
        });

    // Route to handle any other paths (fallback to index.html)
    let fallback = warp::any()
        .map(|| warp::reply::html(include_str!("../../target/dx/ore-app/release/web/public/index.html")))
        .map(|reply| {
            let reply = cache_headers_must_revalidate(reply);
            cors_headers(reply)
        });

    // Combine routes
    let routes = dir.or(index).or(fallback).with(warp::log("ore-app"));

    // Start the warp server
    warp::serve(routes).run(([0, 0, 0, 0], 8080)).await;
}

fn cache_headers_must_revalidate(reply: warp::reply::Response) -> warp::reply::Response {
    warp::reply::with_header(reply, "Cache-Control", "no-cache, must-revalidate")
}

fn cache_headers(reply: warp::reply::Response) -> warp::reply::Response {
    warp::reply::with_header(reply, "Cache-Control", "public, max-age=31536000, immutable")
}

fn cors_headers(reply: warp::reply::Response) -> warp::reply::Response {
    warp::reply::with_header(reply, "Access-Control-Allow-Origin", "*")
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
