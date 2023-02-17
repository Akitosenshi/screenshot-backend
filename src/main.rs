pub mod api;
pub mod frontend;

use warp::Filter;

use crate::api::file;


#[tokio::main]
async fn main() {
	let ctx = "cum";
	let api = warp::path("api").and(warp::any().map(move || ctx.clone()));
	let file_upload = warp::post()
		.and(api)
		.and(warp::path("file"))
		.and(warp::path::end())
		.then(file::upload);

	let routes = warp::any().and(
		file_upload
	);

	warp::serve(routes).run(([127, 0, 0, 1], 8080)).await;
}
