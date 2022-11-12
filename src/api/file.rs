use magic::MagicError;
use rocket::{post, data::{Data, Limits, ToByteUnit}, tokio::io::AsyncWriteExt, serde::json::Json};
use rocket::tokio::fs::File;
use serde::Serialize;
use sha256::digest_bytes;

#[derive(Serialize)]
pub struct UploadResponse {
	status: String,
	url: String,
	error: String,
}

#[post("/file", data = "<data>")]
pub async fn upload(data: Data<'_>, limits: &Limits) -> Json<UploadResponse> {
	let mut buffer: Vec<u8> = Vec::with_capacity(limits.get("file").unwrap_or(4.mebibytes()).as_u64() as usize);
	match data.open(limits.get("file").unwrap_or(4.mebibytes())).stream_to(&mut buffer).await {
		Ok(_) => {},
		Err(_) => {
			return Json(UploadResponse {
				status: String::from("error"),
				url: String::from(""),
				error: String::from("error receiving data")
			});
		}
	}

	let name = digest_bytes(&buffer);
	let ext = get_ext(&buffer).await.unwrap_or(String::from(""));
	//TODO make path configurable
	let path = format!("{}/{}.{}", "/tmp/screenshot", name, ext);

	match File::create(path).await {
		Ok(mut file) => {
			match file.write_all(&buffer).await {
				Ok(_) => {},
				Err(_) => {
					return Json(UploadResponse {
						status: String::from("error"),
						url: String::from(""),
						error: String::from("error while saving file")
					});
				}
			}
		},
		Err(_) => {
			return Json(UploadResponse {
				status: String::from("error"),
				url: String::from(""),
				error: String::from("error while saving file")
			});
		}
	}

	Json(UploadResponse {
		status: String::from("ok"),
		url: String::from(format!("http://localhost:8000/files/{name}.{ext}")),
		error: String::from("")
	})
}

async fn get_ext(buffer: &Vec<u8>) -> Result<String, MagicError> {
	let cookie = magic::Cookie::open(magic::CookieFlags::EXTENSION | magic::CookieFlags::ERROR)?;
	cookie.load::<&str>(&[])?;
	let tmp = cookie.buffer(&buffer)?;
	let ext = tmp.split('/').filter(|ext| ext.len() <= 3).next().unwrap_or("");
	Ok(String::from(ext))
}
