use rocket::{get, routes};

#[get("/")]
fn index() -> &'static str {
	"Henlo, Wrold?"
}

#[get("/yeet/<name>")]
fn yeet(name: &str) -> String {
	format!("{} was yeeted.", name)
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
	let _rocket = rocket::build()
		.mount("/", routes![index, yeet])
		.launch()
		.await?;
	Ok(())
}
