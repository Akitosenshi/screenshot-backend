#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> &'static str {
	"Henlo, Wrold?"
}

#[launch]
fn rocket() -> _ {
	rocket::build().mount("/", routes![index])
}
