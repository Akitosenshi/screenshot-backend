use magic::MagicError;
use rocket::{post, data::{Data, Limits, ToByteUnit}, tokio::io::AsyncWriteExt};
use uuid::Uuid;
use rocket::tokio::fs::File;

#[post("/file", data = "<data>")]
pub async fn upload(data: Data<'_>, limits: &Limits) -> std::io::Result<String> {
	let name = Uuid::new_v4().to_string().replace('-', "");

	let mut buffer: Vec<u8> = Vec::with_capacity(limits.get("file").unwrap_or(4.mebibytes()).as_u64() as usize);
	data.open(limits.get("file").unwrap_or(4.mebibytes())).stream_to(&mut buffer).await?;

	let ext = get_ext(&buffer).await.unwrap_or("".to_string());
	let path = format!("{}/{}.{}", "/tmp/screenshot", name, ext);

	File::create(path).await?.write_all(&buffer).await?;

	Ok(format!("http://localhost:8000/files/{name}.{ext}"))
}

async fn get_ext(buffer: &Vec<u8>) -> Result<String, MagicError> {
	let cookie = magic::Cookie::open(magic::CookieFlags::EXTENSION | magic::CookieFlags::ERROR)?;
	cookie.load::<&str>(&[])?;
	let tmp = cookie.buffer(&buffer)?;
	let ext = tmp.split('/').filter(|ext| ext.len() <= 3).next().unwrap_or("");
	Ok(ext.to_string())
}
