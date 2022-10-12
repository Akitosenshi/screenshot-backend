pub mod api;
pub mod frontend;

use rocket::{get, routes};

#[get("/")]
fn index() -> &'static str {
	"Henlo, Wrold?"
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
	let _rocket = rocket::build()
		.mount("/", routes![index])
		.mount("/api", routes![api::file::upload])
		.mount("/", routes![frontend::file::serve])
		.launch()
		.await?;
	Ok(())
}
