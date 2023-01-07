pub mod api;
pub mod frontend;

use warp::{Filter, path};

use crate::api::file;


#[tokio::main]
async fn main() {
	let config = "no config for u lol";
	let api = warp::path("api").and(warp::any().map(move || config.clone()));
	let file_upload = warp::get()
		.and(api)
		.and(path!("file" / ..))
		.and(
			warp::path::param::<String>()
			.map(Some)
			.or_else(|_| async move {
				Ok::<(Option<String>,), std::convert::Infallible>((None,))
		}))
		.and(warp::path::end())
		.then(|conf, name: Option<String>| async move { format!("config: {}\n name: {:?}", conf, name) });

	let routes = warp::get().and(
		file_upload
	);

	warp::serve(routes).run(([127, 0, 0, 1], 8080)).await;
}
