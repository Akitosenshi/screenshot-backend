use magic::MagicError;
use rocket::{post, Data, data::{Limits, ToByteUnit}};
use uuid::Uuid;

#[post("/file", data = "<data>")]
pub async fn upload(mut data: Data<'_>, limits: &Limits) -> std::io::Result<String> {
	let name = Uuid::new_v4().to_string().replace('-', "");
	let buf = data.peek(512).await;
	let ext = if let Some(res) = infer::get(&buf) {
		res.extension()
	} else {
		if let Ok(res) = get_type(buf) {
			res
		} else {
			""
		}
	};
	let path = format!("/tmp/screenshot/{name}.{ext}");
	data.open(limits.get("file").unwrap_or(1.mebibytes())).into_file(path).await?;

	Ok(format!("http://localhost:8000/files/{name}.{ext}"))
}

fn get_type(buf: &[u8]) -> Result<&str, MagicError> {
	let cookie = magic::Cookie::open(magic::CookieFlags::EXTENSION | magic::CookieFlags::ERROR)?;
	cookie.load::<&str>(&[])?;
	let tmp = cookie.buffer(&buf)?;
	let exts: Vec<&str> = tmp.split('/').filter(|ext| ext.len() <= 3).collect();
	if let Some(res) = exts.first() {
		Ok(res)
	} else {
		Ok("")
	}
}
