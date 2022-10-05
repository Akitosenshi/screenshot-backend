pub mod api;
pub mod frontend;

use rocket::{get, routes};
use tera::Tera;
use lazy_static::lazy_static;


lazy_static! {
	pub static ref TEMPLATES: Tera = {
		let tera = match Tera::new("templates/**/*.html") {
			Ok(t) => t,
			Err(e) => {
				println!("Parsing error(s): {}", e);
				::std::process::exit(1);
			}
		};
		tera
	};
}

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
