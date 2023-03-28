use std::sync::{Arc, RwLock};

use config::Config;

// mod api;
mod config;

#[tokio::main]
async fn main() {
    let config = Arc::new(RwLock::new(Config::new("./config.json")));
    // let file = File::open()

    // let config = Arc::new(RwLock::new(Config::new()))
    // let api = warp::path("api").and(warp::any().map(move || config.clone()));
    // let file_upload = warp::post()
    //     .and(api)
    //     .and(warp::path("file"))
    //     .and(warp::path::end())
    //     .then(file::upload);

    // let routes = warp::any().and(file_upload);

    // warp::serve(routes);
}
