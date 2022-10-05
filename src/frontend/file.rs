use rocket::{get, fs::NamedFile};

#[get("/files/<name>")]
pub async fn serve(name: String) -> Option<NamedFile> {
	NamedFile::open(format!("/tmp/screenshot/{name}")).await.ok()
}

